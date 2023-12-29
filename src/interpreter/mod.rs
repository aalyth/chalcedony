mod codegen;
use codegen::ToBytecode;

use crate::error::{ChalError, Span};
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
}

impl FuncAnnotation {
    pub fn new(args: Vec<(String, Type)>, ret_type: Type) -> Self {
        FuncAnnotation { args, ret_type }
    }
}

pub struct Chalcedony {
    pub vm: CVM,
    func_symtable: BTreeMap<String, FuncAnnotation>,
    func_lookup: BTreeMap<String, u64>,
}

impl Chalcedony {
    pub fn new() -> Self {
        let mut func_symtable = BTreeMap::<String, FuncAnnotation>::new();
        func_symtable.insert(
            "print".to_string(),
            FuncAnnotation::new(vec![(String::from("output"), Type::Str)], Type::Void),
        );

        let mut func_lookup = BTreeMap::<String, u64>::new();
        func_lookup.insert("print".to_string(), 0);

        Chalcedony {
            vm: CVM::new(),
            func_symtable,
            func_lookup,
        }
    }

    pub fn interpret(&mut self, code: &str) {
        let mut parser = Parser::new(code);

        let mut errors = Vec::<ChalError>::new();

        let mut span_lookup = BTreeMap::<u16, Rc<Span>>::new();
        span_lookup.insert(0, parser.span());

        let mut main_start: usize = 0;

        let mut bytecode = vec![
            Bytecode::OpGetVar as u8,
            111,
            117,
            116,
            112,
            117,
            116,
            0,
            Bytecode::OpPrint as u8,
            Bytecode::OpDeleteVar as u8,
            111,
            117,
            116,
            112,
            117,
            116,
            0,
            Bytecode::OpReturn as u8,
        ];

        while !parser.is_empty() {
            match parser.advance() {
                Ok(NodeProg::FuncDef(node)) => {
                    let node_name = node.name().clone();
                    let bytecode_res = node.to_bytecode(
                        bytecode.len(),
                        &mut self.func_symtable,
                        &mut self.func_lookup,
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
                        &mut self.func_symtable,
                        &mut self.func_lookup,
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
