use crate::lexer::line::Line;
use crate::lexer::tokens::{
    is_operator, is_special, Delimiter, Keyword, Operator, Special, Token, TokenKind,
};
use crate::lexer::CharReader;

use crate::error::span::{InlineSpanner, Position, Span, Spanning};
use crate::error::{ChalError, InternalError, LexerError};

use std::collections::VecDeque;
use std::rc::Rc;

pub struct Lexer {
    /* opening delimiters */
    delim_stack: VecDeque<Token>,

    /* to easily iterate over the source code*/
    reader: CharReader,

    /* so errors can be traced to the source code  */
    spanner: Rc<dyn Spanning>,

    /* clone of the previous token kind */
    prev: Option<TokenKind>,
}

impl Lexer {
    pub fn new(code: &str) -> Self {
        /* convert tabs to 4 spaces */
        let mut src = str::replace(code, "\t", "    ");

        /* this is so empty lines at the end do not cause errors */
        src.push_str("\n");

        // TODO: make a function for a file and for an inline lexer
        let mut result = Lexer {
            delim_stack: VecDeque::<Token>::new(),
            reader: CharReader::new(src),
            spanner: Rc::new(InlineSpanner::new(code)),
            prev: None,
        };
        result.remove_trailing_space();
        return result;
    }

    fn advance_chunk(&mut self) -> Result<VecDeque<Line>, ChalError> {
        let mut errors = Vec::<ChalError>::new();
        let mut result = VecDeque::<Line>::new();

        loop {
            match self.reader.peek() {
                Some(' ') | Some('\n') | Some('#') => {}
                Some('e') => match self.reader.peek_word().as_str() {
                    "elif" => {}
                    "else" => {}
                    _ => break,
                },
                Some(_) => break,
                None => break,
            }

            let (line, mut err) = self.advance_line();
            if !err.is_empty() {
                errors.append(&mut err);
            }

            if !err.is_empty() {
            } else {
                /* a line with a length of 1 is just an empty line */
                if line.tokens().len() >= 2 {
                    result.push_back(line);
                }
            }
        }
        if !errors.is_empty() {
            return Err(errors.into());
        }
        Ok(result)
    }

    /* advances the next program node */
    pub fn advance_prog(&mut self) -> Result<VecDeque<Line>, ChalError> {
        if self.reader.is_empty() {
            return Err(
                InternalError::new("Lexer::advance_prog(): advancing an empty lexer").into(),
            );
        }

        let mut result = VecDeque::<Line>::new();
        let mut errors = Vec::<ChalError>::new();
        let (mut line, mut err) = self.advance_line();

        /* a line with a length of 1 is just an empty line */
        while line.tokens().len() < 2 {
            if !err.is_empty() {
                return Err(err.into());
            }
            (line, err) = self.advance_line();
        }

        let front = line.tokens().front().unwrap().clone();

        match front.kind {
            TokenKind::Keyword(Keyword::Let) | TokenKind::Identifier(_) => result.push_back(line),

            TokenKind::Keyword(Keyword::Fn)
            | TokenKind::Keyword(Keyword::If)
            | TokenKind::Keyword(Keyword::While) => {
                result.push_back(line);
                result.append(&mut self.advance_chunk()?);
            }

            invalid @ _ => {
                self.remove_trailing_space();
                return Err(LexerError::invalid_global_statement(
                    invalid.clone(),
                    front.span.clone(),
                )
                .into());
            }
        }

        /* check for unclosed delimiters */
        if self.is_empty() && !self.delim_stack.is_empty() {
            for delim in &self.delim_stack {
                errors.push(LexerError::unclosed_delimiter(&delim.src, delim.span.clone()).into());
            }
        }

        if !errors.is_empty() {
            return Err(errors.into());
        }

        self.remove_trailing_space();
        return Ok(result);
    }

    fn advance_line(&mut self) -> (Line, Vec<ChalError>) {
        if self.reader.is_empty() {
            return (
                Line::new(0, VecDeque::<Token>::new()),
                vec![InternalError::new("Lexer::advance_line(): advancing an empty lexer").into()],
            );
        }

        let indent_raw = self.reader.advance_while(|c: &char| *c == ' ');
        let indent = indent_raw.len() as u64;

        let mut errors = Vec::<ChalError>::new();

        if indent % 4 != 0 {
            let pos = self.reader.pos();
            errors.push(LexerError::invalid_indentation(self.get_span(*pos, *pos)).into());
        }

        let mut result = VecDeque::<Token>::new();
        let mut current = self.advance();

        loop {
            match current {
                Ok(tok) => result.push_back(tok),
                Err(err) => errors.push(err),
            }
            /* check the current token type */
            if result.back().is_some() && result.back().unwrap().kind == TokenKind::Newline {
                break;
            }
            if self.is_empty() {
                break;
            }
            current = self.advance();
        }

        (Line::new(indent, result), errors)
    }

    fn advance_tok(
        &mut self,
        src: String,
        start: Position,
        end: Position,
    ) -> Result<Token, ChalError> {
        /* 1. create the token
         * 2. match the token:
         *  * delimiter:
         *      1. update the delimiter stack
         *      2. check for delimiter errors
         *
         *  * subtraction:
         *      1. check if the operator is binary or unary
         *
         * 3. update the prev token
         */
        let mut tok = Token::new(src, self.get_span(start, end))?;

        match &tok.kind {
            TokenKind::Delimiter(Delimiter::OpenPar)
            | TokenKind::Delimiter(Delimiter::OpenBrace)
            | TokenKind::Delimiter(Delimiter::OpenBracket) => {
                self.delim_stack.push_back(tok.clone())
            }

            /* only closing delimiters match here */
            TokenKind::Delimiter(close_delim) => {
                if self.delim_stack.back() == None {
                    return Err(LexerError::unexpected_closing_delimiter(
                        &tok.src,
                        tok.span.clone(),
                    )
                    .into());
                }

                let open_delim = self.delim_stack.pop_back().unwrap();

                if open_delim.kind != TokenKind::Delimiter(close_delim.inverse()) {
                    return Err(LexerError::mismatching_delimiters(
                        &open_delim.src,
                        &tok.src,
                        self.get_span(open_delim.span.start, start),
                    )
                    .into());
                }
            }

            /* here '-' operators are checked whether they are binary or unary */
            TokenKind::Operator(Operator::Sub) => match self.prev {
                Some(TokenKind::Operator(_))
                | Some(TokenKind::Delimiter(Delimiter::OpenPar))
                | Some(TokenKind::Special(Special::Comma))
                | Some(TokenKind::Keyword(_)) => tok = tok.into_neg()?,
                _ => (),
            },

            _ => (),
        };

        self.prev = Some(tok.kind.clone());

        Ok(tok)
    }

    fn advance(&mut self) -> Result<Token, ChalError> {
        let Some(mut current) = self.reader.advance() else {
            return Err(InternalError::new("Lexer::advance(): advancing an empty lexer").into());
        };

        while current == ' ' {
            let Some(curr) = self.reader.advance() else {
                return Err(
                    InternalError::new("Lexer::advance(): advancing an empty lexer").into(),
                );
            };
            current = curr;
        }
        let start = *self.reader.pos();

        if current == '#' {
            let _ = self.reader.advance_while(|c: &char| *c != '\n');
            self.reader.advance(); /* remove the \n if there's any */
            return self.advance_tok(String::from("\n"), *self.reader.pos(), *self.reader.pos());
        }

        if current.is_numeric()
            || (current == '-'
                && self.reader.peek().is_some()
                && self.reader.peek().unwrap().is_numeric())
        {
            /*
             * check wheather the minus should be interpreted as
             * a negative int or an operator, example:
             * 'a-5' -> identifier(a), sub(-), uint(5)
             * 'a*-5' -> identifier(a), mul(*), int(-5)
             */
            if current == '-' {
                match &self.prev {
                    Some(kind) => {
                        if kind.is_terminal() || *kind == TokenKind::Delimiter(Delimiter::ClosePar)
                        {
                            return self.advance_tok(
                                current.to_string(),
                                start,
                                *self.reader.pos(),
                            );
                        }
                    }
                    None => (),
                }
            }

            let src = String::from(current)
                + &self
                    .reader
                    .advance_while(|c: &char| c.is_numeric() || *c == '.');
            return self.advance_tok(src, start, *self.reader.pos());
        }

        if current.is_alphanumeric() || current == '_' {
            let src = String::from(current)
                + &self
                    .reader
                    .advance_while(|c: &char| c.is_alphanumeric() || *c == '_');
            return self.advance_tok(src, start, *self.reader.pos());
        }

        if is_special(&current) {
            let mut end = start;

            if !is_operator(&current) || self.reader.peek() == None {
                return self.advance_tok(current.to_string(), start, end);
            }

            let mut buffer = String::from(current);
            if let Some(c) = self.reader.peek() {
                buffer.push(c.clone())
            }

            match buffer.as_str() {
                "+=" | "-=" | "*=" | "/=" | "%=" | "&&" | "||" | ">=" | "<=" | "==" | "!="
                | "->" | ":=" => {
                    self.reader.advance();
                    end.advance_col();
                }
                _ => _ = buffer.pop(),
            }
            return self.advance_tok(buffer, start, end);
        }

        // NOTE: the position of the newline is actually wrong - it is on the start of the next
        // line, but that doesn't matter since it's only purpose is for end of line checks
        if current == '\n' {
            return self.advance_tok(String::from(current), start, start);
        }

        if current == '"' || current == '\'' {
            let mut src =
                String::from(current) + &self.reader.advance_while(|c: &char| *c != current);
            if let Some(c) = self.reader.advance() {
                src.push(c);
            } // adds the '"' at the end

            return self.advance_tok(src, start, *self.reader.pos());
        }

        return Err(InternalError::new("Lexer::advance(): could not parse token").into());
    }

    fn remove_trailing_space(&mut self) {
        if self.reader.is_empty() {
            return;
        }
        let mut current = *self.reader.peek().unwrap();
        while !self.reader.is_empty() && (current == '\n' || current == ' ' || current == '#') {
            current = self.reader.advance().unwrap();
            if current == '#' {
                self.reader.advance_while(|ch: &char| *ch != '\n');
                self.reader.advance(); /* remove the trailing newline */
            }
            if self.reader.is_empty() {
                break;
            }
            current = *self.reader.peek().unwrap();
        }
    }

    pub fn is_empty(&self) -> bool {
        self.reader.is_empty()
    }

    pub fn spanner(&self) -> Rc<dyn Spanning> {
        self.spanner.clone()
    }

    fn get_span(&self, start: Position, end: Position) -> Span {
        Span::new(start, end, self.spanner.clone())
    }
}
