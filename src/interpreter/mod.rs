mod codegen;
use codegen::ToBytecode;

use crate::error::ChalError;
use crate::lexer::Type;
use crate::parser::Parser;
use crate::vm::CVM;

use crate::utils::Bytecode;

use std::collections::HashMap;

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
    // var_symtable: HashMap<String, Type>,
    func_symtable: HashMap<String, FuncAnnotation>,
}

impl Chalcedony {
    pub fn new() -> Self {
        let mut func_symtable = HashMap::<String, FuncAnnotation>::new();
        func_symtable.insert(
            String::from("print"),
            FuncAnnotation::new(vec![(String::from("output"), Type::Str)], Type::Void),
        );
        Chalcedony {
            vm: CVM::new(),
            func_symtable,
        }
    }

    pub fn interpret(&mut self, code: &str) {
        let mut parser = Parser::new(code);

        let mut errors = Vec::<ChalError>::new();

        while !parser.is_empty() {
            match parser.advance() {
                Ok(node) => {
                    let bytecode_res = node.to_bytecode(&mut self.func_symtable);
                    let Ok(bytecode) = bytecode_res else {
                        errors.push(bytecode_res.err().unwrap());
                        continue;
                    };

                    if let Err(err) = self.vm.execute(&bytecode) {
                        // TODO: make it throw proper error
                        println!("RUNTIME ERROR: {:?}\n", err);
                    }
                }
                Err(err) => errors.push(err),
            }
        }

        let mut main_call = Vec::<u8>::new();
        main_call.push(Bytecode::OpCallFunc as u8);
        main_call.extend_from_slice("main".as_bytes());
        main_call.push(0);
        if let Err(err) = self.vm.execute(&main_call) {
            println!("RUNTIME ERROR: {:?}\n", err);
        }

        if !errors.is_empty() {
            for err in errors {
                println!("{}", err);
            }
        }
    }
}
