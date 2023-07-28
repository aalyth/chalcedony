use crate::lexer::tokens::{Token, 
             TokenKind,
             Keyword,
             is_special, 
             is_operator};

use crate::errors::{LexerErrors,
                    span::Span,
                    span::Position,
                    };

use crate::lexer::CharReader;
use std::collections::VecDeque;

pub struct Lexer<'a> {  
    tokens: VecDeque<Token>,
    span: &'a Span,
}

impl Lexer<'_>{
    pub fn new(code: &str) -> Result<(Lexer, Span), ()> {
        let span = Span::new(code);

        let mut res = Lexer {
            tokens: VecDeque::new(),
            span: &span,
        };


        res.generate(code);
        res.check_errors()?;
        res.check_delimiters()?;

        Ok( (res, span) )
    }

    // generates the next sequence of token/s
    fn generate(&mut self, code: &str) { 
        let mut reader = CharReader::new(code);
        if reader.is_empty() { 
            panic!("Error: Lexer: generate(): empty string input.");
        }

        while !reader.is_empty() {
            let start = reader.pos().clone();
            let current = match reader.advance() {
                Some(val) => val,
                None      => return,
            };
            
            if current == '#' {
                reader.advance_while(|c| c != '\n');
            }

            if current.is_alphanumeric() {
                let src = String::from(current) + &reader.advance_while(|c| c.is_alphanumeric()); 

                self.tokens.push_back(Token::new(src, &start, reader.pos()));
            }

            if is_special(&current) {
                let src = String::from(current) + &reader.advance_while(|c| is_special(&c)); 
                match TokenKind::from(src.as_str()) {
                    TokenKind::None => self.split_special(&src, &start),
                    _ => self.tokens.push_back(Token::new(src, &start, reader.pos())),
                };
            }
            
            if current == '\n' {
                self.tokens.push_back(Token::new(String::from(current), &start, reader.pos()));
            }

            if current == '"' {
                let mut src = String::from(current) + &reader.advance_while(|c| c != '"' ); 
                if let Some(c) = reader.advance() { src.push(c); }  // adds the '"' at the end

                self.tokens.push_back(Token::new(src, &start, reader.pos()));
            }
        }

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

                self.tokens.push_back(Token::new(current.to_string(), start, &end));
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
            self.tokens.push_back(Token::new(buffer, start, &end))
        }
    }

    fn check_delim(&self, token: &Token, kind: TokenKind, stack: &mut Vec<Token>) -> Result<(), ()> {
        if let Some(tk) = stack.last() {
            if *tk.get_kind() != kind {
                LexerErrors::MissmatchingDelimiter::msg(tk.start(), token.end(), self.span, tk.src(), token.src());
                return Err(());
            } else {
                stack.pop();
            }
        } else {
            LexerErrors::UnexpectedClosingDelimiter::msg(token.start(), token.end(), self.span, token.src());
            return Err(());
        }
        Ok(())
    }

    fn check_delimiters(&mut self) -> Result<(), ()>{
        let mut stack = Vec::<Token>::new(); 

        for token in self.tokens.clone() {
            match token.get_kind() {
                TokenKind::OpenPar   => stack.push(token),
                TokenKind::OpenBrace => stack.push(token),

                TokenKind::ClosePar   => {
                    self.check_delim(&token, TokenKind::OpenPar, &mut stack)?;
                },

                TokenKind::CloseBrace   => {
                    self.check_delim(&token, TokenKind::OpenBrace, &mut stack)?;
                },

                _ => (),
            }
        }

        if !stack.is_empty() {
            let end = stack.pop().unwrap();
            LexerErrors::UnclosedDelimiter::msg(end.start(), end.end(), self.span, end.src());
            return Err(());
        }

        Ok(())
    }

    fn check_errors(&mut self) -> Result<(), ()> {
        let mut error = false;

        for token in &self.tokens {
            if let Err(_) = token.err_msg(self.span) {
                error = true;
            }
        }

        if error { return Err(()); }
        Ok(())
    }

    // returns the next program element => var def/declr or function def
    pub fn advance_program(&mut self) -> Option<VecDeque<Token>> {
        let mut result = VecDeque::new(); 
    
        if let Some(token) = self.tokens.front() {
            match token.get_kind() {
                TokenKind::Keyword(Keyword::Let) => {
                    result.push_back(token.clone());
                    result.append(&mut self.advance_while(|tk| *tk != TokenKind::Newline));
                },

                TokenKind::Keyword(Keyword::Fn) => {
                    let mut result = self.advance_while(|tk| *tk != TokenKind::OpenBrace); 
                    if let Some(token) = self.advance() {
                        result.push_back(token);
                    } else {
                        panic!("Error: Lexer: advance(): not enough tokens.");
                    }

                    let mut indents = 1;

                    while indents > 0 {
                        if let Some(token) = self.advance() {
                            match token.get_kind() {
                                TokenKind::OpenBrace => indents += 1,
                                TokenKind::CloseBrace => indents -= 1,
                                _ => ()
                            }
                            result.push_back(token);

                        } else {
                            panic!("Error: Lexer: advance(): not enough tokens.");
                        }
                        
                    }
                    return Some(result);
                },

                _ => {
                    LexerErrors::InvalidGlobalStatement::msg(token.start(), token.end(), self.span, token.get_kind());
                    return None;
                }
            }

        } else {
            panic!("Error: Lexer: advance(): advancing an empty lexer.");
        }

        Some(result)
    }

    // TODO! pass this function to the TokenReader
    /*
    pub fn expect(&mut self, span: &Span, exp: TokenKind) -> Result<Token, ()> {
        if let Some(tok) = self.peek() {
            // std::mem:discriminant() makes it so we can check only the outer enum variant
            // for example:
            // TokenKind::Identifier('main') is equal to TokenKind::Identifier('')
            if std::mem::discriminant(tok.get_kind()) == std::mem::discriminant(&exp) {
                return Ok(self.advance_token().unwrap());

            } else {
                ParserErrors::UnexpectedToken::msg(&tok, span);
            }

        } else {
            eprintln!("Error: Lexer: expect(): expecting from an empty lexer.");
            return Err(());
        }

        Err(())
    }
    */

    pub fn is_empty(&self) -> bool {
        self.tokens.len() == 0
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.front()
    }

    pub fn advance(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }
    
    pub fn advance_while(&mut self, condition: fn (&TokenKind) -> bool) -> VecDeque<Token> {
        let mut result = VecDeque::<Token>::new();
        while !self.is_empty() && condition(self.peek().unwrap().get_kind()) {
            result.push_back(self.advance().unwrap());
        }
        result
    }
}
