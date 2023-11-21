use crate::parser::ast::{NodeExpr, NodeStmnt};

pub struct NodeWhileLoop {
    /* TODO: change to using a proper condition */
    condition: NodeExpr,
    body: Vec<NodeStmnt>,
}
