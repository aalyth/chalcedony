use super::ToBytecode;

use crate::error::{ChalError, CompileError};
use crate::interpreter::Chalcedony;
use crate::parser::ast::{NodeVarCall, NodeVarDef};

use crate::common::{Bytecode, Type};

impl ToBytecode for NodeVarCall {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        if let Some(current_func) = interpreter.current_func.clone() {
            let current_func = current_func.borrow();
            if let Some(var) = current_func.args.get(&self.name) {
                return Ok(vec![Bytecode::GetArg(var.id)]);
            }

            if let Some(var) = current_func.locals.get(&self.name) {
                return Ok(vec![Bytecode::GetLocal(var.id)]);
            }
        }
        if let Some(var) = interpreter.globals.get(&self.name) {
            return Ok(vec![Bytecode::GetGlobal(var.id)]);
        }
        return Err(CompileError::unknown_variable(self.name, self.span).into());
    }
}

impl ToBytecode for NodeVarDef {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let var_id = interpreter.get_global_id(&self);

        if self.ty != Type::Any {
            let value_type = self.value.as_type(&interpreter)?;
            if self.ty != value_type {
                return Err(CompileError::invalid_type(
                    self.ty,
                    value_type,
                    self.value.span.clone(),
                )
                .into());
            }
        }
        let mut result = self.value.to_bytecode(interpreter)?;
        result.push(Bytecode::SetGlobal(var_id));
        Ok(result)
    }
}
