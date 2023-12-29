use crate::error::{ChalError, InternalError, LexerError, Span};
use crate::lexer::{Keyword, Line, Token, TokenKind};

use std::collections::VecDeque;
use std::rc::Rc;

use super::token_reader::TokenReader;

pub struct LineReader {
    src: VecDeque<Line>,
    span: Rc<Span>,
}

impl LineReader {
    pub fn new(src: VecDeque<Line>, span: Rc<Span>) -> Self {
        LineReader { src, span }
    }

    pub fn span(&self) -> Rc<Span> {
        self.span.clone()
    }

    pub fn peek_tok(&self) -> Option<&Token> {
        self.src.front()?.tokens().front()
    }

    pub fn advance(&mut self) -> Option<Line> {
        self.src.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.src.is_empty()
    }

    pub fn advance_until(
        &mut self,
        cond: impl Fn(&Line) -> bool,
    ) -> Result<VecDeque<Line>, ChalError> {
        let mut result = VecDeque::<Line>::new();
        let mut prev_indent;

        /* we advance at least the first line */
        let Some(front_ln) = self.advance() else {
            return Err(InternalError::new(
                "LexerReader::advance_chunk(): advancing an empty reader",
            )
            .into());
        };
        prev_indent = front_ln.indent();
        result.push_back(front_ln);

        while let Some(front) = self.src.front() {
            if cond(front) {
                break;
            }

            if front.indent().abs_diff(prev_indent) > 4 {
                return Err(LexerError::invalid_indentation(
                    front.tokens().front().unwrap().start(),
                    front.tokens().front().unwrap().end(),
                    self.span.clone(),
                )
                .into());
            }
            prev_indent = front.indent();

            result.push_back(self.advance().unwrap());
        }
        Ok(result)
    }

    pub fn advance_chunk(&mut self) -> Result<Self, ChalError> {
        let Some(front) = self.src.front() else {
            return Err(InternalError::new(
                "LexerReader::advance_chunk(): advancing an empty reader",
            )
            .into());
        };
        let indent = front.indent();
        let cond = |ln: &Line| -> bool { ln.indent() <= indent };

        let mut res = self.advance_until(cond)?;

        /* if the chunk is of type if statement check for else bodies */
        if let Some(front_ln) = res.front() {
            if let Some(front_tok) = front_ln.tokens().front() {
                if *front_tok.kind() != TokenKind::Keyword(Keyword::If) {
                    return Ok(LineReader::new(res, self.span.clone()));
                }
            }
        };
        while let Some(peek) = self.peek_tok() {
            match peek.kind() {
                TokenKind::Keyword(Keyword::Else) | TokenKind::Keyword(Keyword::Elif) => {
                    res.append(&mut self.advance_until(cond)?);
                }
                _ => break,
            }
        }

        Ok(LineReader::new(res, self.span.clone()))
    }

    pub fn advance_reader(&mut self) -> Result<TokenReader, ChalError> {
        let Some(next) = self.src.pop_front() else {
            return Err(InternalError::new(
                "LineReader::advance_reader(): advancing an empty reader",
            )
            .into());
        };

        Ok(TokenReader::new(next.into(), self.span.clone()))
    }
}
