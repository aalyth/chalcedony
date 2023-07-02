pub mod ast;

use crate::errors::{span::Span, ParserErrors};
use crate::lexer::Lexer;
use crate::lexer::tokens::{TokenKind, Keyword};
use ast::*;
use ast::func::NodeFuncDef;

pub struct Parser {
    // fields to contain variable and function definitions
    next: Vec<NodeProg> 
}

impl Parser {
    pub fn new(code: &str) -> Result<(Parser, Span), ()>{
        let mut result = Parser {
            next: Vec::<NodeProg>::new(),
        };

        result.generate(code)
    }

    fn generate(&mut self, code: &str) -> Result<(Parser, Span), ()> {
        let (mut lexer, span) = Lexer::new(code)?;

        while !lexer.is_empty() {
            let current = lexer.advance().unwrap();

            match current.get_kind() {
                TokenKind::Keyword(Keyword::Let) => {
                    let mut tokens = lexer.advance_while(|tk| *tk != TokenKind::Newline && 
                                                              *tk != TokenKind::SemiColon);
                    self.next.push(NodeProg::VarDef(NodeVarDef::new(tokens, &span)?));
                },

                TokenKind::Keyword(Keyword::Fn) => {
                    let mut tokens = lexer.advance_while(|tk| *tk != TokenKind::CloseBrace);
                    tokens.push_back(lexer.advance().unwrap()); // the '}' delimiter
                    self.next.push(NodeProg::FuncDef(NodeFuncDef::new(&lexer, &span)?));
                },

                TokenKind::Newline => (),
                TokenKind::SemiColon => (),
                _ => {
                    // !TODO invalid/unexpected program token
                    ParserErrors::UnexpectedToken::msg(&current, &span);
                    return Err(());
                }
            }
        }
        Ok(())
    }
}
