use super::ToBytecode;

use crate::error::{ChalError, CompileError};
use crate::interpreter::Chalcedony;
use crate::parser::ast::{NodeVarCall, NodeVarDef};

use crate::common::{Bytecode, Type};

impl ToBytecode for NodeVarCall {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        if let Some(func) = interpreter.current_func.clone() {
            if let Some(var) = func.borrow().arg_lookup.get(&self.name) {
                return Ok(vec![Bytecode::GetArg(var.id)]);
            }
        }
        if let Some(var) = interpreter.locals.borrow().get(&self.name) {
            return Ok(vec![Bytecode::GetLocal(var.id)]);
        }
        if let Some(var) = interpreter.globals.get(&self.name) {
            return Ok(vec![Bytecode::GetGlobal(var.id)]);
        }
        return Err(CompileError::unknown_variable(self.name, self.span).into());
    }
}

impl ToBytecode for NodeVarDef {
    fn to_bytecode(mut self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        if interpreter.locals.borrow().contains_key(&self.name) {
            return Err(CompileError::redefining_variable(self.span.clone()).into());
        }

        let value_type = self.value.as_type(&interpreter)?;
        if self.ty != Type::Any {
            if self.ty != value_type {
                return Err(CompileError::invalid_type(
                    self.ty,
                    value_type,
                    self.value.span.clone(),
                )
                .into());
            }
        } else {
            self.ty = value_type;
        }

        let var_id = interpreter.get_global_id(&self);
        let mut result = self.value.to_bytecode(interpreter)?;
        result.push(Bytecode::SetGlobal(var_id));
        Ok(result)
    }
}
