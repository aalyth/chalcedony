use crate::lexer::tokens::{Token, TokenKind};
use crate::errors::span::{Span, pos::Position};

pub struct InvalidToken;
impl InvalidToken {
    pub fn msg(token: &Token, span: &Span, expected: TokenKind) {
        eprintln!("Error: invalid token (expected: '{:#?}', received: '{:#?}'):", expected, token.get_kind());
        span.context_print(token.start(), token.end());
    }
}

pub struct ExpectedToken;
impl ExpectedToken {
    pub fn msg(pos: &Position, span: &Span, expected: TokenKind) {
        eprintln!("Error: expected a token of type {:#?}.", expected);
        span.context_print(pos, pos);
    }
}

pub struct UnexpectedToken;
impl UnexpectedToken {
    pub fn msg(token: &Token, span: &Span) {
        eprintln!("Error: unexpected token ({:#?}):", token.get_kind());
        span.context_print(token.start(), token.end());
    }
}
