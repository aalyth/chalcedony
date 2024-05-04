mod expr;
pub mod func;
mod program;
mod stmnt;
mod var;

pub use expr::{NodeExpr, NodeExprInner};
pub use func::{NodeFuncCall, NodeFuncDef};
pub use program::NodeProg;
pub use stmnt::{
    NodeAssign, NodeBreakStmnt, NodeContStmnt, NodeElifStmnt, NodeElseStmnt, NodeIfBranch,
    NodeIfStmnt, NodeRetStmnt, NodeStmnt, NodeWhileLoop,
};
pub use var::{NodeVarCall, NodeVarDef};

/// The node representing a literal value inside the source code.
#[derive(Clone, Debug, PartialEq)]
pub enum NodeValue {
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(String),
    Bool(bool),
}
