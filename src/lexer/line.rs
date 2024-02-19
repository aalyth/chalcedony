use crate::lexer::Token;
use std::collections::VecDeque;

pub struct Line {
    /* the number of spaces in (not tabulations) */
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
