
use std::collections::VecDeque;

use crate::parser::ast::{NodeVarDef, NodeFuncDef};
use crate::errors::span::Span;
use crate::lexer::{Token, TokenKind, Keyword};

// program node
#[derive(Debug)]
pub enum NodeProg {
    VarDef(NodeVarDef),
    FuncDef(NodeFuncDef),
}

impl NodeProg {
    pub fn new(tokens: VecDeque<Token>, span: &Span) -> Result<NodeProg, ()> {
        match tokens.front() {
            Some(tok) => match tok.get_kind() {
                TokenKind::Keyword(Keyword::Let) => return Ok(NodeProg::VarDef(NodeVarDef::new(tokens, span)?)),
                TokenKind::Keyword(Keyword::Fn)  => return Ok(NodeProg::FuncDef(NodeFuncDef::new(tokens, span)?)),
                _ => return Err(()),
            },
            None => return Err(()),
        }
    }
}
