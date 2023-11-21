use crate::lexer::line::Line;
use crate::lexer::tokens::{
    is_operator, is_special, Delimiter, Keyword, Operator, Special, Token, TokenKind,
};
use crate::lexer::CharReader;

use crate::error::{span::Position, span::Span, ChalError, InternalError, LexerError};

use crate::utils::Reader;

use std::collections::VecDeque;
use std::rc::Rc;

pub struct Lexer {
    /* opening delimiters */
    delim_stack: VecDeque<Token>,

    /* to easily iterate over the source code*/
    reader: CharReader,

    /* so errors can be traced to the source code  */
    span: Rc<Span>,

    /* clone of the previous token kind */
    prev: Option<TokenKind>,
}

impl Lexer {
    pub fn new(code: &str) -> Self {
        /* convert tabs to 4 spaces */
        let mut src = str::replace(code, "\t", "    ");

        /* this is so empty lines at the end do not cause errors */
        src.push_str("\n");

        Lexer {
            delim_stack: VecDeque::<Token>::new(),
            reader: CharReader::new(src),
            span: Rc::new(Span::new(code)),
            prev: None,
        }
    }

    /* advances the next program node (a fn def or variable def) */
    pub fn advance_prog(&mut self) -> Result<VecDeque<Line>, ChalError> {
        if self.reader.is_empty() {
            return Err(ChalError::from(InternalError::new(
                "Lexer::advance_prog(): advancing an empty lexer",
            )));
        }

        let mut result = VecDeque::<Line>::new();
        let mut errors = VecDeque::<ChalError>::new();
        let (mut line, mut err) = self.advance_line();

        while line.tokens().len() < 2 {
            if !err.is_empty() {
                return Err(ChalError::from(err));
            }
            (line, err) = self.advance_line();
        }

        let front = line.tokens().front().unwrap().clone();

        match front.kind() {
            TokenKind::Keyword(Keyword::Let) => result.push_back(line),

            TokenKind::Keyword(Keyword::Fn) => loop {
                match self.reader.peek() {
                    Some(' ') => (),
                    Some('\n') => (),
                    Some(_) => break,
                    None => break,
                }

                if !err.is_empty() {
                    errors.append(&mut err);
                } else {
                    if line.tokens().len() >= 2 {
                        result.push_back(line);
                    }
                }

                (line, err) = self.advance_line();
            },

            invalid @ _ => {
                return Err(ChalError::from(LexerError::invalid_global_statement(
                    invalid.clone(),
                    front.start(),
                    front.end(),
                    self.span().clone(),
                )))
            }
        }

        /* check for unclosed delimiters */
        if self.is_empty() && !self.delim_stack.is_empty() {
            for delim in &self.delim_stack {
                errors.push_back(ChalError::from(LexerError::unclosed_delimiter(
                    delim.src(),
                    delim.start(),
                    delim.end(),
                    self.span.clone(),
                )));
            }
        }

        if !errors.is_empty() {
            return Err(ChalError::from(errors));
        }
        return Ok(result);
    }

    fn advance_line(&mut self) -> (Line, VecDeque<ChalError>) {
        if self.reader.is_empty() {
            return (
                Line::new(0, VecDeque::<Token>::new()),
                VecDeque::from([ChalError::from(InternalError::new(
                    "Lexer::advance_line(): advancing an empty lexer",
                ))]),
            );
        }

        let start = *self.reader.pos();
        let indent_raw = self.reader.advance_string(|c: &char| *c == ' ');
        let indent = indent_raw.len() as u64;

        let mut errors = VecDeque::<ChalError>::new();

        if indent % 4 != 0 {
            errors.push_back(ChalError::from(LexerError::invalid_indentation(
                start.clone(),
                *self.reader.pos(),
                self.span.clone(),
            )));
        }

        let mut result = VecDeque::<Token>::new();
        let mut current = self.advance();

        loop {
            match current {
                Ok(tok) => result.push_back(tok),
                Err(err) => errors.push_back(err),
            }
            /* this clunky check is so we don't have problems with the borrow checker*/
            if result.back().is_some() && *result.back().unwrap().kind() == TokenKind::Newline {
                break;
            }
            if self.is_empty() {
                break;
            }
            current = self.advance();
        }

        (Line::new(indent / 4, result), errors)
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
        let mut tok = Token::new(src, start, end, &self.span)?;

        match tok.kind() {
            TokenKind::Delimiter(Delimiter::OpenPar)
            | TokenKind::Delimiter(Delimiter::OpenBrace)
            | TokenKind::Delimiter(Delimiter::OpenBracket) => {
                self.delim_stack.push_back(tok.clone())
            }

            /* only closing delimiters match here */
            TokenKind::Delimiter(close_delim) => {
                if self.delim_stack.back() == None {
                    return Err(ChalError::from(LexerError::unexpected_closing_delimiter(
                        tok.src(),
                        start,
                        end,
                        self.span.clone(),
                    )));
                }

                let open_delim = self.delim_stack.pop_back().unwrap();

                if *open_delim.kind() != TokenKind::Delimiter(close_delim.inverse()) {
                    return Err(ChalError::from(LexerError::mismatching_delimiters(
                        open_delim.src(),
                        tok.src(),
                        open_delim.start(),
                        start,
                        self.span.clone(),
                    )));
                }
            }

            /* NOTE: here '-' operators are checked if they are binary or unary */
            TokenKind::Operator(Operator::Sub) => match self.prev {
                Some(TokenKind::Operator(_))
                | Some(TokenKind::Delimiter(Delimiter::OpenPar))
                | Some(TokenKind::Special(Special::Comma)) => tok = tok.into_neg()?,
                _ => (),
            },

            _ => (),
        };

        self.prev = Some(tok.kind().clone());

        Ok(tok)
    }

    fn advance(&mut self) -> Result<Token, ChalError> {
        let start = *self.reader.pos();

        /* this way a do-while behaviour is achieved */
        let mut current: char = ' ';
        while current == ' ' {
            let Some(curr) = self.reader.advance() else {
                return Err(ChalError::from(InternalError::new(
                    "Lexer::advance(): advancing an empty lexer",
                )));
            };
            current = curr;
        }

        if current == '#' {
            let _ = self.reader.advance_string(|c: &char| *c != '\n');
            self.reader.advance(); /* remove the \n if there's any */
            return self.advance_tok(String::from("\n"), *self.reader.pos(), *self.reader.pos());
        }

        if current.is_alphanumeric() {
            let src = String::from(current)
                + &self
                    .reader
                    .advance_string(|c: &char| c.is_alphanumeric() || *c == '_');
            return self.advance_tok(src, start, *self.reader.pos());
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
                    .advance_string(|c: &char| c.is_numeric() || *c == '.');
            return self.advance_tok(src, start, *self.reader.pos());
        }

        if is_special(&current) {
            let mut end = start;
            end.advance_col();

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

        if current == '\n' {
            return self.advance_tok(String::from(current), start, *self.reader.pos());
        }

        if current == '"' {
            let mut src = String::from(current) + &self.reader.advance_string(|c: &char| *c != '"');
            if let Some(c) = self.reader.advance() {
                src.push(c);
            } // adds the '"' at the end

            return self.advance_tok(src, start, *self.reader.pos());
        }

        return Err(ChalError::from(InternalError::new(
            "Lexer::advance(): could not parse token",
        )));
    }

    pub fn is_empty(&self) -> bool {
        self.reader.is_empty()
    }

    pub fn span(&self) -> &Rc<Span> {
        &self.span
    }
}
