use super::ToBytecode;

use crate::error::{ChalError, CompileError};
use crate::interpreter::{ArgAnnotation, Chalcedony};
use crate::parser::ast::{NodeFuncCall, NodeFuncDef, NodeStmnt};

use crate::common::{Bytecode, Type};
use itertools::izip;

use ahash::AHashMap;

impl ToBytecode for NodeFuncDef {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let arg_types: Vec<Type> = self.args.iter().map(|arg| arg.ty).collect();
        if interpreter.get_function(&self.name, &arg_types).is_some() {
            return Err(CompileError::overwritten_function(self.span).into());
        }

        /* enumerate over the function's arguments into a sequence of annotations */
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

        // check whether the function has returned, and if it is a void function,
        // append a `Bytecode::ReturnVoid` at the end if there isn't one
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
        interpreter.locals = AHashMap::new();
        Ok(result)
    }
}

impl ToBytecode for NodeFuncCall {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let arg_types: Result<Vec<Type>, ChalError> = self
            .args
            .iter()
            .map(|expr| expr.as_type(interpreter))
            .collect();
        let arg_types = match arg_types {
            Ok(ok) => ok,
            Err(err) => return Err(err),
        };

        let Some(annotation) = interpreter.get_function(&self.name, &arg_types).cloned() else {
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
        for (arg, arg_ty, exp) in izip!(self.args, arg_types, annotation.args.clone()) {
            // NOTE: this is very important to go in before the type check,
            // else an empty value cast is possible
            result.extend(arg.clone().to_bytecode(interpreter)?);
            Type::verify(exp.ty, arg_ty, &mut result, arg.span.clone())?;
        }

        /* complete the function call instruction */
        result.push(Bytecode::CallFunc(annotation.id));

        Ok(result)
    }
}
