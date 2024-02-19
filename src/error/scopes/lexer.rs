use crate::error::err;
use crate::error::span::Span;
use crate::lexer::TokenKind;

use super::display_err;

/* the possible errorous token kinds */
enum LexerErrorKind {
    InvalidIndentation,
    UnclosedString,
    UnclosedDelimiter(String),
    UnexpectedClosingDelimiter(String),
    MismatchingDelimiters(String, String),
    InvalidGlobalStatement(TokenKind),
    InvalidChar(char),
}

pub struct LexerError {
    kind: LexerErrorKind,
    span: Span,
}

impl LexerError {
    fn new(kind: LexerErrorKind, span: Span) -> Self {
        LexerError { kind, span }
    }

    pub fn unclosed_string(span: Span) -> Self {
        LexerError::new(LexerErrorKind::UnclosedString, span)
    }

    pub fn invalid_indentation(span: Span) -> Self {
        LexerError::new(LexerErrorKind::InvalidIndentation, span)
    }

    pub fn unclosed_delimiter(del: &str, span: Span) -> Self {
        let del = del.to_string();
        LexerError::new(LexerErrorKind::UnclosedDelimiter(del), span)
    }

    pub fn unexpected_closing_delimiter(del: &str, span: Span) -> Self {
        let del = del.to_string();
        LexerError::new(LexerErrorKind::UnexpectedClosingDelimiter(del), span)
    }

    pub fn mismatching_delimiters(open_del: &str, close_del: &str, span: Span) -> Self {
        let open_del = open_del.to_string();
        let close_del = close_del.to_string();
        LexerError::new(
            LexerErrorKind::MismatchingDelimiters(open_del, close_del),
            span,
        )
    }

    pub fn invalid_global_statement(token_kind: TokenKind, span: Span) -> Self {
        LexerError::new(LexerErrorKind::InvalidGlobalStatement(token_kind), span)
    }

    pub fn invalid_char(chr: char, span: Span) -> Self {
        LexerError::new(LexerErrorKind::InvalidChar(chr), span)
    }
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            LexerErrorKind::InvalidIndentation => display_err(&self.span, f, "invalid indendation"),
            LexerErrorKind::UnclosedString => display_err(&self.span, f, "unclosed string"),

            LexerErrorKind::UnclosedDelimiter(del) => {
                let msg = &format!("unclosed delimiter ('{}')", del);
                display_err(&self.span, f, msg)
            }

            LexerErrorKind::UnexpectedClosingDelimiter(del) => {
                let msg = &format!("unexpected closing delimiter ('{}')", del);
                display_err(&self.span, f, msg)
            }

            LexerErrorKind::MismatchingDelimiters(open_del, close_del) => {
                let msg = &format!(
                    "missmatching delimiters ('{}' and '{}')",
                    open_del, close_del
                );

                let open_del_span =
                    Span::new(self.span.start, self.span.start, self.span.spanner.clone());
                let close_del_span =
                    Span::new(self.span.end, self.span.end, self.span.spanner.clone());

                let open_ctx = open_del_span.context();
                let end_ctx = close_del_span.context();
                write!(f, "{}:\n{}{}\n", err(msg), open_ctx, end_ctx)
            }

            LexerErrorKind::InvalidGlobalStatement(token_kind) => {
                let msg = &format!("invalid global statement ({:?})", token_kind);
                display_err(&self.span, f, msg)
            }

            LexerErrorKind::InvalidChar(chr) => {
                let msg = &format!("could not lex the given char ({:?})", chr);
                display_err(&self.span, f, msg)
            }
        }
    }
}
