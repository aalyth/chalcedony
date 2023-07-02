use crate::errors::span::{pos::Position, Span};

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
        eprintln!("Error: invalid identifier:");
        span.context_print(start, end);
    }
}

pub struct UnclosedString;
impl UnclosedString {
    pub fn msg(start: &Position, end: &Position, span: &Span) {
        eprintln!("Error: unclosed string:");
        span.context_print(start, end);
    }
}

pub struct UnclosedComment;
impl UnclosedComment {
    pub fn msg(start: &Position, end: &Position, span: &Span) {
        eprintln!("Error: unclosed multiline comment:");
        span.context_print(start, end);
    }
}

pub struct UnclosedDelimiter;
impl UnclosedDelimiter {
    pub fn msg(start: &Position, end: &Position, span: &Span, del: &str) {
        eprintln!("Error: unclosed delimiter ('{}'):", del);
        span.context_print(start, end);
    }
}

pub struct UnexpectedClosingDelimiter;
impl UnexpectedClosingDelimiter {
    pub fn msg(start: &Position, end: &Position, span: &Span, del: &str) {
        eprintln!("Error: unexpected closing delimiter ('{}'):", del);
        span.context_print(start, end);
    }
}

pub struct MissmatchingDelimiter;
impl MissmatchingDelimiter {
    pub fn msg(start: &Position, end: &Position, span: &Span, open_del: &str, close_del: &str) {
        eprintln!("Error: missmatching delimiter ('{}' and '{}'):", open_del, close_del);
        span.context_print(start, end);
    }
}

