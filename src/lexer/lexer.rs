use crate::lexer::tokens::{Token, 
             TokenKind,
             Keyword,
             Delimiter,
             is_special, 
             is_operator,
             };

use crate::lexer::line::Line;

use crate::error::{ChalError,
                   LexerError,
                   InternalError,
                   span::Span,
                   span::Position,
                   };

use crate::lexer::CharReader;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Lexer {  
    /* contains the opening delimiters */
    #[allow(dead_code)]
    delim_stack: VecDeque<Token>,
    reader:      CharReader,
    span:        Rc<Span>,
    /* contains a clone of the previous token kind */
    prev:        Option<TokenKind>,
}

impl Lexer {
    pub fn new(code: &str) -> Self {
        /* convert tabs to 4 spaces */
        let src = str::replace(code, "\t", "    ");

        Lexer {
            delim_stack: VecDeque::<Token>::new(),
            reader:      CharReader::new(src),
            span:        Rc::new(Span::new(code)),
            prev:        None,
        }
    }

    /* advances the next program node (a fn def or variable def) */
    pub fn advance_prog(&mut self) -> Result<VecDeque<Line>, ChalError> {
        if self.reader.is_empty() {
            return Err(ChalError::from( InternalError::new("Lexer::advance_prog(): advancing an empty lexer") ));
        }

        let mut result = VecDeque::<Line>::new();
        let mut current = self.advance_line()?; 

        while current.tokens().len() < 2 {
            if current.tokens().is_empty() {
                return Err(ChalError::from( InternalError::new("Lexer::advance_prog(): received empty line") ));
            }

            if current.tokens().len() == 1 {
                current = self.advance_line()?;
            }
        }

        let front = current.tokens().front().unwrap().clone();

        result.push_back(current);

        match front.kind() {
            TokenKind::Keyword(Keyword::Let) => return Ok(result),
            
            TokenKind::Keyword(Keyword::Fn) => {
                let mut errors = VecDeque::<ChalError>::new();

                while let Some(' ') = self.reader.peek() {
                    let current = self.advance_line();
                    match current {
                        Ok(line) => {
                            if line.tokens().len() > 1 { 
                                result.push_back(line);
                            }
                        }
                        Err(mut err_chunk) => errors.append(&mut err_chunk),
                    }
                }
                
                if !errors.is_empty() { return Err( ChalError::from(errors) ); }
                return Ok(result);
            }

            invalid @ _ 
            => return Err(
                ChalError::from(
                        LexerError::invalid_global_statement(invalid.clone(), *front.start(), *front.end(), Rc::clone(&self.span))
                    )
                ),
        }
    }

    fn advance_line(&mut self) -> Result<Line, VecDeque<ChalError>> {
        if self.reader.is_empty() {
            return Err(
                VecDeque::from([ChalError::from(
                    InternalError::new("Lexer::advance_line(): advancing an empty lexer")
                )])
            );
        }

        let start = *self.reader.pos();
        let indent_raw = self.reader.advance_while(|c: char| c == ' ');
        let indent = indent_raw.len();

        let mut errors = VecDeque::<ChalError>::new();

        if indent % 4 != 0 {
            errors.push_back(
                ChalError::from(
                    LexerError::invalid_indentation(start.clone(), *self.reader.pos(), Rc::clone(&self.span))
                )
            );
        }

        let mut result = VecDeque::<Token>::new();
        let mut current = self.advance();

        loop {
            match current {
                Ok(tok)  => result.push_back(tok),
                Err(err) => errors.push_back(err),
            }
            /* this clunky check is so we don't have problems with the borrow checker*/
            if result.back().is_some() || *result.back().unwrap().kind() == TokenKind::Newline { break; }
            current = self.advance();
        }
        
        if !errors.is_empty() { return Err( errors ); }
        Ok(Line::new(indent/4, result))
    }

    fn advance_tok(&mut self, src: String, start: Position, end: Position) -> Result<Token, ChalError>{
        /* 1. create the token
         * 2. update the prev token
         * 3. update the delimiter stack
         * 4. check for delimiter errors
         */
        let tok = Token::new(src, start, end, &self.span)?;

        self.prev = Some(tok.kind().clone());

        match tok.kind() {
            TokenKind::Delimiter(Delimiter::OpenPar) 
                | TokenKind::Delimiter(Delimiter::OpenBrace)
                | TokenKind::Delimiter(Delimiter::OpenBracket) 
            => self.delim_stack.push_back(tok.clone()),

            /* only closing delimiters match here */
            TokenKind::Delimiter(close_delim) => {
                if self.delim_stack.back() == None {
                    return Err(
                        ChalError::from(
                            LexerError::unexpected_closing_delimiter(tok.src(), start, end, Rc::clone(&self.span))
                        )
                    );
                }

                let open_delim = self.delim_stack.pop_back().unwrap();

                if *open_delim.kind() != TokenKind::Delimiter(close_delim.inverse()) {
                    return Err(
                        ChalError::from( 
                            LexerError::missmatching_delimiters(open_delim.src(), tok.src(), start, end, Rc::clone(&self.span)) 
                        )
                    );
                }
            },
            
            _ => (),
        };

        Ok(tok)
    }
    
    fn advance(&mut self) -> Result<Token, ChalError> {
        let start = *self.reader.pos();

        /* this way a do-while behaviour is achieved */
        let mut current: char = ' ';
        while current == ' ' {
            let Some(current_) = self.reader.advance() else {
                return Err(
                    ChalError::from(
                        InternalError::new("Lexer::advance(): advancing an empty lexer")
                    )
                );
            };
            current = current_;
        };

        if current == '#' {
            let _ = self.reader.advance_while(|c: char| c != '\n');
            self.reader.advance(); /* remove the \n if there's any */
            return self.advance_tok(String::from("\n"), *self.reader.pos(), *self.reader.pos());
        }

        if current.is_alphanumeric() {
            let src = String::from(current) + &self.reader.advance_while(|c| c.is_alphanumeric() || c == '_'); 
            return self.advance_tok(src, start, *self.reader.pos());
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
                match &self.prev {
                    Some(kind) => {
                        if kind.is_terminal() { 
                            return self.advance_tok(current.to_string(), start, *self.reader.pos());
                        }
                    },
                    None => (),
                }
            }

            let src = String::from(current) + &self.reader.advance_while(|c| c.is_numeric() || c == '.');
            return self.advance_tok(src, start, *self.reader.pos());
        }

        if is_special(&current) {
            let mut end = start.clone();
            end.advance_col();

            if !is_operator(&current) ||  self.reader.peek() == None {
                return self.advance_tok(current.to_string(), start, end);
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
            return self.advance_tok(buffer, start, end);
        }
        
        if current == '\n' {
            return self.advance_tok(String::from(current), start, *self.reader.pos());
        }

        if current == '"' {
            let mut src = String::from(current) + &self.reader.advance_while(|c| c != '"' ); 
            if let Some(c) = self.reader.advance() { src.push(c); }  // adds the '"' at the end

            return self.advance_tok(src, start, *self.reader.pos());
        }


        return Err(
            ChalError::from(
                InternalError::new("Lexer::advance(): could not parse token")
            )
        );
    }

    // returns the next program element => var def/declr or function def
    /*
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
    */

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
        self.reader.is_empty()
    }
}
