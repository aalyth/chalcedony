mod expr;
mod func;
mod stmnt;
mod var;

use crate::error::ChalError;
use crate::lexer::Type;
use crate::parser::ast::{NodeProg, NodeVarDef};

use std::collections::{HashMap, HashSet};

use super::FuncAnnotation;

pub trait ToBytecode {
    fn to_bytecode(
        self,
        func_symtable: &mut HashMap<String, FuncAnnotation>,
        local_vars: Option<&mut HashSet<String>>,
    ) -> Result<Vec<u8>, ChalError>;
}

impl ToBytecode for NodeProg {
    fn to_bytecode(
        self,
        func_symtable: &mut HashMap<String, FuncAnnotation>,
        _: Option<&mut HashSet<String>>,
    ) -> Result<Vec<u8>, ChalError> {
        match self {
            NodeProg::VarDef(node) => node.to_bytecode(func_symtable, None),
            NodeProg::FuncDef(node) => node.to_bytecode(func_symtable, None),
        }
    }
}
