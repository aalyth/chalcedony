use crate::error::{ChalError, InternalError, Span};
use crate::lexer::{Keyword, Line, Token, TokenKind};
use crate::parser::ast::{NodeFuncDef, NodeVarDef};

use crate::parser::TokenReader;

use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
pub enum NodeProg {
    VarDef(NodeVarDef),
    FuncDef(NodeFuncDef),
}

impl NodeProg {
    pub fn new(chunk: VecDeque<Line>, span: Rc<Span>) -> Result<Self, ChalError> {
        if chunk.is_empty() {
            return Err(ChalError::from(InternalError::new(
                "NodeProg::new(): received an empty code chunk",
            )));
        }

        let front_line = chunk.front().unwrap();
        if front_line.tokens().is_empty() {
            return Err(ChalError::from(InternalError::new(
                "NodeProg::new(): empty first line of chunk",
            )));
        }

        let front_tok = front_line.tokens().front().unwrap();

        match front_tok.kind() {
            TokenKind::Keyword(Keyword::Let) => NodeProg::var_def(front_line.tokens(), span),
            TokenKind::Keyword(Keyword::Fn) => NodeProg::func_def(chunk, span),

            _ => {
                return Err(ChalError::from(InternalError::new(
                    "NodeProg::new(): invalid chunk front",
                )))
            }
        }
    }

    #[inline]
    fn var_def(chunk: &VecDeque<Token>, span: Rc<Span>) -> Result<Self, ChalError> {
        Ok(Self::VarDef(NodeVarDef::new(TokenReader::new(
            chunk, span,
        ))?))
    }

    #[inline]
    fn func_def(chunk: VecDeque<Line>, span: Rc<Span>) -> Result<Self, ChalError> {
        Ok(Self::FuncDef(NodeFuncDef::new(chunk, span.clone())?))
    }
}
