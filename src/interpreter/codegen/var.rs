use super::ToBytecode;

use crate::error::{ChalError, CompileError};
use crate::interpreter::Chalcedony;
use crate::parser::ast::{NodeVarCall, NodeVarDef};

use crate::common::{Bytecode, Type};

impl ToBytecode for NodeVarCall {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        if let Some(func) = interpreter.current_func.clone() {
            if let Some(var) = func.arg_lookup.get(&self.name) {
                return Ok(vec![Bytecode::GetArg(var.id)]);
            }
        }
        if let Some(var) = interpreter.locals.borrow().get(&self.name) {
            return Ok(vec![Bytecode::GetLocal(var.id)]);
        }
        if let Some(var) = interpreter.globals.get(&self.name) {
            return Ok(vec![Bytecode::GetGlobal(var.id)]);
        }
        Err(CompileError::unknown_variable(self.name, self.span).into())
    }
}

impl ToBytecode for NodeVarDef {
    fn to_bytecode(mut self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        if interpreter.locals.borrow().contains_key(&self.name) {
            return Err(CompileError::redefining_variable(self.span.clone()).into());
        }

        let mut result = self.value.clone().to_bytecode(interpreter)?;

        let value_type = self.value.clone().as_type(interpreter)?;
        if self.ty != Type::Any {
            Type::verify(
                self.ty.clone(),
                value_type,
                &mut result,
                self.value.span.clone(),
            )?;
        } else if value_type != Type::Void {
            self.ty = value_type;
        } else {
            /* check whether no value was provided */
            return Err(
                CompileError::invalid_type(Type::Any, Type::Void, self.span.clone()).into(),
            );
        }

        let var_id = interpreter.get_global_id(&self);
        result.push(Bytecode::SetGlobal(var_id));
        Ok(result)
    }
}

pub fn var_exists(name: &str, interpreter: &Chalcedony) -> bool {
    if let Some(func) = interpreter.current_func.clone() {
        if func.arg_lookup.get(name).is_some() {
            return true;
        }
    }
    if interpreter.locals.borrow().get(name).is_some() || interpreter.globals.get(name).is_some() {
        return true;
    }
    false
}
