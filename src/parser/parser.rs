use crate::errors::{span::Span, ParserErrors};
use crate::lexer::Lexer;
use crate::lexer::{TokenKind, Keyword};
use crate::parser::ast::*;
// use crate::parser::ast::func::NodeFuncDef;

use std::collections::VecDeque;

use super::ast::func::NodeFuncDef;

pub struct Parser {
    // fields to contain variable and function definitions
    nodes: VecDeque<NodeProg> 
}

impl Parser {
    pub fn new(code: &str) -> Result<(Parser, Span), ()>{
        let mut result = Parser {
            nodes: VecDeque::<NodeProg>::new(),
        };

        let span = result.generate(code)?;
        
        Ok( (result, span) )
    }

    fn generate(&mut self, code: &str) -> Result<Span, ()> {
        let (mut lexer, span) = Lexer::new(code)?;

        while !lexer.is_empty() {
            if let Some(tokens) = lexer.advance_program() {
                match tokens.front() {
                    Some(tok) => match tok.get_kind() {
                        TokenKind::Keyword(Keyword::Let) => self.nodes.push_back(NodeVarDef::new(tokens)),
                        TokenKind::Keyword(Keyword::Fn)  => self.nodes.push_back(NodeFuncDef::new(tokens)),
                        _ => return Err(()),
                    },
                    None => return Err(()),
                }

            } else  {
                return Err(());
            }
        }

        /*
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
        */
        Ok(span)
    }
}
