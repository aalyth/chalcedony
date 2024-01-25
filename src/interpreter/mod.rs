mod codegen;
use codegen::*;

use crate::error::{span::Spanning, ChalError};
use crate::lexer::Type;
use crate::parser::ast::NodeProg;
use crate::parser::Parser;
use crate::vm::CVM;

use crate::utils::Bytecode;

use std::cell::RefCell;
use std::rc::Rc;

// ahash is the fastest hashing algorithm in terms of hashing strings (faster than fxhash)
use ahash::AHashMap;

#[derive(Debug, Clone)]
pub struct FuncAnnotation {
    id: usize,
    args: Vec<(String, Type)>,
    locals_symtable: AHashMap<String, usize>,
    locals_id_counter: usize,
    ret_type: Type,
}

impl FuncAnnotation {
    fn new(
        id: usize,
        args: Vec<(String, Type)>,
        ret_type: Type,
        locals: AHashMap<String, usize>,
    ) -> Self {
        FuncAnnotation {
            id,
            args,
            locals_id_counter: locals.len(),
            locals_symtable: locals,
            ret_type,
        }
    }

    fn get_arg(&self, arg_name: &String) -> Option<usize> {
        let mut id = 0 as usize;
        for arg in &self.args {
            if &arg.0 == arg_name {
                return Some(id);
            }
            id += 1;
        }
        None
    }
}

trait InterpreterVisitor {
    fn interpret_node(&mut self, _: NodeProg) -> Result<(), ChalError>;
}

pub struct Chalcedony {
    vm: CVM,
    globals_symtable: AHashMap<String, usize>,
    func_symtable: AHashMap<String, Rc<RefCell<FuncAnnotation>>>,

    // these members are used to track state between the node parsings into bytecode
    current_func: Option<Rc<RefCell<FuncAnnotation>>>,
    globals_id_counter: usize,
    func_id_counter: usize,
}

impl InterpreterVisitor for Chalcedony {
    fn interpret_node(&mut self, node: NodeProg) -> Result<(), ChalError> {
        let bytecode = node.to_bytecode(self)?;
        // println!("BYTECODE: {:#?}\n", bytecode);
        self.vm.execute(&bytecode);
        Ok(())
    }
}

impl Chalcedony {
    pub fn new() -> Self {
        let mut func_symtable = AHashMap::<String, Rc<RefCell<FuncAnnotation>>>::new();
        func_symtable.insert(
            "print".to_string(),
            Rc::new(RefCell::new(FuncAnnotation::new(
                0,
                vec![(String::from("output"), Type::Str)],
                Type::Void,
                AHashMap::new(),
            ))),
        );

        let mut globals_symtable = AHashMap::<String, usize>::new();
        globals_symtable.insert("__name__".to_string(), 0);

        Chalcedony {
            vm: CVM::new(),
            globals_symtable,
            func_symtable,

            current_func: None,
            globals_id_counter: 1,
            func_id_counter: 1,
        }
    }

    pub fn interpret(&mut self, code: &str) {
        let mut parser = Parser::new(code);

        let mut errors = Vec::<ChalError>::new();

        let mut span_lookup = AHashMap::<u16, Rc<dyn Spanning>>::new();
        span_lookup.insert(0, parser.spanner());

        let bytecode = vec![
            Bytecode::CreateFunc(1, 0),
            Bytecode::GetArg(0),
            Bytecode::Print,
            Bytecode::Return,
        ];
        self.vm.execute(&bytecode);

        while !parser.is_empty() {
            match parser.advance() {
                Ok(node) => {
                    if let Err(err) = self.interpret_node(node) {
                        eprint!("{}", err);
                        return;
                    }
                }
                Err(err) => errors.push(err),
            }
        }

        if !errors.is_empty() {
            for err in errors {
                eprint!("{}", err);
            }
        }
    }

    fn create_function(&mut self, name: String, args: Vec<(String, Type)>, ret: Type) {
        let result = Rc::new(RefCell::new(FuncAnnotation::new(
            self.func_id_counter,
            args,
            ret,
            AHashMap::new(),
        )));
        self.func_id_counter += 1;
        self.func_symtable.insert(name, result.clone());
        self.current_func = Some(result);
    }

    fn get_global_id(&mut self, var_name: &String) -> usize {
        if let Some(id) = self.globals_symtable.get(var_name) {
            return *id;
        }
        self.globals_symtable
            .insert(var_name.clone(), self.globals_id_counter);
        self.globals_id_counter += 1;
        self.globals_id_counter - 1
    }

    // NOTE: must only be called inside function context
    fn get_local_id(&mut self, var_name: &String) -> usize {
        let Some(current_func) = self.current_func.clone() else {
            panic!("TOOD: check if this is ok");
        };
        let mut current_func = current_func.borrow_mut();
        if let Some(id) = current_func.locals_symtable.get(var_name) {
            return *id;
        }
        let id_counter = current_func.locals_id_counter;
        current_func
            .locals_symtable
            .insert(var_name.clone(), id_counter);
        current_func.locals_id_counter += 1;
        current_func.locals_id_counter - 1
    }
}
