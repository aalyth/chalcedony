use super::ToBytecode;

use crate::error::ChalError;
use crate::interpreter::FuncAnnotation;
use crate::lexer::Type;
use crate::parser::ast::NodeVarDef;
use crate::utils::Bytecode;

use std::collections::BTreeMap;
use std::sync::Mutex;

lazy_static! {
    static ref VAR_ID: Mutex<usize> = Mutex::new(1);
}

pub fn get_var_id(varname: String, var_symtable: &mut BTreeMap<String, usize>) -> usize {
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
        var_symtable: &mut BTreeMap<String, usize>,
        func_symtable: &mut BTreeMap<String, FuncAnnotation>,
    ) -> Result<Vec<Bytecode>, ChalError> {
        let (var_name, var_type, rhs) = self.disassemble();
        let mut result = rhs.to_bytecode(bytecode_len, var_symtable, func_symtable)?;

        if var_type != Type::Any {
            result.push(Bytecode::Assert(var_type));
        }

        let var_id = get_var_id(var_name, var_symtable);
        result.push(Bytecode::CreateVar(var_id));
        Ok(result)
    }
}
