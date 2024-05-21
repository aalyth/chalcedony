//! The Chalcedony `Abstract Syntax Tree (AST)` - the core Intermediate
//! Representation of the source code, understandable by the the interpreter.
//!
//! The implementation of the parser is a handwritten parser with a lookup of 2.
//! For a detailed explanation of each node's syntax refer to the corresponding
//! node's structure definition.

pub mod class;
mod expr;
pub mod func;
mod program;
mod stmnt;
mod var;

pub use class::{NodeAttrRes, NodeAttribute, NodeClass};
pub use expr::{NodeExpr, NodeExprInner, NodeInlineClass};
pub use func::{NodeFuncCall, NodeFuncCallStmnt, NodeFuncDef};
pub use program::{NodeImport, NodeProg};
pub use stmnt::{
    NodeAssign, NodeBreakStmnt, NodeContStmnt, NodeElifStmnt, NodeElseStmnt, NodeForLoop,
    NodeIfBranch, NodeIfStmnt, NodeRetStmnt, NodeStmnt, NodeThrow, NodeTryCatch, NodeWhileLoop,
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
