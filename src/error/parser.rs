use crate::error::format::err;
use crate::error::span::{pos::Position, Span};
use crate::lexer::tokens::TokenKind;

use std::rc::Rc;

enum ParserErrorKind {
    InvalidToken(TokenKind, TokenKind),
    ExpectedToken(TokenKind),
    UnexpectedToken(TokenKind),
    InvalidAssignmentOperator,
    RepeatedExprTerminal,
    RepeatedExprOperator,
    InvalidStatement,
}

pub struct ParserError {
    kind: ParserErrorKind,
    start: Position,
    end: Position,
    span: Rc<Span>,
}

impl ParserError {
    fn new(kind: ParserErrorKind, start: Position, end: Position, span: Rc<Span>) -> Self {
        ParserError {
            kind,
            start,
            end,
            span,
        }
    }

    pub fn invalid_token(
        expected: TokenKind,
        received: TokenKind,
        start: Position,
        end: Position,
        span: Rc<Span>,
    ) -> Self {
        ParserError::new(
            ParserErrorKind::InvalidToken(expected, received),
            start,
            end,
            span,
        )
    }

    pub fn expected_token(
        expected: TokenKind,
        start: Position,
        end: Position,
        span: Rc<Span>,
    ) -> Self {
        ParserError::new(ParserErrorKind::ExpectedToken(expected), start, end, span)
    }

    pub fn unexpected_token(
        kind: TokenKind,
        start: Position,
        end: Position,
        span: Rc<Span>,
    ) -> Self {
        ParserError::new(ParserErrorKind::UnexpectedToken(kind), start, end, span)
    }

    pub fn invalid_assignment_operator(start: Position, end: Position, span: Rc<Span>) -> Self {
        ParserError::new(ParserErrorKind::InvalidAssignmentOperator, start, end, span)
    }

    pub fn repeated_expr_terminal(start: Position, end: Position, span: Rc<Span>) -> Self {
        ParserError::new(ParserErrorKind::RepeatedExprTerminal, start, end, span)
    }

    pub fn repeated_expr_operator(start: Position, end: Position, span: Rc<Span>) -> Self {
        ParserError::new(ParserErrorKind::RepeatedExprOperator, start, end, span)
    }

    pub fn invalid_statement(start: Position, end: Position, span: Rc<Span>) -> Self {
        ParserError::new(ParserErrorKind::InvalidStatement, start, end, span)
    }

    fn display_err(&self, f: &mut std::fmt::Formatter, msg: &str) -> std::fmt::Result {
        write!(
            f,
            "{}:\n{}\n",
            err(msg),
            self.span.context(&self.start, &self.end)
        )
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            ParserErrorKind::InvalidToken(exp, recv) => {
                let msg = &format!(
                    "invalid token (expected: '{:?}', received: '{:?}')",
                    exp, recv
                );
                self.display_err(f, msg)
            }

            ParserErrorKind::ExpectedToken(exp) => {
                let msg = &format!("expected a token of type '{:?}'", exp);
                self.display_err(f, msg)
            }

            ParserErrorKind::UnexpectedToken(kind) => {
                let msg = &format!("unexpected token ('{:?}')", kind);
                self.display_err(f, msg)
            }

            ParserErrorKind::InvalidAssignmentOperator => {
                self.display_err(f, "invalid assignment operator")
            }

            ParserErrorKind::RepeatedExprTerminal => {
                self.display_err(f, "repeated expression terminal")
            }

            ParserErrorKind::RepeatedExprOperator => {
                self.display_err(f, "repeated expression operator")
            }

            ParserErrorKind::InvalidStatement => self.display_err(f, "invalid statement"),
        }
    }
}
