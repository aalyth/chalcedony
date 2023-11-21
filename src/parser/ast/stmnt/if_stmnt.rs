use crate::parser::ast::{NodeExpr, NodeStmnt};

pub struct NodeIfStmnt {
    /* TODO: change to using a proper condition */
    condition: NodeExpr,
    body: Vec<NodeStmnt>,
}
