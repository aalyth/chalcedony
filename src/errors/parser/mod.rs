use crate::lexer::tokens::{Token, TokenKind};
use crate::errors::span::{Span, pos::Position};
use crate::errors::format::output::Output;

pub struct InvalidToken;
impl InvalidToken {
    pub fn msg(token: &Token, span: &Span, expected: TokenKind) {
        let message = format!("invalid token (expected: '{:#?}', received: '{:#?}'):", expected, token.get_kind());
        Output::err(&message);
        span.context_print(token.start(), token.end());
    }
}

pub struct ExpectedToken;
impl ExpectedToken {
    pub fn msg(pos: &Position, span: &Span, expected: TokenKind) {
        let message = format!("expected  a token of type {:#?}:", expected);
        Output::err(&message);
        span.context_print(pos, pos);
    }
}

pub struct UnexpectedToken;
impl UnexpectedToken {
    pub fn msg(token: &Token, span: &Span) {
        let message = format!("unexpected token ({:#?}):", token.get_kind());
        Output::err(&message);
        span.context_print(token.start(), token.end());
    }
}
