use super::ToBytecode;

use crate::error::ChalError;
use crate::interpreter::FuncAnnotation;
use crate::lexer::Type;
use crate::parser::ast::NodeVarDef;
use crate::utils::Bytecode;

use std::collections::HashMap;

impl ToBytecode for NodeVarDef {
    fn to_bytecode(
        self,
        func_symtable: &mut HashMap<String, FuncAnnotation>,
    ) -> Result<Vec<u8>, ChalError> {
        let (var_name, var_type, rhs) = self.disassemble();
        let mut result = rhs.to_bytecode(func_symtable)?;

        if var_type != Type::Any {
            // SAFETY: the if itself checks for valid conversion input
            let type_bytecode: Bytecode = var_type.try_into().unwrap();
            result.append(&mut vec![Bytecode::OpAssertType as u8, type_bytecode as u8]);
        }

        result.push(Bytecode::OpCreateVar as u8);

        result.extend_from_slice(var_name.as_bytes());
        result.push(0);

        Ok(result)
    }
}
