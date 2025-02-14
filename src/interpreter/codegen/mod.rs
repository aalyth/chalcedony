//! The module responsible for compiling the `AST`, generated by the parser,
//! into the bytecode instructions, executed by the `CVM`.

pub mod class;
pub mod expr;
pub mod func;
pub mod stmnt;
pub mod var;

use crate::common::{Bytecode, Type};
use crate::error::{ChalError, CompileError, CompileErrorKind};
use crate::parser::ast::{NodeFuncCallStmnt, NodeImport, NodeProg};
use func::compile_func_call_inner;

use super::{Chalcedony, ScriptType};

use std::path::Path;

pub trait ToBytecode {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError>;
}

impl ToBytecode for NodeProg {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        match self {
            NodeProg::VarDef(node) => node.to_bytecode(interpreter),
            NodeProg::FuncDef(node) => node.to_bytecode(interpreter),
            NodeProg::FuncCall(NodeFuncCallStmnt(node)) => node.to_bytecode(interpreter),
            NodeProg::Assign(node) => node.to_bytecode(interpreter),
            NodeProg::IfStmnt(node) => node.to_bytecode(interpreter),
            NodeProg::WhileLoop(node) => node.to_bytecode(interpreter),
            NodeProg::ForLoop(node) => node.to_bytecode(interpreter),
            NodeProg::TryCatch(node) => node.to_bytecode(interpreter),
            NodeProg::Import(node) => node.to_bytecode(interpreter),
            NodeProg::Class(node) => node.to_bytecode(interpreter),
        }
    }
}

impl ToBytecode for NodeImport {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let script_path = interpreter.current_path.join(&self.path);

        if !script_path.exists() {
            return Err(CompileError::new(
                CompileErrorKind::ScriptNotFound(script_path.to_str().unwrap().to_string()),
                self.span,
            )
            .into());
        }

        if interpreter.imported_scripts.contains(&script_path) {
            return Ok(Vec::new());
        }
        interpreter.imported_scripts.insert(script_path.clone());

        let parent_script_type = interpreter.script_type;
        interpreter.script_type = ScriptType::Imported;
        let parent_path = interpreter.current_path.clone();
        interpreter.current_path = script_path.parent().unwrap_or(Path::new("")).into();

        let script_const_id = interpreter.get_global_id_internal("__name__", Type::Str, true);
        interpreter.vm.execute(vec![
            Bytecode::ConstS(
                interpreter
                    .current_path
                    .to_str()
                    .unwrap()
                    .to_string()
                    .into(),
            ),
            Bytecode::SetGlobal(script_const_id),
        ]);

        interpreter.interpret_script(script_path.to_str().unwrap().to_string());

        interpreter.script_type = parent_script_type;
        interpreter.current_path = parent_path;

        let name_value = match interpreter.script_type {
            ScriptType::Main => "__main__".to_string(),
            ScriptType::Imported => interpreter.current_path.to_str().unwrap().to_string(),
        };

        interpreter.vm.execute(vec![
            Bytecode::ConstS(name_value.into()),
            Bytecode::SetGlobal(script_const_id),
        ]);
        Ok(Vec::new())
    }
}
