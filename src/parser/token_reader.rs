use crate::error::span::{Span, Spanning};
use crate::error::{ChalError, ParserError, ParserErrorKind};
use crate::lexer::{Delimiter, Special, Token, TokenKind};

use crate::common::Type;

use std::collections::VecDeque;
use std::rc::Rc;

/// The abstraction used to go over a stream of Tokens.
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

    /// Returns the token to the front of the reader.
    pub fn push_front(&mut self, token: Token) {
        self.src.push_front(token)
    }

    /// Advances the next token if it is of type `exp` and returns a
    /// `ParserError` if the token does not match the expected type. This
    /// function uses `soft` checking, i.e. only the outer variant of the token
    /// kind is checked.
    ///
    /// For example: using `reader.expect(TokenKind::Identifier(""))` where the
    /// next token is of type `TokenKind::Identifier("hello")` will still match
    /// since both are of type `TokenKind::Identifier(_)`.
    ///
    /// For strict checking refer to the function `TokenReader::expect_exact()`.
    pub fn expect(&mut self, exp: TokenKind) -> Result<Token, ChalError> {
        /* std::mem:discriminant() checks only the outer enum variant */
        fn condition(current: &TokenKind, exp: &TokenKind) -> bool {
            std::mem::discriminant(current) == std::mem::discriminant(exp)
        }

        self.expect_inner(exp, condition)
    }

    /// Advances the reader if the next token strictly matches the expected type.
    /// Returns a `ParserError` if the type is not valid. For a more liberal
    /// type expectations, refer to `TokenReader::expect()`.
    pub fn expect_exact(&mut self, exp: TokenKind) -> Result<Token, ChalError> {
        self.expect_inner(exp, |current, exp| current == exp)
    }

    /// Returns whether the next token strictly matches the expected type.
    pub fn peek_is_exact(&self, exp: TokenKind) -> bool {
        let Some(peek) = self.peek() else {
            return false;
        };

        peek.kind == exp
    }

    pub fn peek_nth(&self, n: usize) -> Option<&Token> {
        self.src.get(n)
    }

    /// Advances the next token if it is of type `TokenKind::Str()` and returns
    /// it's value. Equivalent to the code:
    /// ```
    /// let token = reader.expect(TokenKind::Identifier(String::new()))?;
    /// let TokenKind::String(result) = token else {
    ///     unreachable!();
    /// };
    /// ```
    pub fn expect_ident(&mut self) -> Result<String, ChalError> {
        let exp = self.expect(TokenKind::Identifier(String::new()))?;
        match exp.kind {
            TokenKind::Identifier(res) => Ok(res),
            _ => panic!("TokenReader::expect_indent(): invalid expect() return value"),
        }
    }

    /// Advances the next token if it is of type `TokenKind::Type()` and returns
    /// it's value. Equivalent to the code:
    /// ```
    /// let token = reader.expect(TokenKind::Type(Type::Any))?;
    /// let TokenKind::Type(result) = token else {
    ///     unreachable!();
    /// };
    /// ```
    pub fn expect_type(&mut self) -> Result<Type, ChalError> {
        let Some(peek) = self.peek().cloned() else {
            return Err(ParserError::new(
                ParserErrorKind::ExpectedToken(TokenKind::Type(Type::Any)),
                self.current.clone(),
            )
            .into());
        };

        match peek.kind {
            /* the type begins with a `[`, so we expect a list */
            TokenKind::Delimiter(Delimiter::OpenBracket) => {
                // this both overrides the immutable self borrow and advances the
                // already checked opening delimiter

                // let peek = self.advance().unwrap();
                let start = self.current.start;
                let mut scope = self.advance_scope_raw(
                    TokenKind::Delimiter(Delimiter::OpenBracket),
                    TokenKind::Delimiter(Delimiter::CloseBracket),
                );

                /* remove the opening and closing brackets */
                scope.pop_front();
                scope.pop_back();

                if scope.is_empty() {
                    /* SAFETY: the lexer guarantees that at least there is a matching closing delim */
                    let closing_delim = scope.front().unwrap();
                    let span = Span::new(start, closing_delim.span.end, peek.span.spanner.clone());
                    return Err(ParserError::new(ParserErrorKind::UntypedList, span).into());
                }

                let mut inner_reader = TokenReader::new(scope, peek.span);

                Ok(Type::List(Box::new(inner_reader.expect_type()?)))
            }

            /* default type expectation */
            _ => match self.advance().unwrap().kind {
                TokenKind::Type(ty) => Ok(ty),
                TokenKind::Identifier(val) => Ok(Type::Custom(Box::new(val))),
                recv => Err(ParserError::new(
                    ParserErrorKind::InvalidToken(TokenKind::Type(Type::Any), recv),
                    self.current.clone(),
                )
                .into()),
            },
        }
    }

    /// Advances the tokens until the condition is met.
    pub fn advance_until(
        &mut self,
        cond: fn(&TokenKind) -> bool,
    ) -> Result<VecDeque<Token>, ChalError> {
        if self.is_empty() {
            panic!("TokenReader::advance_until(): advancing an empty reader")
        }

        let mut result = VecDeque::<Token>::new();
        while !self.src.is_empty() && !cond(&self.src.front().unwrap().kind) {
            result.push_back(self.advance().unwrap());
        }
        Ok(result)
    }

    pub fn advance_scope(&mut self, open_delim: TokenKind, close_delim: TokenKind) -> TokenReader {
        TokenReader::new(
            self.advance_scope_raw(open_delim, close_delim),
            self.current.clone(),
        )
    }

    pub fn advance_scope_raw(
        &mut self,
        open_delim: TokenKind,
        close_delim: TokenKind,
    ) -> VecDeque<Token> {
        let mut open_scopes = 0;
        let mut result = VecDeque::<Token>::new();

        while !self.is_empty() {
            result.push_back(self.advance().unwrap());

            let kind = &result.back().unwrap().kind;

            if kind == &open_delim {
                open_scopes += 1;
            }

            if kind == &close_delim {
                if open_scopes == 1 {
                    break;
                }
                open_scopes -= 1;
            }
        }

        result
    }

    // splits the remainder of the reader by `TokenKind::Special(Special::Comma)`
    // NOTE: elements are possible to be empty
    pub fn split_commas(mut self) -> Vec<VecDeque<Token>> {
        let mut result = Vec::<VecDeque<Token>>::new();
        let mut buffer = VecDeque::<Token>::new();
        let mut open_delims = 0;

        while let Some(token) = self.advance() {
            match token.kind {
                TokenKind::Delimiter(Delimiter::OpenPar)
                | TokenKind::Delimiter(Delimiter::OpenBrace)
                | TokenKind::Delimiter(Delimiter::OpenBracket) => {
                    open_delims += 1;
                    buffer.push_back(token);
                }
                TokenKind::Delimiter(Delimiter::ClosePar)
                | TokenKind::Delimiter(Delimiter::CloseBrace)
                | TokenKind::Delimiter(Delimiter::CloseBracket) => {
                    open_delims -= 1;
                    buffer.push_back(token);
                }
                TokenKind::Special(Special::Comma) if open_delims == 0 => {
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

    pub fn is_empty(&self) -> bool {
        self.src.is_empty()
    }

    // NOTE: expectations only consume tokens if the conditions is successful
    fn expect_inner(
        &mut self,
        exp: TokenKind,
        cond: fn(&TokenKind, &TokenKind) -> bool,
    ) -> Result<Token, ChalError> {
        let Some(token) = self.peek() else {
            return Err(ParserError::new(
                ParserErrorKind::ExpectedToken(exp),
                self.current.clone(),
            )
            .into());
        };

        if cond(&token.kind, &exp) {
            return Ok(self.advance().unwrap());
        }

        let mut span = token.span.clone();
        if token.kind == TokenKind::Newline {
            span = self.current.clone();
        }
        Err(ParserError::new(ParserErrorKind::InvalidToken(exp, token.kind.clone()), span).into())
    }
}

impl std::fmt::Debug for TokenReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.src)
    }
}
