mod codegen;
use codegen::ToBytecode;

use crate::error::{span::Spanning, ChalError};
use crate::lexer::Type;
use crate::parser::ast::NodeProg;
use crate::parser::Parser;
use crate::vm::CVM;

use crate::utils::Bytecode;

use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct FuncAnnotation {
    args: Vec<(String, Type)>,
    ret_type: Type,
    location: u64,
}

impl FuncAnnotation {
    pub fn new(args: Vec<(String, Type)>, ret_type: Type, location: u64) -> Self {
        FuncAnnotation {
            args,
            ret_type,
            location,
        }
    }
}

pub struct Chalcedony {
    vm: CVM,
    var_symtable: BTreeMap<String, usize>,
    func_symtable: BTreeMap<String, FuncAnnotation>,
}

impl Chalcedony {
    pub fn new() -> Self {
        let mut func_symtable = BTreeMap::<String, FuncAnnotation>::new();
        func_symtable.insert(
            "print".to_string(),
            FuncAnnotation::new(vec![(String::from("output"), Type::Str)], Type::Void, 0),
        );

        let mut var_symtable = BTreeMap::<String, usize>::new();
        var_symtable.insert("output".to_string(), 0);

        Chalcedony {
            vm: CVM::new(),
            var_symtable,
            func_symtable,
        }
    }

    pub fn interpret(&mut self, code: &str) {
        let mut parser = Parser::new(code);

        let mut errors = Vec::<ChalError>::new();

        let mut span_lookup = BTreeMap::<u16, Rc<dyn Spanning>>::new();
        span_lookup.insert(0, parser.spanner());

        let mut main_start: usize = 0;

        let mut bytecode = vec![
            Bytecode::GetVar(0),
            Bytecode::Print,
            Bytecode::DeleteVar(0),
            Bytecode::Return,
        ];

        while !parser.is_empty() {
            match parser.advance() {
                Ok(NodeProg::FuncDef(node)) => {
                    let node_name = node.name().clone();
                    let bytecode_res = node.to_bytecode(
                        bytecode.len(),
                        &mut self.var_symtable,
                        &mut self.func_symtable,
                    );

                    let Ok(mut bytecode_raw) = bytecode_res else {
                        errors.push(bytecode_res.err().unwrap());
                        continue;
                    };

                    if node_name == "main" {
                        main_start = bytecode.len();
                    }

                    bytecode.append(&mut bytecode_raw);
                }
                Ok(NodeProg::VarDef(node)) => {
                    let bytecode_res = node.to_bytecode(
                        bytecode.len(),
                        &mut self.var_symtable,
                        &mut self.func_symtable,
                    );

                    let Ok(bytecode_raw) = bytecode_res else {
                        errors.push(bytecode_res.err().unwrap());
                        continue;
                    };

                    if let Err(err) = self.vm.execute(0, bytecode_raw) {
                        errors.push(err.into(&span_lookup));
                    }
                }
                Err(err) => errors.push(err),
            }
        }

        if errors.is_empty() {
            if let Err(err) = self.vm.execute(main_start, bytecode) {
                eprint!("{}", err.into(&span_lookup));
            }
        } else {
            for err in errors {
                eprint!("{}", err);
            }
        }
    }
}
