use super::ToBytecode;

use crate::error::{ChalError, InternalError};
use crate::interpreter::FuncAnnotation;
use crate::lexer::Type;
use crate::parser::ast::{NodeFuncCall, NodeFuncDef};
use crate::utils::Bytecode;

use std::collections::{HashMap, HashSet};

impl ToBytecode for NodeFuncDef {
    fn to_bytecode(
        self,
        func_symtable: &mut HashMap<String, FuncAnnotation>,
        _: Option<&mut HashSet<String>>,
    ) -> Result<Vec<u8>, ChalError> {
        let (name, args, ret, body_raw) = self.disassemble();

        /* compile the bytecode for all body statements */
        let mut body = Vec::<u8>::new();
        let mut var_lookup = HashSet::<String>::new();
        for arg in &args {
            var_lookup.insert(arg.0.clone());
        }

        let annotation = FuncAnnotation::new(args, ret);
        func_symtable.insert(name.clone(), annotation);

        for stmnt in body_raw {
            body.append(&mut stmnt.to_bytecode(func_symtable, Some(&mut var_lookup))?)
        }

        /* remove the arguments as local variables */
        for var in var_lookup.into_iter() {
            body.push(Bytecode::OpDeleteVar as u8);
            body.extend_from_slice(var.as_bytes());
            body.push(0);
        }

        /* complete the function creation instruction */
        let mut result = Vec::<u8>::with_capacity(1 + name.len() + 1 + 8 + body.len());
        result.push(Bytecode::OpCreateFunc as u8);
        result.extend_from_slice(name.as_bytes());
        result.push(0);
        result.extend_from_slice(&(body.len() as u64).to_ne_bytes());
        result.append(&mut body);

        Ok(result)
    }
}

impl ToBytecode for NodeFuncCall {
    fn to_bytecode(
        self,
        func_symtable: &mut HashMap<String, FuncAnnotation>,
        _: Option<&mut HashSet<String>>,
    ) -> Result<Vec<u8>, ChalError> {
        let (fn_name, args) = self.disassemble();
        if !func_symtable.contains_key(&fn_name) {
            return Err(ChalError::from(InternalError::new(
                "TODO: make this a proper error for missing function definition",
            )));
        }
        let annotation = (*func_symtable.get(&fn_name).unwrap()).clone();

        if annotation.args.len() != args.len() {
            return Err(ChalError::from(InternalError::new(
                "TODO: make this a proper error for mismatching number of arguments",
            )));
        }

        /* set up the function arguments as local variables inside the function scope */
        let mut result = Vec::<u8>::new();
        let arg_iter = args.into_iter();
        let mut annotation_iter = annotation.args.into_iter();

        for arg in arg_iter {
            /* push the argument value */
            result.append(&mut arg.to_bytecode(func_symtable, None)?);

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
        result.push(Bytecode::OpCallFunc as u8);
        result.extend_from_slice(fn_name.as_bytes());
        result.push(0);

        Ok(result)
    }
}
