use crate::lexer::tokens::TokenKind;
use crate::error::span::{Span, pos::Position};
use crate::error::format::err;

enum ParserErrorKind {
    InvalidToken (TokenKind, TokenKind),
    ExpectedToken (TokenKind),
    UnexpectedToken (TokenKind),
}

pub struct ParserError<'a> {
    kind: ParserErrorKind,
    start: &'a Position,
    end:   &'a Position,
    span:  &'a Span,
}

impl<'a> ParserError<'a> {
    fn new(
        kind:   ParserErrorKind,
        start: &Position,
        end:   &Position,
        span:  &Span,
    ) -> Self {
        ParserError {
            kind,
            start,
            end,
            span
        }
    }

    pub fn invalid_token(
        expected:  TokenKind,
        received:  TokenKind,
        start:    &Position, 
        end:      &Position, 
        span:     &Span,
    ) -> Self {
        ParserError::new(ParserErrorKind::InvalidToken(expected, received), start, end, span)
    }

    pub fn expected_token(
        expected:  TokenKind,
        start:    &Position, 
        end:      &Position, 
        span:     &Span,
    ) -> Self {
        ParserError::new(ParserErrorKind::ExpectedToken(expected), start, end, span)
    }

    pub fn unexpected_token(
        kind:   TokenKind,
        start: &Position, 
        end:   &Position, 
        span:  &Span,
    ) -> Self {
        ParserError::new(ParserErrorKind::UnexpectedToken(kind), start, end, span)
    }

    fn display_err(&self, f: &mut std::fmt::Formatter, msg: &str) -> std::fmt::Result {
        write!(f, "{}:\n{}", err(msg), self.span.context(self.start, self.end))
    }

}

impl std::fmt::Display for ParserError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.kind {
            ParserErrorKind::InvalidToken(exp, recv) => { 
                let msg = &format!("invalid token (expected: '{:?}', received: '{:?}')", exp, recv);
                self.display_err(f, msg)
            },

            ParserErrorKind::ExpectedToken(exp) => { 
                let msg = &format!("expected a token of type '{:?}'", exp);
                self.display_err(f, msg)
            },

            ParserErrorKind::UnexpectedToken(kind) => { 
                let msg = &format!("unexpected token ('{:?}')", kind);
                self.display_err(f, msg)
            },
        }
    }
}

