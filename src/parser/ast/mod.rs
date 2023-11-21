pub mod expr;
pub mod func;
mod operators;
pub mod program;
pub mod stmnt;
pub mod var;

pub use expr::NodeExpr;
pub use func::{NodeFuncCall, NodeFuncDef};
pub use program::NodeProg;
pub use stmnt::NodeStmnt;
pub use var::{NodeVarCall, NodeVarDef};

use operators::*;

/*
#[derive(Debug)]
pub enum VarType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Str,
    // TODO: Custom(String), // an user-defined type (ex. struct or enum)
}

impl VarType {
    pub fn new(token: Token) -> Option<VarType> {
        let mut result = VarType::Str;
        match token.kind() {
            TokenKind::Type(Type::I8) => result = VarType::I8,
            TokenKind::Type(Type::I16) => result = VarType::I16,
            TokenKind::Type(Type::I32) => result = VarType::I32,
            TokenKind::Type(Type::I64) => result = VarType::I64,

            TokenKind::Type(Type::U8) => result = VarType::U8,
            TokenKind::Type(Type::U16) => result = VarType::U16,
            TokenKind::Type(Type::U32) => result = VarType::U32,
            TokenKind::Type(Type::U64) => result = VarType::U64,

            TokenKind::Type(Type::F32) => result = VarType::F32,
            TokenKind::Type(Type::F64) => result = VarType::F64,

            TokenKind::Type(Type::Str) => (), // type is str by default

            _ => {
                eprintln!("Error: VarType: new(): could not convert type.");
                return None;
            }
        }

        Some(result)
    }
}
*/

#[derive(Debug)]
pub enum NodeValue {
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(String),
    // TODO: add custom values - structs
}
