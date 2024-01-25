pub mod expr;
pub mod func;
pub mod operators;
pub mod program;
pub mod stmnt;
pub mod var;

pub use expr::{NodeExpr, NodeExprInner};
pub use func::{NodeFuncCall, NodeFuncDef};
pub use program::NodeProg;
pub use stmnt::{
    parse_body, NodeAssign, NodeElifStmnt, NodeElseStmnt, NodeIfStmnt, NodeRetStmnt, NodeStmnt,
    NodeWhileLoop,
};
pub use var::{NodeVarCall, NodeVarDef};

pub enum NodeValue {
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(String),
    Bool(bool),
}
