use crate::lexer::Token;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Line {
    indent: u64,
    tokens: VecDeque<Token>,
}

impl Line {
    /* pass the number of tabulations in
     * not the number of spaces
     */
    pub fn new(indent: u64, tokens: VecDeque<Token>) -> Self {
        Line { indent, tokens }
    }

    pub fn tokens(&self) -> &VecDeque<Token> {
        &self.tokens
    }

    pub fn indent(&self) -> u64 {
        self.indent
    }
}
