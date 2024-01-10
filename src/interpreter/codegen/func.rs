use super::var::get_var_id;
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
        var_symtable: &mut BTreeMap<String, usize>,
        func_symtable: &mut BTreeMap<String, FuncAnnotation>,
    ) -> Result<Vec<Bytecode>, ChalError> {
        /* compile the bytecode for all body statements */
        let mut body = Vec::<Bytecode>::new();
        let mut var_lookup = HashSet::<String>::new();
        for arg in &self.args {
            var_lookup.insert(arg.0.clone());
            /* this asserts the variable is in the var symtable */
            let _ = get_var_id(arg.0.clone(), var_symtable);
        }

        let annotation = FuncAnnotation::new(self.args, self.ret_type.clone(), bytecode_len as u64);
        func_symtable.insert(self.name.clone(), annotation);

        let mut mock_lookup = HashSet::<String>::new();
        let mut returned = false;
        for stmnt in self.body {
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

        match self.ret_type {
            Type::Void if !returned => {
                body.push(Bytecode::Return);
            }
            _ if !returned => return Err(RuntimeError::no_default_return_stmnt(self.span).into()),
            _ => {}
        }

        /* remove the arguments as local variables */
        for var in var_lookup.into_iter() {
            let var_id = get_var_id(var, var_symtable);
            body.push(Bytecode::DeleteVar(var_id))
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
    ) -> Result<Vec<Bytecode>, ChalError> {
        // let (fn_name, args, start, end, span) = self.disassemble();
        /* this check is already performed in expression and statement node parsing, but it's safer */
        if !func_symtable.contains_key(&self.name) {
            return Err(RuntimeError::unknown_function(self.name, self.span).into());
        }
        let annotation = (*func_symtable.get(&self.name).unwrap()).clone();

        if annotation.args.len() != self.args.len() {
            if annotation.args.len() < self.args.len() {
                return Err(RuntimeError::too_many_arguments(
                    annotation.args.len(),
                    self.args.len(),
                    self.span,
                )
                .into());
            } else {
                return Err(RuntimeError::too_few_arguments(
                    annotation.args.len(),
                    self.args.len(),
                    self.span,
                )
                .into());
            }
        }

        /* set up the function arguments as local variables inside the function scope */
        let mut result = Vec::<Bytecode>::new();
        let arg_iter = self.args.into_iter();
        let mut annotation_iter = annotation.args.into_iter();

        for arg in arg_iter {
            /* push the argument value */
            result.append(&mut arg.to_bytecode(bytecode_len, var_symtable, func_symtable)?);

            let ann = annotation_iter.next().unwrap();

            /* assert the argument type */
            result.push(Bytecode::Assert(ann.1));

            /* put the argument value inside a local variable */
            let var_id = get_var_id(ann.0, var_symtable);
            result.push(Bytecode::CreateVar(var_id));
        }

        /* complete the function call instruction */
        // TODO: convert to using usize
        let fn_pos = func_symtable[&self.name].location as usize;
        result.push(Bytecode::CallFunc(fn_pos));

        Ok(result)
    }
}
