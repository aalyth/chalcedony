use std::collections::VecDeque;

use crate::errors::span::pos::Position;

use super::tokens::Token;

pub struct TokenReader {
    prev_start: Position, 
    prev_end:   Position,
    tokens:     Vec<Token>, 
}

impl TokenReader {
    fn new() -> TokenReader {
        // TODO! implement
    }

    fn advance() -> VecDeque<Token> {
        // check the next token:
        // keyword 'let' => new variable - end newline
        // keyword 'if'  => new if       - end closing delimiter
        // default => new statement      - end newline
    }
}
