use crate::lexer::Token;
use std::collections::VecDeque;

/// Represents a single line inside the source code, containing the indentation
/// and the remaining tokens.
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

    pub fn len(&self) -> usize {
        self.tokens.len()
    }
}

impl From<Line> for VecDeque<Token> {
    fn from(value: Line) -> Self {
        value.tokens
    }
}
