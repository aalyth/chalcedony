
use crate::parser::ast::operators::UnaryOprType;
use crate::parser::ast::NodeExpr;

pub struct NodeUnaryExpr {
    operand: Box<NodeExpr>,
    operator: UnaryOprType,
}
