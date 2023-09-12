use crate::lexer::tokens::{Token, 
             TokenKind,
             Keyword,
             Delimiter,
             is_special, 
             is_operator,
             };

use crate::error::{ChalError,
                   LexerError,
                   InternalError,
                   span::Span,
                   span::Position,
                   };

use crate::lexer::CharReader;
use std::collections::VecDeque;
use std::rc::Rc;

use regex::Regex;

pub struct Lexer<'a> {  
    /* contains the opening delimiters */
    delim_stack: VecDeque<&'a Token>,
    reader:      CharReader<'a>,
    span:        Rc<Span>,
    prev:        Option<&'a Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &str) -> Self {
        /* convert tabs to 4 spaces */
        let src = &str::replace(code, "\t", "    ");

        /* remove comments */
        let re = Regex::new(r"#.*$").unwrap();
        let src = &re.replace(src, "").into_owned();

        /* remove empty lines*/
        let re = Regex::new(r"^[ \t]*$").unwrap();
        let src = &re.replace(src, "").into_owned();

        Lexer {
            delim_stack: VecDeque::<&Token>::new(),
            reader:      CharReader::new(code),
            span:        Rc::new(Span::new(code)),
            prev:        None,
        }
    }

    /* advances the next program node (a fn def or variable def) */
    pub fn advance_prog() -> Result<VecDeque<Token>, LexerError<'a>> {
        Ok(VecDeque::<Token>::new())
    }

    fn advance_line(&mut self) -> Result<(VecDeque<Token>, usize), ChalError> {
        if self.reader.is_empty() {
            return Err(ChalError::from( InternalError::new("Lexer::advance_line(): advancing an empty lexer") ));
        }

        let start = self.reader.pos();
        let indent = self.reader.advance_while(|c: char| c == ' ');
        let indent_size = indent.len();

        if indent_size % 4 != 0 {
            return Err(ChalError::from( LexerError::invalid_indentation(start, self.reader.pos(), &self.span) ));
        }
        
        Ok( (VecDeque::<Token>::new(), 0) )
    }

    fn compare_delim(&mut self, &tok) -> Result<(), ChalError<'a>>{
        
    }

    fn advance_tok(&mut self, src: String, start: &Position, end: &Position) -> Result<Token, ChalError<'a>>{
        /* 1. create the token
         * 2. update the prev token
         * 3. update the delimiter stack
         * 4. check for delimiter errors
         */
        let tok = Token::new(src, start, end, self.span.as_ref())?;

        self.prev = Some(&tok);

        match tok.kind() {
            TokenKind::Delimiter(Delimiter::OpenPar) 
                | TokenKind::Delimiter(Delimiter::OpenBrace)
                | TokenKind::Delimiter(Delimiter::OpenBracket) 
            => self.delim_stack.push_back(&tok),

            TokenKind::Delimiter(Delimiter::ClosePar) => {
                if self.delim_stack.back() == None {
                    return Err(
                        ChalError::from(
                            LexerError::unexpected_closing_delimiter(tok.src(), start, end, self.span.as_ref())
                            )
                        );
                }

                let open_del = *self.delim_stack.back().unwrap();

                if *open_del.kind() != TokenKind::Delimiter(Delimiter::OpenPar) {
                    return Err(
                        ChalError::from( 
                            LexerError::missmatching_delimiters(open_del.src(), tok.src(), start, end, self.span.as_ref()) 
                        )
                    );
                }

                self.delim_stack.pop_back();     
            },
            
            _ => (),
        };

        Ok(tok)
    }
    
    fn advance(&mut self) -> Result<Token, ChalError<'a>> {
        let start = self.reader.pos();
        let current: char;
        if let Some(ch) = self.reader.advance() {
            current = ch;
        } else {
            return Err(InternalError::new("Lexer::advance(): advancing an empty lexer"));

        }

        if current.is_alphanumeric() {
            let src = String::from(current) + &self.reader.advance_while(|c| c.is_alphanumeric() || c == '_'); 
            return Token::new(src, &start, self.reader.pos());
        }

        if current.is_numeric() || 
           (current == '-' && 
            self.reader.peek().is_some() && 
            self.reader.peek().unwrap().is_numeric()
            )
        {
            /* 
             * check weather the minus should be interpreted as
             * a negative int or an operator, example:
             * 'a-5' -> identifier(a), sub(-), uint(5)
             * 'a*-5' -> identifier(a), mul(*), int(-5)
             */
            if current == '-' {
                match self.prev {
                    Some(token) => {
                        if token.is_terminal() { 
                            return Token::new(current.to_string(), &start, self.reader.pos());
                        }
                    },
                    None => (),
                }
            }

            let src = String::from(current) + &self.reader.advance_while(|c| c.is_numeric() || c == '.');
            return Token::new(src, &start, self.reader.pos());
        }

        if is_special(&current) {
            let mut end = start.clone();
            end.advance_col();

            if !is_operator(&current) ||  self.reader.peek() == None {
                return Token::new(current.to_string(), &start, &end);
            }

            let mut buffer = String::from(current);
            if let Some(c) = self.reader.peek() { buffer.push(c.clone()) }

            match buffer.as_str() {
                "+=" | "-=" | "*=" | "/=" | "%=" | "==" | 
                "!=" | "<=" | ">=" | "->" | ":=" => {
                    self.reader.advance();
                    end.advance_col();
                },
                _ => _ = buffer.pop(),
            }
            return Token::new(buffer, &start, &end);
        }
        
        if current == '\n' {
            return Token::new(String::from(current), &start, self.reader.pos());
        }

        if current == '"' {
            let mut src = String::from(current) + &self.reader.advance_while(|c| c != '"' ); 
            if let Some(c) = self.reader.advance() { src.push(c); }  // adds the '"' at the end

            return Token::new(src, &start, self.reader.pos());
        }

        return Err(InternalError::new("Lexer::advance(): could not parse token"));
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

        for token in self.tokens.clone() {
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

        for token in &self.tokens {
            if let Err(_) = token.err_msg(span) {
                error = true;
            }
        }

        if error { return Err(()); }
        Ok(())
    }

    // returns the next program element => var def/declr or function def
    pub fn advance_program(&mut self, span: &Span) -> Option<VecDeque<Token>> {
        let mut result = VecDeque::new(); 
    
        if let Some(token) = self.tokens.front() {
            match token.get_kind() {
                TokenKind::Keyword(Keyword::Let) => {
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

                TokenKind::Newline => {
                    self.advance();
                    return None;
                },

                _ => {
                    LexerErrors::InvalidGlobalStatement::msg(token.start(), token.end(), span, token.get_kind());
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

    /*
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
    */
}
