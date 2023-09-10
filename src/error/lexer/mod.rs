use crate::error::span::{pos::Position, Span};
use crate::error::format::err;
use crate::lexer::tokens::TokenKind;

// the enum is the possible errorous token kinds
#[derive (PartialEq, Debug, Clone)]
enum LexerErrorKind<'a> {
    InvalidIdentifier,
    UnclosedString,
    UnclosedComment,
    UnclosedDelimiter(&'a str),
    UnexpectedClosingDelimiter(&'a str),
    MissmatchingDelimiters(&'a str, &'a str),
    InvalidGlobalStatement(&'a TokenKind),
}

pub struct LexerError<'a> {
    kind:  LexerErrorKind<'a>,
    start: &'a Position,
    end:   &'a Position,
    span:  &'a Span,
}

impl<'a> LexerError<'a> {
    fn new(
        kind:   LexerErrorKind, 
        start: &Position,
        end:   &Position,
        span:  &Span
    ) -> Self {
        LexerError {
            kind,
            start,
            end,
            span,
        }
    }

    pub fn invalid_identifier(start: &Position, end: &Position, span: &Span) -> Self {
        LexerError::new(LexerErrorKind::InvalidIdentifier, start, end, span)
    }

    pub fn unclosed_string(start: &Position, end: &Position, span: &Span) -> Self {
        LexerError::new(LexerErrorKind::UnclosedString, start, end, span)
    }

    pub fn unclosed_comment(start: &Position, end: &Position, span: &Span) -> Self {
        LexerError::new(LexerErrorKind::UnclosedComment, start, end, span)
    }

    pub fn unclosed_delimiter(
        del:   &str,
        start: &Position, 
        end:   &Position, 
        span:  &Span
    ) -> Self {
        LexerError::new(LexerErrorKind::UnclosedDelimiter(del), start, end, span)
    }

    pub fn unexpected_closing_delimiter(
        del:   &str,
        start: &Position, 
        end:   &Position, 
        span:  &Span
    ) -> Self {
        LexerError::new(LexerErrorKind::UnexpectedClosingDelimiter(del), start, end, span)
    }

    pub fn missmatching_delimiters(
        open_del:  &str,
        close_del: &str,
        start:     &Position, 
        end:       &Position, 
        span:      &Span
    ) -> Self {
        LexerError::new(LexerErrorKind::MissmatchingDelimiters(open_del, close_del), start, end, span)
    }

    pub fn invalid_global_statement(
        token_kind: &TokenKind, 
        start:      &Position, 
        end:        &Position, 
        span:       &Span
    ) -> Self {
        LexerError::new(LexerErrorKind::InvalidIdentifier, start, end, span)
    }

    fn display_err(&self, f: &mut std::fmt::Formatter, msg: &str) -> std::fmt::Result {
        write!(f, "{}:\n{}", err(msg), self.span.context(self.start, self.end))
    }
}

impl std::fmt::Display for LexerError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.kind {
            LexerErrorKind::InvalidIdentifier => self.display_err(f, "invalid identifier"),
            LexerErrorKind::UnclosedString    => self.display_err(f, "unclosed string"),
            LexerErrorKind::UnclosedComment   => self.display_err(f, "unclosed multiline comment"),

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

