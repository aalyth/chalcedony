use crate::error::span::Position;

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

    pub fn advance_while(&mut self, cond: impl Fn(&char) -> bool) -> String {
        let mut result = Vec::<char>::new();
        while !self.is_empty() && cond(self.peek().unwrap()) {
            result.push(self.advance().unwrap());
        }
        return result.into_iter().collect();
    }

    pub fn advance(&mut self) -> Option<char> {
        let result = self.src.pop_front();

        if let Some(val) = result {
            if val == '\n' {
                self.pos.advance_ln();
            } else {
                self.pos.advance_col();
            }
        }

        result
    }

    pub fn peek(&self) -> Option<&char> {
        self.src.front()
    }

    pub fn peek_word(&self) -> String {
        let mut result = String::new();
        for c in &self.src {
            if !c.is_alphabetic() {
                break;
            }
            result.push(*c);
        }
        result
    }

    pub fn is_empty(&self) -> bool {
        self.src.is_empty()
    }
}
