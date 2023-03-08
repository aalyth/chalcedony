mod tokens;
mod reader;

use tokens::{Token, TokenKind};
use super::errors::span::Span;
use crate::errors::lexer_errors::LexerError;
use reader::Reader;
use std::iter::Peekable;
use std::collections::{HashSet, VecDeque};


pub struct Lexer {  
    tokens: VecDeque<Token>,
    src: Span,
}

impl Lexer {
    pub fn new(src_code: &str) -> Option<Lexer> {
        let mut reader = Reader::new(src_code.clone());

        let mut tokens = VecDeque::<Token>::new();
        let src = Span::new(src_code);

        let special_chars = HashSet::from([
               '(', ')', '[', ']', 
               '{', '}', ':', ';',
               '+', '-', '*', '/', 
               '%', '=', '<', '>', 
               '!', '&', '|', '~',
               '^', ','
        ]);
        
        let mut buffer = "".to_string();
        let mut buffer_pos = reader.position().clone();
        while !reader.is_empty() {
            let current = reader.advance().unwrap();  

            if current == '\t' { continue; }

            // a single line comment
            if current == '#' {
                tokens.push_back(Token::new(buffer.clone(), buffer_pos, reader.prev_position()));
                reader.advance_while(|x| x != '\n');
                buffer = "".to_string();
                buffer_pos = reader.position();
                continue;
            }

            // Multiline comment
            if current == '$' {
                tokens.push_back(Token::new(buffer.clone(), buffer_pos, reader.prev_position()));
                buffer_pos = reader.position();

                buffer = "".to_string();
                reader.advance_while(|x| x != '$');

                // unclosed comment
                if reader.is_empty() {
                    tokens.push_back(Token::err(buffer_pos.clone(), reader.prev_position(), LexerError::UnclosedComment));
                    break;
                }

                buffer.push(reader.advance().unwrap());
                buffer_pos = reader.position();
                continue;
            }

            // String 
            if current == '"' {
                tokens.push_back(Token::new(buffer.clone(), buffer_pos, reader.prev_position()));
                buffer_pos = reader.position();
                buffer = "".to_string();
                buffer.push(current);
                buffer.push_str(&reader.advance_while(|x| x != '"'));

                // unclosed quote
                if reader.is_empty() {
                    tokens.push_back(Token::err(buffer_pos.clone(), reader.prev_position(), LexerError::UnclosedString));
                    break;
                }

                buffer.push(reader.advance().unwrap());
                tokens.push_back(Token::new(buffer.clone(), buffer_pos, reader.prev_position()));

                buffer = "".to_string();
                buffer_pos = reader.position();
                continue;
            }

            if current == ' ' { 
                let curr_token = Token::new(buffer.clone(), buffer_pos, reader.prev_position());

                tokens.push_back(curr_token);
                buffer = "".to_string();
                buffer_pos = reader.position();
                continue;
            }

            if special_chars.contains(&current) {
                tokens.push_back(Token::new(buffer.clone(), buffer_pos, reader.prev_position()));

                buffer_pos = reader.prev_position();
                buffer = "".to_string();
                buffer.push(current);

                match reader.peek() {
                    None => (),
                    Some(chr) => {
                        buffer.push(chr.clone());
                        match buffer.as_str() {
                            "+=" | "-=" | "*=" | "/=" | 
                            "%=" | "==" | "!=" | "<=" | 
                            ">=" | "->" => reader.advance(),
                            _ => buffer.pop(),
                        };
                    }
                }

                tokens.push_back(Token::new(buffer.clone(), buffer_pos, reader.position()));

                buffer = "".to_string();
                buffer_pos = reader.position();
                continue;
            }

            buffer.push(current);
        }

        tokens.push_back(Token::new(buffer.clone(), buffer_pos.clone(), reader.prev_position()));
        tokens.retain(|x| *x.get_kind() != TokenKind::None);

        match Lexer::check_errors(&tokens, &src) {
            Err(_) => return None,
            Ok(_) => (), 
        }

        match Lexer::check_scoping(&tokens, &src) {
            Err(_) => return None,
            Ok(_) => (),
        }

        Some(Lexer {
            tokens: tokens.clone(),
            src: src,
        })
    }

    fn check_errors(tokens: &VecDeque<Token>, src: &Span) -> Result<(), ()> {
        let mut errs = false; 
        for i in tokens {
            match i.get_kind() {
                TokenKind::Error(_) => {
                    i.err_msg(src);             
                    errs = true;
                }

                _ => (),
            }
        }

        if errs { return Err(()); }
        Ok(())
    }

    fn check_scoping(tokens: &VecDeque<Token>, src: &Span) -> Result<(), ()> {
        let mut openers = VecDeque::<Token>::new(); // contains opening delimiters
        for i in tokens {
            match i.get_kind() {
                TokenKind::OpenPar     => openers.push_back(Token::new("(".to_string(), i.start_pos().clone(), i.end_pos().clone())),
                TokenKind::OpenBrace   => openers.push_back(Token::new("{".to_string(), i.start_pos().clone(), i.end_pos().clone())),
                TokenKind::OpenBracket => openers.push_back(Token::new("[".to_string(), i.start_pos().clone(), i.end_pos().clone())),
                
                TokenKind::ClosePar | TokenKind::CloseBrace | TokenKind::CloseBracket => {
                    if openers.back() == None {
                        let err = Token::err(i.start_pos().clone(), i.end_pos().clone(), LexerError::UnexpectedClosingDelimiter(i.src().to_string()));
                        err.err_msg(src);
                        return Err(());
                    }

                    let prev_del = openers.back().unwrap();
                    let missmatching: bool = match prev_del.get_kind() {
                        TokenKind::OpenPar     =>  *i.get_kind() != TokenKind::ClosePar,
                        TokenKind::OpenBrace   =>  *i.get_kind() != TokenKind::CloseBrace,
                        TokenKind::OpenBracket =>  *i.get_kind() != TokenKind::CloseBracket,
                        _ => false,
                    };

                    if missmatching {
                        let err = Token::err(prev_del.start_pos().clone(), i.end_pos().clone(), LexerError::MissmatchingDelimiter(prev_del.src().to_string(), i.src().to_string()));
                        err.err_msg(src);
                        return Err(());
                    }
                    openers.pop_back();
                },

                _ => (),
            }
        }

        if !openers.is_empty() {
            let end_del = openers.back().unwrap();
            let err = Token::err(end_del.start_pos().clone(), end_del.end_pos().clone(), LexerError::UnclosedDelimiter(end_del.src().to_string()));
            err.err_msg(src);
            return Err(());
        }
        Ok(())
    }

    pub fn advance(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.get(0)
    }

    pub fn is_empty(&mut self) -> bool {
        self.tokens.is_empty()
    }
}

