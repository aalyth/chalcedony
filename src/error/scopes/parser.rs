use crate::error::span::Span;
use crate::lexer::TokenKind;

use super::display_err;

enum ParserErrorKind {
    InvalidToken(TokenKind, TokenKind),
    ExpectedToken(TokenKind),
    UnexpectedToken(TokenKind),
    InvalidAssignmentOperator,
    RepeatedExprTerminal,
    RepeatedExprOperator,
    InvalidStatement,
    InvalidExprEnd,
    EmptyExpr,
    UntypedList,
}

pub struct ParserError {
    kind: ParserErrorKind,
    span: Span,
}

impl ParserError {
    fn new(kind: ParserErrorKind, span: Span) -> Self {
        ParserError { kind, span }
    }

    pub fn invalid_token(expected: TokenKind, received: TokenKind, span: Span) -> Self {
        ParserError::new(ParserErrorKind::InvalidToken(expected, received), span)
    }

    pub fn expected_token(expected: TokenKind, span: Span) -> Self {
        ParserError::new(ParserErrorKind::ExpectedToken(expected), span)
    }

    pub fn unexpected_token(kind: TokenKind, span: Span) -> Self {
        ParserError::new(ParserErrorKind::UnexpectedToken(kind), span)
    }

    pub fn invalid_assignment_operator(span: Span) -> Self {
        ParserError::new(ParserErrorKind::InvalidAssignmentOperator, span)
    }

    pub fn repeated_expr_terminal(span: Span) -> Self {
        ParserError::new(ParserErrorKind::RepeatedExprTerminal, span)
    }

    pub fn repeated_expr_operator(span: Span) -> Self {
        ParserError::new(ParserErrorKind::RepeatedExprOperator, span)
    }

    pub fn invalid_statement(span: Span) -> Self {
        ParserError::new(ParserErrorKind::InvalidStatement, span)
    }

    pub fn invalid_expr_end(span: Span) -> Self {
        ParserError::new(ParserErrorKind::InvalidExprEnd, span)
    }

    pub fn empty_expr(span: Span) -> Self {
        ParserError::new(ParserErrorKind::EmptyExpr, span)
    }

    pub fn untyped_list(span: Span) -> Self {
        ParserError::new(ParserErrorKind::UntypedList, span)
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
        }
    }
}
