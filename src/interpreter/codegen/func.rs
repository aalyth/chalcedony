use super::var::get_var_id;
use super::ToBytecode;

use crate::error::{ChalError, RuntimeError};
use crate::interpreter::FuncAnnotation;
use crate::lexer::Type;
use crate::parser::ast::{NodeFuncCall, NodeFuncDef, NodeStmnt};
use crate::utils::BytecodeOpr;

use super::stmnt_to_bytecode;

use std::collections::{BTreeMap, HashSet};

impl ToBytecode for NodeFuncDef {
    fn to_bytecode(
        self,
        bytecode_len: usize,
        var_symtable: &mut BTreeMap<String, usize>,
        func_symtable: &mut BTreeMap<String, FuncAnnotation>,
    ) -> Result<Vec<BytecodeOpr>, ChalError> {
        let (name, args, ret_type, body_raw, start, end, span) = self.disassemble();

        /* compile the bytecode for all body statements */
        let mut body = Vec::<BytecodeOpr>::new();
        let mut var_lookup = HashSet::<String>::new();
        for arg in &args {
            var_lookup.insert(arg.0.clone());
        }

        let annotation = FuncAnnotation::new(args, ret_type.clone(), bytecode_len as u64);
        func_symtable.insert(name.clone(), annotation);

        let mut mock_lookup = HashSet::<String>::new();
        let mut returned = false;
        for stmnt in body_raw {
            if let NodeStmnt::RetStmnt(_) = stmnt {
                returned = true;
            }
            body.append(&mut stmnt_to_bytecode(
                stmnt,
                bytecode_len,
                var_symtable,
                func_symtable,
                &mut mock_lookup,
                &mut var_lookup,
            )?)
        }

        match ret_type {
            Type::Void if !returned => {
                body.push(BytecodeOpr::Return);
            }
            _ if !returned => {
                return Err(RuntimeError::no_default_return_stmnt(start, end, span).into())
            }
            _ => {}
        }

        /* remove the arguments as local variables */
        for var in var_lookup.into_iter() {
            let var_id = get_var_id(var, var_symtable);
            body.push(BytecodeOpr::DeleteVar(var_id))
        }

        Ok(body)
    }
}

impl ToBytecode for NodeFuncCall {
    fn to_bytecode(
        self,
        bytecode_len: usize,
        var_symtable: &mut BTreeMap<String, usize>,
        func_symtable: &mut BTreeMap<String, FuncAnnotation>,
    ) -> Result<Vec<BytecodeOpr>, ChalError> {
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
        let mut result = Vec::<BytecodeOpr>::new();
        let arg_iter = args.into_iter();
        let mut annotation_iter = annotation.args.into_iter();

        for arg in arg_iter {
            /* push the argument value */
            result.append(&mut arg.to_bytecode(bytecode_len, var_symtable, func_symtable)?);

            let ann = annotation_iter.next().unwrap();

            /* assert the argument type */
            result.push(BytecodeOpr::Assert(ann.1));

            /* put the argument value inside a local variable */
            let var_id = get_var_id(ann.0, var_symtable);
            result.push(BytecodeOpr::CreateVar(var_id));
        }

        /* complete the function call instruction */
        // TODO: convert to using usize
        let fn_pos = func_symtable[&fn_name].location as usize;
        result.push(BytecodeOpr::CallFunc(fn_pos));

        Ok(result)
    }
}
