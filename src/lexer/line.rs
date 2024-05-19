use crate::lexer::{Token, TokenKind};
use std::collections::VecDeque;

/// Represents a single line inside the source code, containing the indentation
/// and the remaining tokens.
#[derive(Debug)]
pub struct Line {
    /* the indentation measured in the number of spaces (not tabulations) */
    pub indent: u64,
    pub tokens: VecDeque<Token>,
}

impl Line {
    pub fn new(indent: u64, tokens: VecDeque<Token>) -> Self {
        Line { indent, tokens }
    }

    pub fn front_tok(&self) -> Option<&Token> {
        self.tokens.front()
    }

    pub fn peek_kind(&self) -> Option<&TokenKind> {
        Some(&self.tokens.front()?.kind)
    }

    // NOTE: there is no need to implement an `is_empty()` method, since a line
    // must always contain at least a single token - `TokenKind::Newline`.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.tokens.len()
    }
}

impl From<Line> for VecDeque<Token> {
    fn from(value: Line) -> Self {
        value.tokens
    }
}
