mod expr;
mod func;
mod program;
mod stmnt;
mod var;

pub use expr::{NodeExpr, NodeExprInner};
pub use func::{NodeFuncCall, NodeFuncDef};
pub use program::NodeProg;
pub use stmnt::{
    NodeAssign, NodeElifStmnt, NodeElseStmnt, NodeIfBranch, NodeIfStmnt, NodeRetStmnt, NodeStmnt,
    NodeWhileLoop,
};
pub use var::{NodeVarCall, NodeVarDef};

#[derive(Clone)]
pub enum NodeValue {
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(String),
    Bool(bool),
}
