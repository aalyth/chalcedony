use crate::error::span::Position;
use crate::utils::Reader;
use std::collections::VecDeque;

pub struct CharReader {
    pos: Position,
    src: VecDeque<char>,
}

impl CharReader {
    pub fn new(source: String) -> CharReader {
        CharReader { 
            pos: Position::new(1, 1),
            src: source.chars().collect::<VecDeque<char>>(),
        }
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn advance_string(&mut self, cond: fn (&char) -> bool) -> String {
        let mut result = Vec::<char>::new();
        while !self.is_empty() && cond(self.peek().unwrap()) { 
            result.push(self.advance().unwrap()); 
        }
        return result.into_iter().collect();
    }
}

impl Reader<char> for CharReader {
    fn advance(&mut self) -> Option<char> {
        let result = self.src.pop_front();

        if let Some(val) = result {
            if val == '\n' { self.pos.advance_ln(); }
            else { self.pos.advance_col(); }
        }

        result
    }

    fn peek(&self) -> Option<&char> {
        self.src.front()
    }

    fn is_empty(&self) -> bool {
        self.src.is_empty()
    }

    #[allow(dead_code)]
    fn advance_while(&mut self, condition: fn (&char) -> bool) -> VecDeque<char> {
        let mut result = VecDeque::<char>::new();
        while !self.is_empty() && condition(self.peek().unwrap()) { 
            result.push_back(self.advance().unwrap()); 
        }
        return result;
    }
}
