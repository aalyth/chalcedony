//! The Chalcedony `Abstract Syntax Tree (AST)` - the core Intermediate
//! Representation of the source code, understandable by the the interpreter.
//!
//! The implementation of the parser is a handwritten parser with a lookup of 2.
//! For a detailed explanation of each node's syntax refer to the corresponding
//! node's structure definition.

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
    NodeIfStmnt, NodeRetStmnt, NodeStmnt, NodeThrow, NodeTryCatch, NodeWhileLoop,
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
