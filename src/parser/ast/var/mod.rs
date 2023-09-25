pub mod def;

pub use def::NodeVarDef;

use crate::parser::ast::{VarType, NodeExpr};
use crate::lexer::{Token, TokenKind, Keyword, Type};

// use crate::errors::{ParserErrors, span::Span};
// use crate::parser::TokenReader;

use std::collections::VecDeque;

pub struct NodeVarCall {
    name: String,
}

impl NodeVarCall {
    pub fn new(token: &Token) -> Option<NodeVarCall> {
        if let TokenKind::Identifier(val) = token.get_kind() {
            return Some( NodeVarCall { name: val.clone() });
        }
        None
    }
}

