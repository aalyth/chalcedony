use crate::error::span::{Span, Spanning};
use crate::error::{ChalError, ParserError, ParserErrorKind};
use crate::lexer::{Keyword, Line, Token, TokenKind};

use std::collections::VecDeque;
use std::rc::Rc;

use super::token_reader::TokenReader;

/// An abstraction, used to go over code chunks. For reference to code chunks
/// refer to the function `Lexer::advance_chunk()`.
pub struct LineReader {
    src: VecDeque<Line>,
    spanner: Rc<dyn Spanning>,
}

impl LineReader {
    pub fn new(src: VecDeque<Line>, spanner: Rc<dyn Spanning>) -> Self {
        LineReader { src, spanner }
    }

    pub fn spanner(&self) -> Rc<dyn Spanning> {
        self.spanner.clone()
    }

    pub fn indent(&self) -> Option<u64> {
        Some(self.src.front()?.indent)
    }

    pub fn peek_tok(&self) -> Option<&Token> {
        self.src.front()?.front_tok()
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

        /* we advance at least the first line */
        let Some(front_ln) = self.advance() else {
            return Ok(result);
        };
        result.push_back(front_ln);

        while let Some(front) = self.src.front() {
            if cond(front) {
                break;
            }

            result.push_back(self.advance().unwrap());
        }
        Ok(result)
    }

    pub fn advance_chunk(&mut self) -> Result<Self, ChalError> {
        let Some(front) = self.src.front() else {
            panic!("LineReader::advance_chunk(): advancing an empty reader");
        };
        // NOTE: this line is necessary so front goes out of scope and the
        // borrow checker is happy
        let indent = front.indent;
        let cond = |ln: &Line| -> bool { ln.indent <= indent };

        let mut res = self.advance_until(cond)?;

        // check wheter the resulting chunk is:
        //   - if statement -> get any `elif/else` branches
        //   - try/catch    -> get the `catch` statement

        let Some(front_ln) = res.front() else {
            return Ok(LineReader::new(res, self.spanner.clone()));
        };

        match front_ln.peek_kind().expect("empty line") {
            TokenKind::Keyword(Keyword::If) => res.extend(self.advance_if_branches(indent)?),
            TokenKind::Keyword(Keyword::Try) => {
                /* SAFETY: there is at least 1 line in the result */
                let last_line = res.back().unwrap();
                let last_span = &last_line.tokens.back().expect("empty line").span;
                res.extend(self.advance_catch_block(indent, last_span)?)
            }
            _ => {}
        };

        Ok(LineReader::new(res, self.spanner.clone()))
    }

    /// Advances the next line and builts a `TokenReader` over it.
    pub fn advance_reader(&mut self) -> TokenReader {
        let Some(next) = self.src.pop_front() else {
            panic!("LineReader::advance_reader(): advancing an empty reader");
        };

        TokenReader::new(next.into(), Span::from(self.spanner.clone()))
    }

    fn advance_if_branches(&mut self, indent: u64) -> Result<VecDeque<Line>, ChalError> {
        let mut res = VecDeque::<Line>::new();
        let cond = |ln: &Line| -> bool { ln.indent <= indent };

        while let Some(peek) = self.peek_tok() {
            match peek.kind {
                TokenKind::Keyword(Keyword::Elif) => res.append(&mut self.advance_until(cond)?),
                TokenKind::Keyword(Keyword::Else) => {
                    res.append(&mut self.advance_until(cond)?);
                    break;
                }
                _ => break,
            }
        }

        Ok(res)
    }

    fn advance_catch_block(
        &mut self,
        indent: u64,
        current_span: &Span,
    ) -> Result<VecDeque<Line>, ChalError> {
        let cond = |ln: &Line| -> bool { ln.indent <= indent };

        let Some(peek) = self.peek_tok() else {
            return Err(ParserError::new(
                ParserErrorKind::ExpectedToken(TokenKind::Keyword(Keyword::Catch)),
                current_span.clone(),
            )
            .into());
        };

        if peek.kind != TokenKind::Keyword(Keyword::Catch) {
            return Err(
                ParserError::new(ParserErrorKind::MissingCatchBlock, current_span.clone()).into(),
            );
        }

        self.advance_until(cond)
    }
}
