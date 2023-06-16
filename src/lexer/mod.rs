mod tokens;
mod reader;

use tokens::{Token, TokenKind, Keyword, is_special, is_operator};
use crate::errors::lexer_errors::LexerError;
use super::errors::span::Span;
use super::errors::span::pos::Position;
use reader::Reader;
use std::collections::VecDeque;

pub struct Lexer {  
    next: VecDeque<Token>,
    span: Span,
}

impl Lexer {
    pub fn new(code: &str) -> Result<Lexer, ()> {
        let mut res = Lexer {
            next: VecDeque::new(),
            span: Span::new(code),
        };

        res.generate(code)?;
        res.check_errors()?;
        res.check_delim()?;

        Ok(res)
    }

    // generates the next sequence of token/s
    fn generate(&mut self, code: &str) -> Result<(), ()> { 
        let mut reader = Reader::new(code);
        if reader.is_empty() { 
            eprintln!("Error: Lexer: generate(): empty string input.");
            return Err(()); 
        }

        while !reader.is_empty() {
            let start = reader.pos().clone();
            let current = match reader.advance() {
                Some(val) => val,
                None      => return Ok(()),
            };
            
            if current == '#' {
                reader.advance_while(|c| c != '\n');
            }

            if current.is_alphanumeric() {
                let src = String::from(current) + &reader.advance_while(|c| c.is_alphanumeric()); 

                self.next.push_back(Token::new(src, &start, reader.pos()));
            }

            if is_special(&current) {
                let src = String::from(current) + &reader.advance_while(|c| is_special(&c)); 
                match TokenKind::from(src.as_str()) {
                    TokenKind::None => self.split_special(&src, &start),
                    _ => self.next.push_back(Token::new(src, &start, reader.pos())),
                };
            }
            
            if current == '\n' {
                self.next.push_back(Token::new(String::from(current), &start, reader.pos()));
            }

            if current == '"' {
                let mut src = String::from(current) + &reader.advance_while(|c| c != '"' ); 
                if let Some(c) = reader.advance() { src.push(c); }  // adds the '"' at the end

                self.next.push_back(Token::new(src, &start, reader.pos()));
            }
        }
        
        Ok(())
    }

    // this takes all special characters and conjoins any double operators such as '+=', '-=', etc.
    fn split_special(&mut self, src: &str, start: &Position) {
        let mut specials = Reader::new(src);

        while !specials.is_empty() {
            let current = specials.advance().unwrap();
            let mut end = start.clone();
            end.advance_col();

            if !is_operator(&current) || 
                    specials.peek() == None {

                self.next.push_back(Token::new(current.to_string(), start, &end));
                continue;
            }

            let mut buffer = String::from(current);
            if let Some(c) = specials.peek() { buffer.push(c.clone()) }

            match buffer.as_str() {
                "+=" | "-=" | "*=" | "/=" | 
                "%=" | "==" | "!=" | "<=" | 
                ">=" | "->" => {
                    specials.advance();
                    end.advance_col();
                },

                _ => _ = buffer.pop(),
            }
            self.next.push_back(Token::new(buffer, start, &end))
        }
    }

    fn delim_missmatch(&self, start: &Token, end: &Token) {
        let err = Token::err(start.start(), end.end(), &LexerError::MissmatchingDelimiter(start.src().to_string(), end.src().to_string()));
        err.err_msg(&self.span).ok();
    }

    fn delim_unexpected(&self, token: &Token) {
        let err = Token::err(token.start(), token.end(), &LexerError::UnexpectedClosingDelimiter(token.src().to_string()));
        err.err_msg(&self.span).ok();
    }

    fn check_delimiter(&self, token: &Token, kind: TokenKind, stack: &mut Vec<Token>) -> Result<(), ()> {
        if let Some(tk) = stack.last() {
            if *tk.get_kind() != kind {
                self.delim_missmatch(tk, token);
                return Err(());
            } else {
                stack.pop();
            }
        } else {
            self.delim_unexpected(token);
            return Err(());
        }
        Ok(())
    }

    fn check_delim(&mut self) -> Result<(), ()>{
        let mut stack = Vec::<Token>::new(); 

        for token in self.next.clone() {
            match token.get_kind() {
                TokenKind::Keyword(kw) => match kw {
                    Keyword::Fn => stack.push(token),
                    Keyword::If => stack.push(token),

                    Keyword::Nf => {
                        self.check_delimiter(&token, TokenKind::Keyword(Keyword::Fn), &mut stack)?;
                    }
                    
                    Keyword::Fi => {
                        self.check_delimiter(&token, TokenKind::Keyword(Keyword::If), &mut stack)?;
                    }

                    _ => (),
                },

                TokenKind::OpenPar   => stack.push(token),
                TokenKind::OpenBrace => stack.push(token),

                TokenKind::ClosePar   => {
                    self.check_delimiter(&token, TokenKind::OpenPar, &mut stack)?;
                },

                TokenKind::CloseBrace   => {
                    self.check_delimiter(&token, TokenKind::OpenBrace, &mut stack)?;
                },

                _ => (),
            }
        }

        if !stack.is_empty() {
            let end = stack.pop().unwrap();
            let err = Token::err(end.start(), end.end(), &LexerError::UnclosedDelimiter(end.src().to_string()));
            err.err_msg(&self.span).ok();
            return Err(());
        }

        Ok(())
    }

    fn check_errors(&mut self) -> Result<(), ()> {
        let mut error = false;

        for token in &self.next {
            if let Err(_) = token.err_msg(&self.span) {
                error = true;
            }
        }

        if error { return Err(()); }
        Ok(())
    }

    pub fn is_empty(&mut self) -> bool {
        self.next.len() == 0
    }

    pub fn advance(&mut self) -> Option<Token> {
        self.next.pop_front()
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.next.get(0)
    }

}

