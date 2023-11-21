use crate::error::{ChalError, InternalError, LexerError, Position, Span};
use crate::lexer::{Line, Token};

use std::collections::VecDeque;
use std::rc::Rc;

pub struct LineReader {
    src: VecDeque<Line>,
    pos: Position,
    span: Rc<Span>,
}

impl LineReader {
    pub fn new(src: VecDeque<Line>, span: Rc<Span>) -> Self {
        let mut pos = Position::new(1, 1);

        /* check if there is at least 1 token in the source
         * and take the first token's end position */
        if !src.is_empty() && !src.front().unwrap().tokens().is_empty() {
            pos = src.front().unwrap().tokens().front().unwrap().end();
        }

        LineReader { src, pos, span }
    }

    pub fn peek_tok(&self) -> Option<&Token> {
        self.src.front()?.tokens().front()
    }

    pub fn peek_indent(&self) -> Option<u64> {
        Some(self.src.front()?.indent())
    }

    pub fn advance(&mut self) -> Option<Line> {
        self.src.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.src.is_empty()
    }

    fn advance_until(&mut self, cond: impl Fn(&Line) -> bool) -> Result<VecDeque<Line>, ChalError> {
        let mut result = VecDeque::<Line>::new();
        let mut prev_indent = 0;

        if let Some(front) = self.src.front() {
            prev_indent = front.indent();
        }

        while let Some(front) = self.src.front() {
            if !cond(front) {
                break;
            }

            if front.indent().abs_diff(prev_indent) > 4 {
                return Err(ChalError::from(LexerError::invalid_indentation(
                    front.tokens().front().unwrap().start(),
                    front.tokens().front().unwrap().end(),
                    self.span.clone(),
                )));
            }

            result.push_back(self.src.pop_front().unwrap());
        }
        Ok(result)
    }

    pub fn advance_chunk(&mut self) -> Result<VecDeque<Line>, ChalError> {
        let Some(front) = self.src.front() else {
            return Err(ChalError::from(InternalError::new(
                "LexerReader::advance_chunk(): advancing an empty reader",
            )));
        };
        let indent = front.indent();
        let cond = |ln: &Line| -> bool { ln.indent() > indent };
        self.advance_until(cond)
    }
}
