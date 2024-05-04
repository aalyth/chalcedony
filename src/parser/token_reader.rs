use crate::error::span::{Span, Spanning};
use crate::error::{ChalError, ParserError, ParserErrorKind};
use crate::lexer::{Token, TokenKind};

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
        let exp = self.expect(TokenKind::Type(Type::Any))?;
        match exp.kind {
            TokenKind::Type(res) => Ok(res),
            _ => panic!("TokenReader::expect_type(): invalid expect() return value"),
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
