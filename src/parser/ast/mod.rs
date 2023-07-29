mod operators;
pub mod func;
pub mod var;
pub mod program;

pub use var::{NodeVarCall, NodeVarDef};
pub use func::NodeFuncDef;
pub use program::NodeProg;

use operators::*;

use crate::lexer::tokens::*;

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
    // !TODO Custom(String), // an user-defined type (ex. struct or enum)
}

impl VarType {
    pub fn new(token: Token) -> Option<VarType> {
        let mut result = VarType::Str;
        match token.get_kind() {
            TokenKind::Type(Type::I8)  => result = VarType::I8,  
            TokenKind::Type(Type::I16) => result = VarType::I16,  
            TokenKind::Type(Type::I32) => result = VarType::I32,  
            TokenKind::Type(Type::I64) => result = VarType::I64,  

            TokenKind::Type(Type::U8)  => result = VarType::U8,  
            TokenKind::Type(Type::U16) => result = VarType::U16,  
            TokenKind::Type(Type::U32) => result = VarType::U32,  
            TokenKind::Type(Type::U64) => result = VarType::U64,  

            TokenKind::Type(Type::F32) => result = VarType::F32,  
            TokenKind::Type(Type::F64) => result = VarType::F64,  

            TokenKind::Type(Type::Str) => (), // type is str by default
        
            _ => {
                eprintln!("Error: VarType: new(): could not convert type.");
                return None;
            },
        }
        
        Some(result)
    }
}


pub enum NodeValue {
    Int(i64),
    UInt(u64),
    Float(f64),
    Str(String),
    // add custom values - structs
}

pub struct NodeBinExpr {
    left: Box<NodeExpr>,
    right: Box<NodeExpr>,
    operator: BinOprType,
}

pub struct NodeUnaryExpr {
    operand: Box<NodeExpr>,
    operator: UnaryOprType,
}

pub enum NodeExpr {
    BinExpr(NodeBinExpr),
    UnaryExpr(NodeUnaryExpr),
    Value(NodeValue),
}

pub struct NodeUnaryCond {
    operand:  Box<NodeCond>,
    operator: UnaryCondType,
}

pub struct NodeBinCond {
    left:     Box<NodeCond>,
    right:    Box<NodeCond>,
    operator: BinCondType,
}

pub enum NodeCond {
    BinCond(NodeBinCond),
    UnaryCond(NodeUnaryCond),
    Value(NodeValue),
}

pub struct NodeIfStmnt {
    condition: NodeExpr,
    body: Vec<NodeStmnt>
}

pub struct NodeWhileLoop {
    condition: NodeExpr,
    body: Vec<NodeStmnt>
}

pub enum NodeStmnt {
   Expr(NodeExpr),
   IfStmnt(NodeIfStmnt),
   WhileLoop(NodeWhileLoop),
   RetStmnt(NodeRetStmnt),
}

pub struct NodeRetStmnt {
    value: NodeExpr,
}


