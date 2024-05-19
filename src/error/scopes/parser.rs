use crate::error::span::Span;
use crate::lexer::TokenKind;

use super::display_err;

/// The error types, which could be encountered during the transforming the
/// lexed stream of tokens into the Abstract Syntax Tree. For each error's
/// meaning refer to implementation of `std::fmt::Display` for `ParserError`.
pub enum ParserErrorKind {
    /// `<exp-type>`, `<recv-type>`
    InvalidToken(TokenKind, TokenKind),
    /// `<token-type>`
    ExpectedToken(TokenKind),
    /// `<token-type>`
    UnexpectedToken(TokenKind),
    InvalidAssignmentOperator,
    RepeatedExprTerminal,
    RepeatedExprOperator,
    InvalidStatement,
    InvalidExprEnd,
    EmptyExpr,
    UntypedList,
    MissingCatchBlock,
}

pub struct ParserError {
    kind: ParserErrorKind,
    span: Span,
}

impl ParserError {
    pub fn new(kind: ParserErrorKind, span: Span) -> Self {
        ParserError { kind, span }
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
                display_err(&self.span, f, msg)
            }

            ParserErrorKind::ExpectedToken(exp) => {
                let msg = &format!("expected a token of type '{:?}'", exp);
                display_err(&self.span, f, msg)
            }

            ParserErrorKind::UnexpectedToken(kind) => {
                let msg = &format!("unexpected token ('{:?}')", kind);
                display_err(&self.span, f, msg)
            }

            ParserErrorKind::InvalidAssignmentOperator => {
                display_err(&self.span, f, "invalid assignment operator")
            }

            ParserErrorKind::RepeatedExprTerminal => {
                display_err(&self.span, f, "repeated expression terminal")
            }

            ParserErrorKind::RepeatedExprOperator => {
                display_err(&self.span, f, "repeated expression operator")
            }

            ParserErrorKind::InvalidStatement => display_err(&self.span, f, "invalid statement"),
            ParserErrorKind::InvalidExprEnd => {
                display_err(&self.span, f, "expressions must end with a terminal")
            }

            ParserErrorKind::EmptyExpr => display_err(&self.span, f, "expected an expression"),
            ParserErrorKind::UntypedList => {
                display_err(&self.span, f, "expected a type to the list")
            }

            ParserErrorKind::MissingCatchBlock => display_err(
                &self.span,
                f,
                "`try` blocks must be followed by `catch` blocks",
            ),
        }
    }
}
