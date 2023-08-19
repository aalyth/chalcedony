
use crate::parser::ast::operators::UnaryOprType;
use crate::parser::ast::NodeExpr;

#[derive(Debug)]
pub struct NodeUnaryExpr {
    operand: Box<NodeExpr>,
    operator: UnaryOprType,
}
