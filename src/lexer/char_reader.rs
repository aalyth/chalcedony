use std::str::Chars;
use std::iter::Peekable;
use crate::errors::span::Position;

pub struct CharReader<'a> {
    pos: Position,
    src: Peekable<Chars<'a>>,
}

impl CharReader<'_> {
    pub fn new<'a>(src_code: &'a str) -> CharReader<'a> {
        CharReader { 
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

    pub fn peek(&mut self) -> Option<&char> {
        self.src.peek()
    }

    pub fn is_empty(&mut self) -> bool {
        self.src.peek() == None
    }

    pub fn pos(&mut self) -> &Position {
        &self.pos
    }

    pub fn advance_while(&mut self, condition: fn (char) -> bool) -> String {
        let mut result = Vec::<char>::new();
        while !self.is_empty() && condition(*self.peek().unwrap()) { 
            result.push(self.advance().unwrap()); 
        }
        return result.into_iter().collect();
    }

}
