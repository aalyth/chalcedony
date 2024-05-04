use crate::error::err;
use crate::error::span::Span;
use crate::lexer::TokenKind;

use super::display_err;

/// The errors types, which can be encountered transforming the source code into
/// a series of tokens. For each error's meaning refer to implementation of
/// `std::fmt::Display` for `LexerError`.
pub enum LexerErrorKind {
    InvalidIndentation,
    UnclosedString,
    /// `<delim-literal>`
    UnclosedDelimiter(String),
    /// `<delim-literal>`
    UnexpectedClosingDelimiter(String),
    /// `<open-delim-literal>`, `<close-delim-literal>`
    MismatchingDelimiters(String, String),
    /// `<token-type>`
    InvalidGlobalStatement(TokenKind),
    InvalidChar(char),
}

pub struct LexerError {
    kind: LexerErrorKind,
    span: Span,
}

impl LexerError {
    pub fn new(kind: LexerErrorKind, span: Span) -> Self {
        LexerError { kind, span }
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
