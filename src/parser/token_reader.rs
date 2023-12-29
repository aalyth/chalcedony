use crate::error::{ChalError, InternalError, ParserError, Position, Span};
use crate::lexer::{Token, TokenKind, Type};

use std::collections::VecDeque;
use std::rc::Rc;

pub struct TokenReader {
    src: VecDeque<Token>,
    start: Position,
    end: Position,
    span: Rc<Span>,
}

impl TokenReader {
    pub fn new(src: VecDeque<Token>, span: Rc<Span>) -> Self {
        let mut start = Position::new(0, 0);
        let mut end = Position::new(0, 0);

        /* check if there is at least 1 token in the source
         * and take the first token's end position */
        if !src.is_empty() {
            let front = src.front().unwrap();
            start = front.start();
            end = front.end();
        }

        TokenReader {
            start,
            end,
            src,
            span,
        }
    }

    pub fn span(&self) -> Rc<Span> {
        self.span.clone()
    }

    pub fn start(&self) -> Position {
        self.start
    }

    pub fn end(&self) -> Position {
        self.end
    }

    pub fn advance(&mut self) -> Option<Token> {
        let Some(res) = self.src.pop_front() else {
            return None;
        };

        self.start = res.start();
        self.end = res.end();
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

        *peek.kind() == exp
    }

    pub fn expect_ident(&mut self) -> Result<String, ChalError> {
        let exp = self.expect(TokenKind::Identifier(String::new()))?;
        match exp.kind() {
            TokenKind::Identifier(res) => Ok(res.clone()),
            _ => Err(
                InternalError::new("TokenReader::expect_ident(): invalid expect() value").into(),
            ),
        }
    }

    pub fn expect_type(&mut self) -> Result<Type, ChalError> {
        let exp = self.expect(TokenKind::Type(Type::Any))?;
        match exp.kind() {
            TokenKind::Type(res) => Ok(res.clone()),
            _ => {
                Err(InternalError::new("TokenReader::expect_type(): invalid expect() value").into())
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
        while !self.src.is_empty() && !cond(self.src.front().unwrap().kind()) {
            result.push_back(self.advance().unwrap());
        }
        Ok(result)
    }

    fn expect_inner(
        &mut self,
        exp: TokenKind,
        cond: fn(&TokenKind, &TokenKind) -> bool,
    ) -> Result<Token, ChalError> {
        let Some(token) = self.peek() else {
            return Err(
                ParserError::expected_token(exp, self.end, self.end, self.span.clone()).into(),
            );
        };

        if cond(token.kind(), &exp) {
            return Ok(self.advance().unwrap());
        }

        let mut start = token.start();
        let mut end = token.end();
        if token.kind() == &TokenKind::Newline {
            start = self.end();
            end = self.end();
        }
        Err(
            ParserError::invalid_token(exp, token.kind().clone(), start, end, self.span.clone())
                .into(),
        )
    }
}
