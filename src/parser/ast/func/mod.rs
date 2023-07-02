use std::collections::VecDeque;

use crate::parser::ast::*;
use crate::lexer::{Lexer, tokens::*};
use crate::errors::{span::*,
                    span::pos::*,
                    parser::*};

pub struct NodeFuncDef {
    name:     String,
    args:     Vec<(String, VarType)>,
    ret_type: Option<VarType>,
    body:     Vec<NodeStmnt>
}

pub struct NodeFuncCall {
    name: String,
    args: Vec<NodeExpr>,
}

impl NodeFuncDef {
    // TODO! implement a proper TokenReader instead of using the lexer (the lexer 
    // should be for getting the function body and the TokenReader for traversing it)
    pub fn new(lexer: &Lexer, span: &Span) -> Result<NodeFuncDef, ()>{
        let mut res = NodeFuncDef {
            name: String::new(),
            args: Vec::<(String, VarType)>::new(),
            ret_type: None,
            body: Vec::<NodeStmnt>::new(),
        };

        lexer.expect(span, TokenKind::Keyword(Keyword::Fn))?;
        if let TokenKind::Identifier(func_name) = lexer.expect(span, TokenKind::Identifier(String::new()))?.get_kind() {
            res.name = func_name.clone();

        } else {
            // the lexer.expect() has already printed an error
            return Err(());
        }

        res.scrape_args(lexer, span)?;

        Ok(res)
    }

    fn scrape_args(&mut self, lexer: &Lexer, span: &Span) -> Result<(), ()> {
         
        Ok(())
    }

}
