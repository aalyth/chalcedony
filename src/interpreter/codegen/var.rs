use super::ToBytecode;

use crate::error::ChalError;
use crate::interpreter::FuncAnnotation;
use crate::lexer::Type;
use crate::parser::ast::NodeVarDef;
use crate::utils::Bytecode;

use std::collections::BTreeMap;

impl ToBytecode for NodeVarDef {
    fn to_bytecode(
        self,
        bytecode_len: usize,
        func_symtable: &mut BTreeMap<String, FuncAnnotation>,
        func_lookup: &mut BTreeMap<String, u64>,
    ) -> Result<Vec<u8>, ChalError> {
        let (var_name, var_type, rhs) = self.disassemble();
        let mut result = rhs.to_bytecode(bytecode_len, func_symtable, func_lookup)?;

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
