pub mod tokens;
//pub mod token_reader;
mod char_reader;

use tokens::{Token, 
             TokenKind, 
             is_special, 
             is_operator};

use crate::errors::{LexerErrors,
                    span::Span,
                    span::pos::Position,
                    ParserErrors,
                    };

use char_reader::CharReader;
use std::collections::VecDeque;

pub struct Lexer {  
    next: VecDeque<Token>,
}

impl Lexer {
    pub fn new(code: &str) -> Result<(Lexer, Span), ()> {
        let mut res = Lexer {
            next: VecDeque::new(),
        };

        let span = Span::new(code);

        res.generate(code)?;
        res.check_errors(&span)?;
        res.check_delimiters(&span)?;

        Ok( (res, span) )
    }

    // generates the next sequence of token/s
    fn generate(&mut self, code: &str) -> Result<(), ()> { 
        let mut reader = CharReader::new(code);
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
        let mut specials = CharReader::new(src);

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

    fn check_delim(&self, span: &Span, token: &Token, kind: TokenKind, stack: &mut Vec<Token>) -> Result<(), ()> {
        if let Some(tk) = stack.last() {
            if *tk.get_kind() != kind {
                LexerErrors::MissmatchingDelimiter::msg(tk.start(), token.end(), span, tk.src(), token.src());
                return Err(());
            } else {
                stack.pop();
            }
        } else {
            LexerErrors::UnexpectedClosingDelimiter::msg(token.start(), token.end(), span, token.src());
            return Err(());
        }
        Ok(())
    }

    fn check_delimiters(&mut self, span: &Span) -> Result<(), ()>{
        let mut stack = Vec::<Token>::new(); 

        for token in self.next.clone() {
            match token.get_kind() {
                TokenKind::OpenPar   => stack.push(token),
                TokenKind::OpenBrace => stack.push(token),

                TokenKind::ClosePar   => {
                    self.check_delim(span, &token, TokenKind::OpenPar, &mut stack)?;
                },

                TokenKind::CloseBrace   => {
                    self.check_delim(span, &token, TokenKind::OpenBrace, &mut stack)?;
                },

                _ => (),
            }
        }

        if !stack.is_empty() {
            let end = stack.pop().unwrap();
            LexerErrors::UnclosedDelimiter::msg(end.start(), end.end(), span, end.src());
            return Err(());
        }

        Ok(())
    }

    fn check_errors(&mut self, span: &Span) -> Result<(), ()> {
        let mut error = false;

        for token in &self.next {
            if let Err(_) = token.err_msg(span) {
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

    pub fn expect(&mut self, span: &Span, exp: TokenKind) -> Result<Token, ()> {
        if let Some(tok) = self.peek() {
            // std::mem:discriminant() makes it so we can check only the outer enum variant
            // for example:
            // TokenKind::Identifier('main') is equal to TokenKind::Identifier('')
            if std::mem::discriminant(tok.get_kind()) == std::mem::discriminant(&exp) {
                return Ok(self.advance().unwrap());

            } else {
                ParserErrors::UnexpectedToken::msg(&tok, span);
            }

        } else {
            eprint!("Error: Lexer: expect(): expecting from an empty lexer.\n");
            return Err(());
        }

        Err(())
    }

    pub fn advance_while(&mut self, condition: fn(&TokenKind) -> bool) -> VecDeque<Token> {
        let mut result = VecDeque::<Token>::new();
        while !self.is_empty() && condition(self.peek().unwrap().get_kind()) { 
            result.push_back(self.advance().unwrap()); 
        }
        result
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.next.get(0)
    }

}

