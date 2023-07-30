
use crate::parser::ast::{NodeExpr, operators::BinOprType};

pub struct NodeBinExpr {
    left: Box<NodeExpr>,
    right: Box<NodeExpr>,
    operator: BinOprType,
}
