use super::ToBytecode;

use crate::error::{ChalError, CompileError, CompileErrorKind};
use crate::interpreter::{ArgAnnotation, Chalcedony, SafetyScope};
use crate::parser::ast::{NodeFuncCall, NodeFuncDef, NodeStmnt};

use crate::common::{Bytecode, Type};
use itertools::izip;
use std::collections::VecDeque;

use ahash::AHashMap;

impl ToBytecode for NodeFuncDef {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let arg_types: VecDeque<Type> = self.args.iter().map(|arg| arg.ty.clone()).collect();
        if interpreter
            .get_function(&self.name, &arg_types, self.namespace.as_ref())
            .is_some()
        {
            return Err(CompileError::new(CompileErrorKind::OverwrittenFunction, self.span).into());
        }

        // enumerate over the function's arguments to a sequence of annotations
        let mut args = Vec::<ArgAnnotation>::new();
        for (idx, arg) in self.args.iter().enumerate() {
            if arg.ty == Type::Void {
                return Err(CompileError::new(CompileErrorKind::VoidArgument, self.span).into());
            }
            args.push(ArgAnnotation::new(idx, arg.name.clone(), arg.ty.clone()));
        }

        interpreter.create_function(&self, args);

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

        // check whether the function has returned, and if it is a void
        // functionm, append `Bytecode::ReturnVoid` at the end if there is not
        match self.ret_type {
            Type::Void if body.is_empty() && !returned => {
                return Err(
                    CompileError::new(CompileErrorKind::NoDefaultReturnStmnt, self.span).into(),
                )
            }
            Type::Void if !returned => {
                body.push(Bytecode::ReturnVoid);
            }
            _ if !returned => {
                return Err(
                    CompileError::new(CompileErrorKind::NoDefaultReturnStmnt, self.span).into(),
                )
            }
            _ => {}
        }

        let Some(annotation) = interpreter.current_func.clone() else {
            panic!("Chalcedony::create_function() did not set the annotation properly");
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
        compile_func_call_inner(self, interpreter, None)
    }
}

pub fn compile_func_call_inner(
    mut node: NodeFuncCall,
    interpreter: &mut Chalcedony,
    parent_type: Option<Type>,
) -> Result<Vec<Bytecode>, ChalError> {
    let arg_types: Result<VecDeque<Type>, ChalError> = node
        .args
        .iter()
        .map(|expr| expr.as_type(interpreter))
        .collect();
    let mut arg_types = match arg_types {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    /* the function is called as a method */
    if let Some(ty) = &parent_type {
        arg_types.push_front(ty.clone());
        if node.namespace.is_some() {
            panic!("calling a namespace function also as a method");
        }
        node.namespace = Some(ty.clone().as_class());
    }

    /* check whether function's namespace exists */
    if let Some(namespace) = &node.namespace {
        if !interpreter.namespaces.contains_key(namespace) {
            return Err(CompileError::new(
                CompileErrorKind::UnknownNamespace(namespace.clone()),
                node.span,
            )
            .into());
        }
    }

    /* get the function's annotation */
    let Some(annotation) = interpreter
        .get_function(&node.name, &arg_types, node.namespace.as_ref())
        .cloned()
    else {
        let mut func_name = node.name;
        if let Some(class) = node.namespace {
            func_name = class + "::" + &func_name;
        }
        return Err(
            CompileError::new(CompileErrorKind::UnknownFunction(func_name), node.span).into(),
        );
    };

    // check whether an unsafe function is called within in an unguarded scope
    if node.name.ends_with('!') && interpreter.safety_scope == SafetyScope::Catch {
        return Err(CompileError::new(CompileErrorKind::UnsafeCatch, node.span).into());
    }

    /* check for mismatching number of arguments */
    if annotation.args.len() != arg_types.len() {
        if annotation.args.len() < arg_types.len() {
            return Err(CompileError::new(
                CompileErrorKind::TooManyArguments(annotation.args.len(), arg_types.len()),
                node.span,
            )
            .into());
        }
        return Err(CompileError::new(
            CompileErrorKind::TooFewArguments(annotation.args.len(), arg_types.len()),
            node.span,
        )
        .into());
    }

    /* if the function is a method the first argument is already compiled */
    let mut arguments = izip!(node.args, arg_types, annotation.args.clone());
    if parent_type.is_some() {
        arguments.next();
    }

    /* push on the stack each of the argument's expression value */
    let mut result = Vec::<Bytecode>::new();
    for (arg, arg_ty, exp) in arguments {
        result.extend(arg.clone().to_bytecode(interpreter)?);
        Type::verify(exp.ty, arg_ty, &mut result, arg.span.clone())?;
    }

    /* complete the function call instruction */
    result.push(Bytecode::CallFunc(annotation.id));

    Ok(result)
}
