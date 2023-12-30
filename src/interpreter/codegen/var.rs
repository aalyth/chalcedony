use super::ToBytecode;

use crate::error::ChalError;
use crate::interpreter::FuncAnnotation;
use crate::lexer::Type;
use crate::parser::ast::NodeVarDef;
use crate::utils::Bytecode;

use std::collections::BTreeMap;
use std::sync::Mutex;

lazy_static! {
    static ref VAR_ID: Mutex<u64> = Mutex::new(1);
}

pub fn get_var_id(varname: String, var_symtable: &mut BTreeMap<String, u64>) -> u64 {
    if !var_symtable.contains_key(&varname) {
        let mut lock = VAR_ID.lock().unwrap();
        var_symtable.insert(varname.clone(), *lock);
        *lock += 1;
    }
    return var_symtable[&varname];
}

impl ToBytecode for NodeVarDef {
    fn to_bytecode(
        self,
        bytecode_len: usize,
        var_symtable: &mut BTreeMap<String, u64>,
        func_symtable: &mut BTreeMap<String, FuncAnnotation>,
    ) -> Result<Vec<u8>, ChalError> {
        let (var_name, var_type, rhs) = self.disassemble();
        let mut result = rhs.to_bytecode(bytecode_len, var_symtable, func_symtable)?;

        if var_type != Type::Any {
            // SAFETY: the if itself checks for valid conversion input
            let type_bytecode: Bytecode = var_type.try_into().unwrap();
            result.append(&mut vec![Bytecode::OpAssertType as u8, type_bytecode as u8]);
        }

        result.push(Bytecode::OpCreateVar as u8);
        let var_id = get_var_id(var_name, var_symtable);
        result.extend_from_slice(&var_id.to_ne_bytes());
        // result.extend_from_slice(var_name.as_bytes());
        // result.push(0);

        Ok(result)
    }
}
