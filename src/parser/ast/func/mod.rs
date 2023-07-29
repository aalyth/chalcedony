use std::collections::VecDeque;

use crate::parser::{ast::*, TokenReader};
use crate::lexer::{Lexer, tokens::*};
use crate::errors::{span::*,
                    span::pos::*,
                    parser::*};

#[derive(Debug)]
pub struct NodeFuncDef {
    name:     String,
    args:     Vec<(String, VarType)>,
    ret_type: Option<VarType>,
    // body:     Vec<NodeStmnt>
}

pub struct NodeFuncCall {
    name: String,
    args: Vec<NodeExpr>,
}

impl NodeFuncDef {
    // TODO! implement a proper TokenReader instead of using the lexer (the lexer 
    // should be for getting the function body and the TokenReader for traversing it)
    pub fn new(tokens: VecDeque<Token>, span: &Span) -> Result<NodeFuncDef, ()>{
        let mut res = NodeFuncDef {
            name: String::new(),
            args: Vec::<(String, VarType)>::new(),
            ret_type: None,
            // body: Vec::<NodeStmnt>::new(),
        };

        let mut reader = TokenReader::new(tokens, span);

        reader.expect(TokenKind::Keyword(Keyword::Fn))?;
        if let TokenKind::Identifier(func_name) = reader.expect(TokenKind::Identifier(String::new()))?.get_kind() {
            res.name = func_name.clone();

        } else {
            // the lexer.expect() has already printed an error
            return Err(());
        }

        reader.expect(TokenKind::OpenPar)?;
        res.scrape_args(&mut reader)?;

        Ok(res)
        //Err(())
    }

    fn scrape_args(&mut self, reader: &mut TokenReader) -> Result<(), ()> {
        while !reader.is_empty() {
            match reader.peek().unwrap().get_kind() {
                TokenKind::ClosePar => return Ok(()),

                _ => {
                    let arg_name_tok = reader.expect(TokenKind::Identifier(String::new()))?;
                    let mut arg_name = String::new();

                    if let TokenKind::Identifier(name) = arg_name_tok.get_kind() {
                        arg_name = name.clone();
                    }

                    reader.expect(TokenKind::Colon)?;

                    let arg_type_tok = reader.expect(TokenKind::Type(Type::Any))?;
                    let arg_type = VarType::new(arg_type_tok).unwrap(); // this should never error

                    self.args.push((arg_name, arg_type));

                    // TODO! check if the next token is , or )
                    if let Some(token) = reader.peek() {
                        if *token.get_kind() != TokenKind::ClosePar {
                            reader.expect(TokenKind::Comma)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

}
