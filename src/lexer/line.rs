use crate::lexer::Token;
use std::collections::VecDeque;

pub struct Line {
    /* the number of spaces in (not tabulations) */
    indent: u64,
    tokens: VecDeque<Token>,
}

impl Line {
    pub fn new(indent: u64, tokens: VecDeque<Token>) -> Self {
        Line { indent, tokens }
    }

    pub fn tokens(&self) -> &VecDeque<Token> {
        &self.tokens
    }

    pub fn indent(&self) -> u64 {
        self.indent
    }

    pub fn front_tok(&self) -> Option<&Token> {
        self.tokens.front()
    }
}

impl From<Line> for VecDeque<Token> {
    fn from(value: Line) -> Self {
        value.tokens
    }
}
