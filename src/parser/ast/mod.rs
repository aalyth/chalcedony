mod operators;
pub mod func;

use operators::*;
use crate::errors::parser::*;
use func::*;

use crate::lexer::tokens::*;

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
    pub fn new(tk: &TokenKind) -> Option<Self> {
        let mut result = VarType::Str;
        match tk {
            TokenKind::Keyword(Keyword::I8)  => result = VarType::I8,  
            TokenKind::Keyword(Keyword::I16) => result = VarType::I16,  
            TokenKind::Keyword(Keyword::I32) => result = VarType::I32,  
            TokenKind::Keyword(Keyword::I64) => result = VarType::I64,  

            TokenKind::Keyword(Keyword::U8)  => result = VarType::U8,  
            TokenKind::Keyword(Keyword::U16) => result = VarType::U16,  
            TokenKind::Keyword(Keyword::U32) => result = VarType::U32,  
            TokenKind::Keyword(Keyword::U64) => result = VarType::U64,  

            TokenKind::Keyword(Keyword::F32) => result = VarType::F32,  
            TokenKind::Keyword(Keyword::F64) => result = VarType::F64,  

            TokenKind::Str(_) => (), // result is already string
        
            _ => return None,
        }
        
        Some(result)
    }
}

pub enum NodeValue {
    Int(i64),
    UInt(u64),
    Float(f64),
    Str(String),
}

pub struct NodeVarCall {
    name: String,
}

pub struct NodeVarDef {
    r#type: VarType,
    name:   String,
    value:  Option<NodeExpr>
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

// program node
pub enum NodeProg {
    VarDef(NodeVarDef),
    FuncDef(NodeFuncDef),
}

