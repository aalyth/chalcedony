use crate::errors::span::{pos::Position, Span};
use crate::errors::format::output::Output;
use crate::lexer::tokens::TokenKind;

// the enum is the possible errorous token kinds
#[derive (PartialEq, Debug, Clone)]
pub enum LexerError {
    InvalidIdentifier,
    UnclosedString,
    UnclosedComment,
}

pub struct InvalidIdentifier;
impl InvalidIdentifier {
    pub fn msg(start: &Position, end: &Position, span: &Span) {
        Output::err("invalid identifier:");
        span.context_print(start, end);
    }
}

pub struct UnclosedString;
impl UnclosedString {
    pub fn msg(start: &Position, end: &Position, span: &Span) {
        Output::err("unclosed string:");
        span.context_print(start, end);
    }
}

pub struct UnclosedComment;
impl UnclosedComment {
    pub fn msg(start: &Position, end: &Position, span: &Span) {
        Output::err("unclosed multiline comment:");
        span.context_print(start, end);
    }
}

pub struct UnclosedDelimiter;
impl UnclosedDelimiter {
    pub fn msg(start: &Position, end: &Position, span: &Span, del: &str) {
        let message = format!("unclosed delimiter ('{}'):", del);
        Output::err(&message);
        span.context_print(start, end);
    }
}

pub struct UnexpectedClosingDelimiter;
impl UnexpectedClosingDelimiter {
    pub fn msg(start: &Position, end: &Position, span: &Span, del: &str) {
        let message = format!("unexpected closing delimiter ('{}'):", del);
        Output::err(&message);
        span.context_print(start, end);
    }
}

pub struct MissmatchingDelimiter;
impl MissmatchingDelimiter {
    pub fn msg(start: &Position, end: &Position, span: &Span, open_del: &str, close_del: &str) {
        let message = format!("missmatching delimiter ('{}' and '{}'):", open_del, close_del);
        Output::err(&message);
        span.context_print(start, end);
    }
}

pub struct InvalidGlobalStatement;
impl InvalidGlobalStatement {
    pub fn msg(start: &Position, end: &Position, span: &Span, token_kind: &TokenKind) {
        let message = format!("invalid global statement ({:?}):", token_kind);
        Output::err(&message);
        span.context_print(start, end);
    }
}

