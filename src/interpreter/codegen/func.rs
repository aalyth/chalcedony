use super::ToBytecode;

use crate::error::{ChalError, RuntimeError};
use crate::interpreter::Chalcedony;
use crate::lexer::Type;
use crate::parser::ast::{NodeFuncCall, NodeFuncDef, NodeStmnt};
use crate::utils::Bytecode;

use std::collections::HashSet;

impl ToBytecode for NodeFuncDef {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        /* compile the bytecode for all body statements */
        let mut body = Vec::<Bytecode>::new();
        let mut var_lookup = HashSet::<String>::new();
        for arg in &self.args {
            var_lookup.insert(arg.0.clone());
            /* this asserts the variable is in the var symtable */
            // let _ = get_var_id(arg.0.clone(), intevar_symtable);
        }

        interpreter.create_function(self.name.clone(), self.args, self.ret_type.clone());

        let mut returned = false;
        for stmnt in self.body {
            if let NodeStmnt::RetStmnt(_) = stmnt {
                returned = true;
            }
            body.append(&mut stmnt.to_bytecode(interpreter)?);
            // if we have a return statement, there's no need to waste time on unreachable code
            if returned {
                break;
            }
        }

        match self.ret_type {
            Type::Void if body.len() == 0 && !returned => {
                return Err(RuntimeError::no_default_return_stmnt(self.span).into())
            }
            Type::Void if !returned => {
                body.push(Bytecode::Return);
            }
            _ if !returned => return Err(RuntimeError::no_default_return_stmnt(self.span).into()),
            _ => {}
        }

        let Some(annotation) = interpreter.current_func.clone() else {
            panic!("TODO: check if this is ok");
        };
        let annotation = annotation.borrow();
        let mut result = Vec::<Bytecode>::with_capacity(body.len() + 1);
        result.push(Bytecode::CreateFunc(
            annotation.args.len(),
            annotation.locals_id_counter,
        ));
        result.append(&mut body);

        interpreter.current_func = None;
        Ok(result)
    }
}

impl ToBytecode for NodeFuncCall {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let Some(annotation) = interpreter.func_symtable.get(&self.name).cloned() else {
            return Err(RuntimeError::unknown_function(self.name, self.span).into());
        };
        let annotation = annotation.borrow();

        /* Checking for mismatching number of arguments */
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
        // let mut annotation_iter = annotation.args.clone().into_iter();

        for arg in arg_iter {
            /* push the argument value */
            result.append(&mut arg.to_bytecode(interpreter)?);

            // let ann = annotation_iter.next().unwrap();

            // TODO: check the type
        }

        /* complete the function call instruction */
        // TODO: convert to using usize
        result.push(Bytecode::CallFunc(annotation.id));

        Ok(result)
    }
}
