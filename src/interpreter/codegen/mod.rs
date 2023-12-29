mod expr;
mod func;
mod stmnt;
mod var;

use crate::error::ChalError;
use crate::parser::ast::NodeProg;

use std::collections::BTreeMap;

use stmnt::stmnt_to_bytecode;

use super::FuncAnnotation;

pub trait ToBytecode {
    fn to_bytecode(
        self,
        bytecode_len: usize,
        func_symtable: &mut BTreeMap<String, FuncAnnotation>,
        func_lookup: &mut BTreeMap<String, u64>,
    ) -> Result<Vec<u8>, ChalError>;
}

impl ToBytecode for NodeProg {
    fn to_bytecode(
        self,
        bytecode_len: usize,
        func_symtable: &mut BTreeMap<String, FuncAnnotation>,
        func_lookup: &mut BTreeMap<String, u64>,
    ) -> Result<Vec<u8>, ChalError> {
        match self {
            NodeProg::VarDef(node) => node.to_bytecode(bytecode_len, func_symtable, func_lookup),
            NodeProg::FuncDef(node) => node.to_bytecode(bytecode_len, func_symtable, func_lookup),
        }
    }
}
