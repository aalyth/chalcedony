
use crate::lexer::Token;
use crate::parser::ast::{NodeBinExpr, NodeUnaryExpr, NodeValue};

use std::collections::VecDeque;

pub enum NodeExpr {
    BinExpr(NodeBinExpr),
    UnaryExpr(NodeUnaryExpr),
    Value(NodeValue),
}

impl NodeExpr {
    pub fn new(tokens: VecDeque<Token>) -> Option<NodeExpr> {
        // 1. get the needed tokens from the stream
        // 2. identify the unary expressions
        // 3. start parsing from the lowest precedence binary operators
        // 4. works???

        None
    }
}
