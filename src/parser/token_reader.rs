use crate::error::span::{Span, Spanning};
use crate::error::{ChalError, InternalError, ParserError};
use crate::lexer::{Delimiter, Special, Token, TokenKind};

use crate::common::Type;

use std::collections::VecDeque;
use std::rc::Rc;

pub struct TokenReader {
    src: VecDeque<Token>,
    current: Span,
    spanner: Rc<dyn Spanning>,
}

impl TokenReader {
    pub fn new(src: VecDeque<Token>, current: Span) -> Self {
        let mut start = current.start;
        let mut end = current.end;

        if !src.is_empty() {
            let front = src.front().unwrap();
            start = front.span.start;
            end = front.span.end;
        }

        TokenReader {
            src,
            current: Span::new(start, end, current.spanner.clone()),
            spanner: current.spanner.clone(),
        }
    }

    pub fn current(&self) -> Span {
        self.current.clone()
    }

    pub fn spanner(&self) -> Rc<dyn Spanning> {
        self.spanner.clone()
    }

    pub fn advance(&mut self) -> Option<Token> {
        let Some(res) = self.src.pop_front() else {
            return None;
        };

        self.current.start = res.span.start;
        self.current.end = res.span.end;
        Some(res)
    }

    pub fn peek(&self) -> Option<&Token> {
        self.src.front()
    }

    /* NOTE: expectations only consume tokens if the conditions is successful */
    pub fn expect(&mut self, exp: TokenKind) -> Result<Token, ChalError> {
        /* std::mem:discriminant() makes it so we can check only the outer enum variant
         * for example:
         * TokenKind::Identifier('main') is equal to TokenKind::Identifier('')
         */
        fn condition(current: &TokenKind, exp: &TokenKind) -> bool {
            std::mem::discriminant(current) == std::mem::discriminant(exp)
        }

        self.expect_inner(exp, condition)
    }

    pub fn expect_exact(&mut self, exp: TokenKind) -> Result<Token, ChalError> {
        self.expect_inner(exp, |current, exp| current == exp)
    }

    /* returns weather the next token is of expected kind */
    pub fn peek_is_exact(&self, exp: TokenKind) -> bool {
        let Some(peek) = self.peek() else {
            return false;
        };

        peek.kind == exp
    }

    pub fn expect_ident(&mut self) -> Result<String, ChalError> {
        let exp = self.expect(TokenKind::Identifier(String::new()))?;
        match exp.kind {
            TokenKind::Identifier(res) => Ok(res),
            _ => Err(
                InternalError::new("TokenReader::expect_ident(): invalid expect() value").into(),
            ),
        }
    }

    pub fn expect_type(&mut self) -> Result<Type, ChalError> {
        let Some(peek) = self.peek() else {
            return Err(InternalError::new(
                "TokenReader::expect_type(): expecting from empty reader",
            )
            .into());
        };

        match peek.kind {
            /* the type begins with a `[`, so we expect a list */
            TokenKind::Delimiter(Delimiter::OpenBracket) => {
                // this both overrides the immutable self borrow and advances the
                // already checked opening delimiter
                let peek = self.advance().unwrap();
                let mut scope = self.advance_scope();

                if scope.len() < 2 {
                    /* SAFETY: the lexer guarantees that at least there is a matching closing delim */
                    let closing_delim = scope.front().unwrap();
                    let span = Span::new(
                        peek.span.start,
                        closing_delim.span.end,
                        peek.span.spanner.clone(),
                    );
                    return Err(ParserError::untyped_list(span).into());
                }

                /* remove the closing delimiter */
                scope.pop_back();
                let mut inner_reader = TokenReader::new(scope, peek.span);

                Ok(Type::List(Box::new(inner_reader.expect_type()?)))
            }

            /* default type expectation */
            _ => {
                let advanced = self.expect(TokenKind::Type(Type::Any))?;
                let TokenKind::Type(res) = advanced.kind else {
                    return Err(InternalError::new(
                        "TokenReader::expect_type(): invalid expect() value",
                    )
                    .into());
                };
                Ok(res)
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.src.is_empty()
    }

    pub fn advance_until(
        &mut self,
        cond: fn(&TokenKind) -> bool,
    ) -> Result<VecDeque<Token>, ChalError> {
        if self.is_empty() {
            return Err(InternalError::new(
                "TokenReader::advance_until(): advancing an empty reader",
            )
            .into());
        }

        let mut result = VecDeque::<Token>::new();
        while !self.src.is_empty() && !cond(&self.src.front().unwrap().kind) {
            result.push_back(self.advance().unwrap());
        }
        Ok(result)
    }

    // advances the reader until a full scope between two delimiters is met
    // NOTE: the opening delimiter must already be advanced before calling `advance_scope()`
    pub fn advance_scope(&mut self) -> VecDeque<Token> {
        let mut scoping = 1;
        let mut result = VecDeque::<Token>::new();

        while scoping > 0 {
            let Some(current) = self.advance() else { break };
            match current.kind {
                TokenKind::Delimiter(Delimiter::ClosePar)
                | TokenKind::Delimiter(Delimiter::CloseBrace)
                | TokenKind::Delimiter(Delimiter::CloseBracket) => scoping -= 1,

                TokenKind::Delimiter(Delimiter::OpenPar)
                | TokenKind::Delimiter(Delimiter::OpenBrace)
                | TokenKind::Delimiter(Delimiter::OpenBracket) => scoping += 1,
                _ => {}
            }

            result.push_back(current);
        }

        result
    }

    // splits the remainder of the reader by `TokenKind::Special(Special::Comma)`
    // NOTE: elements are possible to be empty
    pub fn split_commas(mut self) -> Vec<VecDeque<Token>> {
        let mut result = Vec::<VecDeque<Token>>::new();
        let mut buffer = VecDeque::<Token>::new();

        while let Some(token) = self.advance() {
            match token.kind {
                TokenKind::Special(Special::Comma) => {
                    result.push(buffer);
                    buffer = VecDeque::<Token>::new();
                    continue;
                }

                _ => buffer.push_back(token),
            }
        }

        result.push(buffer);
        result
    }

    fn expect_inner(
        &mut self,
        exp: TokenKind,
        cond: fn(&TokenKind, &TokenKind) -> bool,
    ) -> Result<Token, ChalError> {
        let Some(token) = self.peek() else {
            return Err(ParserError::expected_token(exp, self.current.clone()).into());
        };

        if cond(&token.kind, &exp) {
            return Ok(self.advance().unwrap());
        }

        let mut span = token.span.clone();
        if token.kind == TokenKind::Newline {
            span = self.current.clone();
        }
        Err(ParserError::invalid_token(exp, token.kind.clone(), span).into())
    }
}
