pub mod expr;
pub mod func;
mod operators;
pub mod program;
pub mod stmnt;
pub mod var;

pub use expr::NodeExpr;
pub use func::{NodeFuncCall, NodeFuncDef};
pub use program::NodeProg;
pub use stmnt::{parse_body, NodeStmnt};
pub use var::{NodeVarCall, NodeVarDef};

use operators::*;

#[derive(Debug)]
pub enum NodeValue {
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(String),
    // TODO: add custom values - structs
}
