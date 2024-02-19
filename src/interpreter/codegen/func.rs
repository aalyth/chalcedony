use std::cell::RefCell;

use super::ToBytecode;

use crate::error::{ChalError, CompileError};
use crate::interpreter::{ArgAnnotation, Chalcedony};
use crate::parser::ast::{NodeFuncCall, NodeFuncDef, NodeStmnt};

use crate::common::{Bytecode, Type};

use ahash::AHashMap;

impl ToBytecode for NodeFuncDef {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        if interpreter.func_symtable.get(&self.name).is_some() {
            return Err(CompileError::overloaded_function(self.span).into());
        }

        /* enumerate over the function's arguments */
        let mut args = Vec::<ArgAnnotation>::new();
        for (idx, arg) in self.args.iter().enumerate() {
            if arg.ty == Type::Void {
                return Err(CompileError::void_argument(self.span).into());
            }
            args.push(ArgAnnotation::new(idx, arg.name.clone(), arg.ty));
        }

        interpreter.create_function(self.name.clone(), args, self.ret_type);

        /* compile the bytecode for each statement in the body */
        let mut body = Vec::<Bytecode>::new();
        let mut errors = Vec::<ChalError>::new();
        let mut returned = false;
        for stmnt in self.body {
            if let NodeStmnt::RetStmnt(_) = stmnt {
                returned = true;
            }

            match stmnt.to_bytecode(interpreter) {
                Ok(bytecode) => body.extend(bytecode),
                Err(err) => errors.push(err),
            }
        }

        if !errors.is_empty() {
            return Err(errors.into());
        }

        /* check whether the function has returned, and if it is a void function, append
         * a ReturnVoid at the end if there isn't one */
        match self.ret_type {
            Type::Void if body.is_empty() && !returned => {
                return Err(CompileError::no_default_return_stmnt(self.span).into())
            }
            Type::Void if !returned => {
                body.push(Bytecode::ReturnVoid);
            }
            _ if !returned => return Err(CompileError::no_default_return_stmnt(self.span).into()),
            _ => {}
        }

        let Some(annotation) = interpreter.current_func.clone() else {
            panic!("CVM::create_function() did not set the annotation properly");
        };

        let mut result = Vec::<Bytecode>::with_capacity(body.len() + 1);
        result.push(Bytecode::CreateFunc(annotation.args.len()));
        result.append(&mut body);

        interpreter.current_func = None;
        interpreter.locals = RefCell::new(AHashMap::new());
        Ok(result)
    }
}

impl ToBytecode for NodeFuncCall {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let Some(annotation) = interpreter.func_symtable.get(&self.name).cloned() else {
            return Err(CompileError::unknown_function(self.name, self.span).into());
        };

        /* check for mismatching number of arguments */
        if annotation.args.len() != self.args.len() {
            if annotation.args.len() < self.args.len() {
                return Err(CompileError::too_many_arguments(
                    annotation.args.len(),
                    self.args.len(),
                    self.span,
                )
                .into());
            }
            return Err(CompileError::too_few_arguments(
                annotation.args.len(),
                self.args.len(),
                self.span,
            )
            .into());
        }

        if annotation.ret_type != Type::Void && interpreter.inside_stmnt {
            return Err(
                CompileError::non_void_func_stmnt(annotation.ret_type, self.span.clone()).into(),
            );
        }

        /* push on the stack each of the argument's expression value */
        let mut result = Vec::<Bytecode>::new();
        for (idx, arg_expr) in self.args.into_iter().enumerate() {
            let exp_type = annotation
                .args
                .get(idx)
                .expect("the argument bounds should have already been checked")
                .ty;

            result.extend(arg_expr.clone().to_bytecode(interpreter)?);
            let recv_type = arg_expr.as_type(interpreter)?;
            Type::verify(exp_type, recv_type, &mut result, arg_expr.span)?;
        }

        /* complete the function call instruction */
        result.push(Bytecode::CallFunc(annotation.id));

        Ok(result)
    }
}
