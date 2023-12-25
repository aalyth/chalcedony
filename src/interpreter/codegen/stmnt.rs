use super::ToBytecode;

use crate::error::{ChalError, InternalError};
use crate::interpreter::FuncAnnotation;
use crate::lexer::Type;
use crate::parser::ast::{NodeRetStmnt, NodeStmnt};
use crate::utils::Bytecode;

use std::collections::{HashMap, HashSet};

impl ToBytecode for NodeStmnt {
    fn to_bytecode(
        self,
        func_symtable: &mut HashMap<String, FuncAnnotation>,
        local_vars: Option<&mut HashSet<String>>,
    ) -> Result<Vec<u8>, ChalError> {
        if local_vars == None {
            return Err(ChalError::from(InternalError::new(
                "Codegen::NodeStmnt::to_bytecode(): empty local variable set",
            )));
        }
        let local_vars = local_vars.unwrap();

        match self {
            NodeStmnt::VarDef(node) => {
                let var_name = node.name();
                let mut var_bytecode = node.to_bytecode(func_symtable, None)?;

                /* this capacity is for the best case scenario, if we need to delete the already
                 * existing variable, there will be an allocation*/
                let mut result = Vec::<u8>::with_capacity(var_bytecode.len() + 1);

                /* 'shadow' the old variable value */
                if local_vars.contains(&var_name) {
                    result.push(Bytecode::OpDeleteVar as u8);
                    result.extend_from_slice(var_name.as_bytes());
                    result.push(0);
                } else {
                    local_vars.insert(var_name);
                }

                result.append(&mut var_bytecode);
                result.push(200);
                return Ok(result);
            }

            NodeStmnt::FuncCall(node) => {
                let Some(annotation) = func_symtable.get(&node.name()) else {
                    return Err(ChalError::from(InternalError::new(
                        "TODO: make this a proper error for missing function definition",
                    )));
                };

                if annotation.ret_type != Type::Void {
                    return Err(ChalError::from(InternalError::new(
                        "TODO: make this a proper error for calling a non-void function as a statement",
                    )));
                }

                node.to_bytecode(func_symtable, None)
            }

            NodeStmnt::RetStmnt(NodeRetStmnt(expr)) => {
                let mut result = Vec::<u8>::new();
                println!("RETURN EXPR: {:?}\n", expr);
                result.append(&mut expr.to_bytecode(func_symtable, None)?);

                for var in local_vars.iter() {
                    result.push(Bytecode::OpDeleteVar as u8);
                    result.extend_from_slice(var.as_bytes());
                    result.push(0);
                }

                result.push(Bytecode::OpReturn as u8);
                Ok(result)
            }
            _ => Ok(vec![]),
        }
    }
}
