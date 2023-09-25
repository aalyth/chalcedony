use crate::parser::ast::{NodeVarDef, NodeFuncDef};
use crate::error::{ChalError, InternalError};
use crate::lexer::{Line, TokenKind, Keyword};

use std::collections::VecDeque;

#[derive(Debug)]
pub enum NodeProg {
    VarDef  (NodeVarDef),
    FuncDef (NodeFuncDef),
}

impl NodeProg {
    pub fn new(chunk: VecDeque<Line>) -> Result<NodeProg, ChalError> {
        if chunk.is_empty() {
            return Err(ChalError::from( InternalError::new("NodeProg::new(): received an empty code chunk") ));
        }
        
        let front_line = chunk.front().unwrap();
        if front_line.tokens().is_empty() {
            return Err(ChalError::from( InternalError::new("NodeProg::new(): empty first line of chunk") ));
        }

        let front_tok = front_line.tokens().front().unwrap();

        match front_tok.kind() {
            TokenKind::Keyword(Keyword::Let) => return NodeProg::VarDef(NodeVarDef::new(chunk)),
            TokenKind::Keyword(Keyword::Fn)  => return NodeProg::FuncDef(NodeFuncDef::new(chunk)),
            
            _ => return Err(ChalError::from( InternalError::new("NodeProg::new(): invalid chunk front") )),
        }
    }
}
