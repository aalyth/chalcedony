
use crate::parser::ast::{NodeExpr, operators::BinOprType};

#[derive(Debug)]
pub struct NodeBinExpr {
    left: Box<NodeExpr>,
    right: Box<NodeExpr>,
    operator: BinOprType,
}
