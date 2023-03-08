use std::str::Chars;
use std::iter::Peekable;
use crate::errors::lexer_errors::LexerError;
use crate::errors::span::pos::Position;

pub struct Reader<'a> {
    pos: Position,
    src: Peekable<Chars<'a>>,
}

impl Reader<'_> {
    pub fn new(src_code: &str) -> Reader<'_> {
        Reader { 
            pos: Position::new(1, 1),
            src: src_code.chars().peekable(), 
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        let result = self.src.next();

        if let Some(val) = result {
            if val == '\n' { self.pos.advance_ln(); }
            else { self.pos.advance_col(); }
        }

        result
    }

    pub fn peek(&mut self) -> Option<char> {
        let result = self.src.peek()?;
        Some(result.clone())
    }

    pub fn is_empty(&mut self) -> bool {
        match self.src.peek() {
            Some(_) => false,
            None => true,
        }
    }

    pub fn prev_position(&self) -> Position {
        if self.pos. ln == 1 && self.pos.col == 1 { return Position::new(1, 1); }
        if self.pos.col - 1 == 0 { return Position::new(self.pos.ln - 1, self.pos.col); }
        Position::new(self.pos.ln, self.pos.col - 1)
    }

    pub fn position(&self) -> Position {
        Position::new(self.pos.ln, self.pos.col)
    }

    pub fn next_position(&mut self) -> Position {
        match self.peek() {
            Some('\n') => return Position::new(self.pos.ln + 1, 1),
            _ => return Position::new(self.pos.ln, self.pos.col),
        }
    }

    pub fn advance_while(&mut self, condition: fn (char) -> bool) -> String {
        let mut result = Vec::<char>::new();
        while !self.is_empty() && condition(self.peek().unwrap()) { result.push(self.advance().unwrap()); }
        return result.into_iter().collect();
    }
}
