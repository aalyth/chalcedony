mod expr;
mod func;
mod stmnt;
mod var;

use crate::error::ChalError;
use crate::parser::ast::NodeProg;

use std::collections::HashMap;

use stmnt::stmnt_to_bytecode;

use super::FuncAnnotation;

pub trait ToBytecode {
    fn to_bytecode(
        self,
        func_symtable: &mut HashMap<String, FuncAnnotation>,
    ) -> Result<Vec<u8>, ChalError>;
}

impl ToBytecode for NodeProg {
    fn to_bytecode(
        self,
        func_symtable: &mut HashMap<String, FuncAnnotation>,
    ) -> Result<Vec<u8>, ChalError> {
        match self {
            NodeProg::VarDef(node) => node.to_bytecode(func_symtable),
            NodeProg::FuncDef(node) => node.to_bytecode(func_symtable),
        }
    }
}
