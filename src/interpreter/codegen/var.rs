use super::ToBytecode;

use crate::error::{ChalError, RuntimeError};
use crate::interpreter::Chalcedony;
use crate::parser::ast::{NodeVarCall, NodeVarDef};
use crate::utils::Bytecode;

impl ToBytecode for NodeVarCall {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        if let Some(current_func) = interpreter.current_func.clone() {
            let current_func = current_func.borrow();
            if let Some(var_id) = current_func.locals_symtable.get(&self.name) {
                return Ok(vec![Bytecode::GetLocal(*var_id)]);
            }
            if let Some(var_id) = current_func.get_arg(&self.name) {
                return Ok(vec![Bytecode::GetArg(var_id)]);
            }
        }
        if let Some(var_id) = interpreter.globals_symtable.get(&self.name) {
            return Ok(vec![Bytecode::GetGlobal(*var_id)]);
        }
        // TODO: make this a runtime error
        return Err(RuntimeError::unknown_variable(self.name, self.span).into());
    }
}

impl ToBytecode for NodeVarDef {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let mut result = self.value.to_bytecode(interpreter)?;

        // TODO: assert the variable type
        /*
        if self.kind != Type::Any {
            result.push(Bytecode::Assert(self.kind));
        }
        */

        let var_id = interpreter.get_global_id(&self.name);
        result.push(Bytecode::SetGlobal(var_id));
        Ok(result)
    }
}
