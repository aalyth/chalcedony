//! The module representing the first stage of the interpreting process. It is
//! responsible for converting the source code from a stream of text into a
//! stream of [`Tokens`].
//!
//! This part of the interpreter is arguably the simplest, but it plays the most
//! fundamental role in the whole `Chalcedony` interpreter.

mod line;
mod tokens;

mod char_reader;

pub use line::Line;
pub use tokens::{Delimiter, Keyword, Operator, Special, Token, TokenKind};

use crate::error::span::{InlineSpanner, Position, Span, Spanning};
use crate::error::{ChalError, LexerError, LexerErrorKind};
use crate::lexer::tokens::{is_operator, is_special};
use crate::utils::Stack;
use char_reader::CharReader;

use std::collections::VecDeque;
use std::fs;
use std::rc::Rc;

/// The structure, used to transform the given script into a series of tokens.
pub struct Lexer {
    /// Keeps track of the currently open delimiters - used to check for
    /// any mismatching delimiters.
    delim_stack: Stack<Token>,

    /// The reader, used to iterate over the source and keep track of the
    /// current position.
    reader: CharReader,

    /// The spanner object, used to build code snippets to the errror messages.
    spanner: Rc<dyn Spanning>,

    /// The previous parsed token - used to perform checks such as determening
    /// whether a `-` should be treated as an unary or binary operator.
    prev: Option<TokenKind>,
}

impl Lexer {
    pub fn new(code: &str, filename: Option<String>) -> Self {
        /* convert tabs to 4 spaces */
        let mut src = str::replace(code, "\t", "    ");

        /* this is so empty lines at the end do not cause errors */
        src.push('\n');

        // NOTE: in the case of implementing an inline lexer, a separate
        // function must be created
        let mut result = Lexer {
            delim_stack: Stack::<Token>::new(),
            reader: CharReader::new(src),
            spanner: Rc::new(InlineSpanner::new(code, filename)),
            prev: None,
        };

        result.remove_trailing_space();
        result
    }

    pub fn from_file(filename: String) -> Option<Self> {
        let Ok(script) = fs::read_to_string(filename.clone()) else {
            return None;
        };
        Some(Self::new(&script, Some(filename)))
    }

    /// Advances the next code chunk - a code chunk is defined as the next line,
    /// followed by any lines whose indentation is other than 0.
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
                Some('c') => match self.reader.peek_word().as_str() {
                    "catch" => {}
                    _ => break,
                },
                Some(_) => break,
                None => break,
            }

            match self.advance_line() {
                Ok(line) => {
                    /* a line with a length of 1 is just an empty line */
                    if line.len() >= 2 {
                        result.push_back(line)
                    }
                }
                Err(err) => errors.push(err),
            }
        }

        if !errors.is_empty() {
            return Err(errors.into());
        }

        Ok(result)
    }

    /// Advances the next interpretable code unit, i.e. a stream of tokens which
    /// could be built into a `NodeProg` node. A program node does not necessary
    /// mean a single code chunk - for example an `if` statement with one or
    /// more `else/elif` branches.
    pub fn advance_prog(&mut self) -> Result<VecDeque<Line>, ChalError> {
        if self.reader.is_empty() {
            panic!("Lexer::advance_prog(): advancing an empty lexer")
        }

        let mut result = VecDeque::<Line>::new();
        let mut errors = Vec::<ChalError>::new();
        let mut line: Line = self.advance_line()?;

        /* a line with a length of 1 is just an empty line */
        while line.len() < 2 {
            line = self.advance_line()?;
        }

        let front = line.front_tok().unwrap().clone();

        match front.kind {
            TokenKind::Keyword(Keyword::Let)
            | TokenKind::Identifier(_)
            | TokenKind::Keyword(Keyword::Import)
            | TokenKind::Keyword(Keyword::Const) => result.push_back(line),

            TokenKind::Keyword(Keyword::Fn)
            | TokenKind::Keyword(Keyword::If)
            | TokenKind::Keyword(Keyword::While)
            | TokenKind::Keyword(Keyword::Try)
            | TokenKind::Keyword(Keyword::For)
            | TokenKind::Keyword(Keyword::Class) => {
                result.push_back(line);
                result.extend(self.advance_chunk()?);
            }

            invalid => {
                self.remove_trailing_space();
                return Err(LexerError::new(
                    LexerErrorKind::InvalidGlobalStatement(invalid.clone()),
                    front.span.clone(),
                )
                .into());
            }
        }

        /* check for any unclosed delimiters */
        if !self.delim_stack.is_empty() {
            while let Some(delim) = self.delim_stack.pop() {
                errors.push(
                    LexerError::new(LexerErrorKind::UnclosedDelimiter(delim.src), delim.span)
                        .into(),
                );
            }
        }

        // NOTE: the delim stack always remains empty after every program node
        if !errors.is_empty() {
            return Err(errors.into());
        }

        self.remove_trailing_space();
        Ok(result)
    }

    fn advance_line(&mut self) -> Result<Line, ChalError> {
        if self.reader.is_empty() {
            panic!("Lexer::advance_line(): advancing an empty lexer")
        }

        let indent_raw = self.reader.advance_while(|c: &char| *c == ' ');
        let indent = indent_raw.len() as u64;

        let mut result = VecDeque::<Token>::new();
        let mut errors = Vec::<ChalError>::new();

        if indent % 4 != 0 {
            let pos = self.reader.pos();
            errors.push(
                LexerError::new(
                    LexerErrorKind::InvalidIndentation,
                    self.get_span(*pos, *pos),
                )
                .into(),
            );
        }

        loop {
            if self.is_empty() {
                break;
            }

            match self.advance() {
                Ok(tok) => result.push_back(tok),
                Err(err) => errors.push(err),
            }
            /* check the current token type */
            if result.back().unwrap().kind == TokenKind::Newline {
                break;
            }
        }

        if !errors.is_empty() && result.len() > 1 {
            return Err(errors.into());
        }

        Ok(Line::new(indent, result))
    }

    /* the internal function, used to build tokens */
    fn advance_tok(
        &mut self,
        src: String,
        start: Position,
        end: Position,
    ) -> Result<Token, ChalError> {
        // 1. create the token
        // 2. match the token:
        //  * delimiter:
        //      1. update the delimiter stack
        //      2. check for delimiter errors
        //
        //  * subtraction:
        //      1. check whether the operator is binary or unary
        //
        // 3. update the prev token
        let mut tok = Token::new(src, self.get_span(start, end))?;

        match &tok.kind {
            TokenKind::Delimiter(Delimiter::OpenPar)
            | TokenKind::Delimiter(Delimiter::OpenBrace)
            | TokenKind::Delimiter(Delimiter::OpenBracket) => self.delim_stack.push(tok.clone()),

            /* only closing delimiters match here */
            TokenKind::Delimiter(close_delim) => {
                let Some(open_delim) = self.delim_stack.pop() else {
                    return Err(LexerError::new(
                        LexerErrorKind::UnexpectedClosingDelimiter(tok.src),
                        tok.span,
                    )
                    .into());
                };

                if open_delim.kind != TokenKind::Delimiter(close_delim.inverse()) {
                    return Err(LexerError::new(
                        LexerErrorKind::MismatchingDelimiters(open_delim.src, tok.src),
                        self.get_span(open_delim.span.start, start),
                    )
                    .into());
                }
            }

            // here '-' operators are checked whether they are binary or unary
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

    /* advances the next token in the source code */
    pub fn advance(&mut self) -> Result<Token, ChalError> {
        let Some(mut current) = self.reader.advance() else {
            panic!("Lexer::advance(): advancing an empty lexer")
        };

        while current == ' ' {
            let Some(curr) = self.reader.advance() else {
                panic!("Lexer::advance(): advancing an empty lexer")
            };
            current = curr;
        }
        let start = *self.reader.pos();

        /* a comment */
        if current == '#' {
            let _ = self.reader.advance_while(|c: &char| *c != '\n');
            self.reader.advance(); /* remove the \n if there's any */
            return self.advance_tok(String::from("\n"), *self.reader.pos(), *self.reader.pos());
        }

        /* a number */
        if current.is_numeric()
            || (current == '-'
                && self.reader.peek().is_some()
                && self.reader.peek().unwrap().is_numeric())
        {
            // check wheather the minus should be interpreted as a negative int
            // or an operator, example:
            // 'a-5' -> identifier(a), sub(-), uint(5)
            // 'a*-5' -> identifier(a), mul(*), int(-5)
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
                    None => {}
                }
            }

            let src = String::from(current)
                + &self
                    .reader
                    .advance_while(|c: &char| c.is_numeric() || *c == '.' || *c == '_');
            return self.advance_tok(src, start, *self.reader.pos());
        }

        /* an identifier */
        if current.is_alphabetic() || current == '_' {
            let mut src = String::from(current)
                + &self
                    .reader
                    .advance_while(|c: &char| c.is_alphanumeric() || *c == '_');

            /* only function names can end with a '!' */
            if self.reader.peek() == Some(&'!') && self.reader.peek_nth(1) == Some(&'(') {
                src.push(self.reader.advance().unwrap());
            }

            return self.advance_tok(src, start, *self.reader.pos());
        }

        /* any operator or special character */
        if is_special(&current) {
            let mut end = start;

            if !is_operator(&current) || self.reader.peek().is_none() {
                return self.advance_tok(current.to_string(), start, end);
            }

            let mut buffer = String::from(current);
            if let Some(c) = self.reader.peek() {
                buffer.push(*c)
            }

            match buffer.as_str() {
                "+=" | "-=" | "*=" | "/=" | "%=" | "&&" | "||" | ">=" | "<=" | "==" | "!="
                | "->" | ":=" | "::" => {
                    self.reader.advance();
                    end.advance_col();
                }
                _ => _ = buffer.pop(),
            }
            return self.advance_tok(buffer, start, end);
        }

        if current == '\\' {
            current = self.reader.advance().expect("expected a token");
            while current == ' ' {
                current = self.reader.advance().expect("expected a token");
            }
            if current != '\n' {
                return Err(LexerError::new(
                    LexerErrorKind::InvalidNewlineEscape,
                    self.get_span(start, start),
                )
                .into());
            }

            return self.advance();
        }

        // If the current is a newline, but there are open delimiters, the lexer
        // moves on as if there is no newline - this is one of the newer
        // features of the Python language
        // NOTE: the position of the newline is actually wrong - it is on the
        // start of the next line, but that doesn't matter since it's only
        // purpose is for end of line checks
        if current == '\n' {
            if self.delim_stack.is_empty() || self.is_empty() {
                return self.advance_tok(String::from(current), start, start);
            }
            return self.advance();
        }

        /* any string */
        if current == '"' || current == '\'' {
            let mut src =
                String::from(current) + &self.reader.advance_while(|c: &char| *c != current);
            if let Some(c) = self.reader.advance() {
                /* adds the `"` at the end */
                src.push(c);
            }

            return self.advance_tok(src, start, *self.reader.pos());
        }

        Err(LexerError::new(
            LexerErrorKind::InvalidChar(current),
            self.get_span(start, start),
        )
        .into())
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
