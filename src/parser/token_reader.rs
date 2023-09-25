use crate::error::{Span, Position, ChalError, ParserError};
use crate::lexer::{Line, Token, TokenKind};

use std::collections::VecDeque;
use std::rc::Rc;

pub struct TokenReader {
    src:  VecDeque<Token>,
    pos:  Position,
    span: Rc<Span>,
}

impl TokenReader {
    pub fn new(src: VecDeque<Token>, span: &Rc<Span>) -> Self {
        let mut pos = Position::new(1, 1);

        /* check if there is at least 1 token in the source
         * and take the first token's end position */
        if !src.is_empty() {
            pos = *src.front().unwrap().end(); 
        }

        TokenReader {
            src,
            pos,
            span: Rc::clone(span),
        }
    }

    pub fn advance(&mut self) -> Option<Token> {
        let Some(res) = self.src.pop_front() else {
            return None;
        };

        self.pos = *res.end();
        Some(res)
    }

    pub fn peek(&self) -> Option<&Token> {
        self.src.front()
    }

    fn expect_inner(&mut self, exp: TokenKind, cond: fn (&TokenKind, &TokenKind) -> bool) -> Result<Token, ChalError>{
        let Some(token) = self.peek() else {
            return Err(
                ChalError::from(
                    ParserError::expected_token(exp, self.pos, self.pos, Rc::clone(&self.span))
                )
            );
        };

        if cond(token.kind(), &exp) {
            return Ok(self.advance().unwrap());
        } 

        Err(
            ChalError::from(
                ParserError::expected_token(exp, self.pos, self.pos, Rc::clone(&self.span))
            )
        )

    }

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
}
