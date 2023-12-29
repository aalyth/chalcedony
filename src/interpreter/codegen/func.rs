use super::ToBytecode;

use crate::error::{ChalError, RuntimeError};
use crate::interpreter::FuncAnnotation;
use crate::lexer::Type;
use crate::parser::ast::{NodeFuncCall, NodeFuncDef, NodeStmnt};
use crate::utils::Bytecode;

use super::stmnt_to_bytecode;

use std::collections::{BTreeMap, HashSet};

impl ToBytecode for NodeFuncDef {
    fn to_bytecode(
        self,
        bytecode_len: usize,
        func_symtable: &mut BTreeMap<String, FuncAnnotation>,
        func_lookup: &mut BTreeMap<String, u64>,
    ) -> Result<Vec<u8>, ChalError> {
        let (name, args, ret_type, body_raw, start, end, span) = self.disassemble();

        /* compile the bytecode for all body statements */
        let mut body = Vec::<u8>::new();
        let mut var_lookup = HashSet::<String>::new();
        for arg in &args {
            var_lookup.insert(arg.0.clone());
        }

        let annotation = FuncAnnotation::new(args, ret_type.clone());
        func_symtable.insert(name.clone(), annotation);
        func_lookup.insert(name.clone(), bytecode_len as u64);

        let mut mock_lookup = HashSet::<String>::new();
        let mut returned = false;
        for stmnt in body_raw {
            if let NodeStmnt::RetStmnt(_) = stmnt {
                returned = true;
            }
            body.append(&mut stmnt_to_bytecode(
                stmnt,
                bytecode_len,
                func_symtable,
                func_lookup,
                &mut mock_lookup,
                &mut var_lookup,
            )?)
        }

        match ret_type {
            Type::Void if !returned => {
                body.push(Bytecode::OpReturn as u8);
            }
            _ if !returned => {
                return Err(RuntimeError::no_default_return_stmnt(start, end, span).into())
            }
            _ => {}
        }

        /* remove the arguments as local variables */
        for var in var_lookup.into_iter() {
            body.push(Bytecode::OpDeleteVar as u8);
            body.extend_from_slice(var.as_bytes());
            body.push(0);
        }

        Ok(body)
    }
}

impl ToBytecode for NodeFuncCall {
    fn to_bytecode(
        self,
        bytecode_len: usize,
        func_symtable: &mut BTreeMap<String, FuncAnnotation>,
        func_lookup: &mut BTreeMap<String, u64>,
    ) -> Result<Vec<u8>, ChalError> {
        let (fn_name, args, start, end, span) = self.disassemble();
        /* this check is already performed in expression and statement node parsing, but it's safer */
        if !func_symtable.contains_key(&fn_name) {
            return Err(RuntimeError::unknown_function(fn_name, start, end, span).into());
        }
        let annotation = (*func_symtable.get(&fn_name).unwrap()).clone();

        if annotation.args.len() != args.len() {
            if annotation.args.len() < args.len() {
                return Err(RuntimeError::too_many_arguments(
                    annotation.args.len(),
                    args.len(),
                    start,
                    end,
                    span,
                )
                .into());
            } else {
                return Err(RuntimeError::too_few_arguments(
                    annotation.args.len(),
                    args.len(),
                    start,
                    end,
                    span,
                )
                .into());
            }
        }

        /* set up the function arguments as local variables inside the function scope */
        let mut result = Vec::<u8>::new();
        let arg_iter = args.into_iter();
        let mut annotation_iter = annotation.args.into_iter();

        for arg in arg_iter {
            /* push the argument value */
            result.append(&mut arg.to_bytecode(bytecode_len, func_symtable, func_lookup)?);

            let ann = annotation_iter.next().unwrap();

            /* assert the argument type */
            let type_bytecode: Bytecode = ann.1.try_into().unwrap();
            result.append(&mut vec![Bytecode::OpAssertType as u8, type_bytecode as u8]);

            /* put the argument value inside a local variable */
            result.push(Bytecode::OpCreateVar as u8);
            result.extend_from_slice(ann.0.as_bytes());
            result.push(0);
        }

        /* complete the function call instruction */
        let fn_pos = func_lookup[&fn_name];
        result.push(Bytecode::OpCallFunc as u8);
        result.extend_from_slice(&fn_pos.to_ne_bytes());

        Ok(result)
    }
}
