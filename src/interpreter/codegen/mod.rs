pub mod expr;
pub mod func;
pub mod stmnt;
pub mod var;

use crate::error::ChalError;
use crate::parser::ast::NodeProg;
use crate::utils::Bytecode;

use super::Chalcedony;

pub trait ToBytecode {
    fn to_bytecode(self, _: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError>;
}

impl ToBytecode for NodeProg {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        match self {
            NodeProg::VarDef(node) => node.to_bytecode(interpreter),
            NodeProg::FuncDef(node) => node.to_bytecode(interpreter),
            NodeProg::FuncCall(node) => node.to_bytecode(interpreter),
            NodeProg::Assign(node) => node.to_bytecode(interpreter),
            NodeProg::IfStmnt(node) => node.to_bytecode(interpreter),
            NodeProg::WhileLoop(node) => node.to_bytecode(interpreter),
        }
    }
}
