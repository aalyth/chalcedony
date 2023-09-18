use crate::error::span::{pos::Position, Span};
use crate::error::format::err;
use crate::lexer::tokens::TokenKind;

use std::rc::Rc;

// the enum is the possible errorous token kinds
#[derive (PartialEq, Debug, Clone)]
enum LexerErrorKind {
    InvalidIdentifier,
    InvalidIndentation,
    UnclosedString,
    UnclosedComment,
    UnclosedDelimiter(String),
    UnexpectedClosingDelimiter(String),
    MissmatchingDelimiters(String, String),
    InvalidGlobalStatement(TokenKind),
}

pub struct LexerError {
    kind:  LexerErrorKind,
    start: Position,
    end:   Position,
    span:  Rc<Span>,
}

impl LexerError {
    fn new(
        kind:   LexerErrorKind, 
        start:  Position,
        end:    Position,
        span:   Rc<Span>,
    ) -> Self {
        LexerError {
            kind,
            start,
            end,
            span,
        }
    }

    pub fn invalid_identifier(start: Position, end: Position, span: Rc<Span>) -> Self {
        LexerError::new(LexerErrorKind::InvalidIdentifier, start, end, span)
    }

    pub fn unclosed_string(start: Position, end: Position, span: Rc<Span>) -> Self {
        LexerError::new(LexerErrorKind::UnclosedString, start, end, span)
    }

    pub fn unclosed_comment(start: Position, end: Position, span: Rc<Span>) -> Self {
        LexerError::new(LexerErrorKind::UnclosedComment, start, end, span)
    }

    pub fn invalid_indentation(start: Position, end: Position, span: Rc<Span>) -> Self {
        LexerError::new(LexerErrorKind::InvalidIndentation, start, end, span)
    }

    pub fn unclosed_delimiter(
        del:   &str,
        start: Position, 
        end:   Position, 
        span:  Rc<Span>,
    ) -> Self {
        let del = del.to_string();
        LexerError::new(LexerErrorKind::UnclosedDelimiter(del), start, end, span)
    }

    pub fn unexpected_closing_delimiter(
        del:   &str,
        start: Position, 
        end:   Position, 
        span:  Rc<Span>,
    ) -> Self {
        let del = del.to_string();
        LexerError::new(LexerErrorKind::UnexpectedClosingDelimiter(del), start, end, span)
    }

    pub fn missmatching_delimiters(
        open_del:  &str,
        close_del: &str,
        start:     Position, 
        end:       Position, 
        span:      Rc<Span>,
    ) -> Self {
        let open_del  = open_del.to_string();
        let close_del = close_del.to_string();
        LexerError::new(LexerErrorKind::MissmatchingDelimiters(open_del, close_del), start, end, span)
    }

    pub fn invalid_global_statement(
        token_kind: TokenKind, 
        start:      Position, 
        end:        Position, 
        span:       Rc<Span>
    ) -> Self {
        LexerError::new(LexerErrorKind::InvalidGlobalStatement(token_kind), start, end, span)
    }

    fn display_err(&self, f: &mut std::fmt::Formatter, msg: &str) -> std::fmt::Result {
        write!(f, "{}:\n{}", err(msg), self.span.context(&self.start, &self.end))
    }
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            LexerErrorKind::InvalidIdentifier  => self.display_err(f, "invalid identifier"),
            LexerErrorKind::UnclosedString     => self.display_err(f, "unclosed string"),
            LexerErrorKind::UnclosedComment    => self.display_err(f, "unclosed multiline comment"),
            LexerErrorKind::InvalidIndentation => self.display_err(f, "invalid indendation"),

            LexerErrorKind::UnclosedDelimiter(del) => {
                let msg = &format!("unclosed delimiter ('{}')", del);
                self.display_err(f, msg)
            },

            LexerErrorKind::UnexpectedClosingDelimiter(del) => { 
                let msg = &format!("unexpected closing delimiter ('{}')", del);
                self.display_err(f, msg)
            },

            LexerErrorKind::MissmatchingDelimiters(open_del, close_del) => {
                let msg = &format!("missmatching delimiters ('{}' and '{}')", open_del, close_del);
                self.display_err(f, msg)
            },

            LexerErrorKind::InvalidGlobalStatement(token_kind) => {
                let msg = &format!("invalid global statement ({:?})", token_kind);
                self.display_err(f, msg)
            }
        }
    }
}

