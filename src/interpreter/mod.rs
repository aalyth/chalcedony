mod codegen;
use codegen::ToBytecode;

mod type_eval;

use crate::error::{span::Spanning, ChalError};
use crate::parser::ast::{NodeProg, NodeVarDef};
use crate::parser::Parser;
use crate::vm::CVM;

use crate::common::{Bytecode, Type};

use std::cell::RefCell;
use std::rc::Rc;

// ahash is the fastest hashing algorithm in terms of hashing strings (faster than fxhash)
use ahash::AHashMap;

#[derive(Debug, Clone)]
struct VarAnnotation {
    id: usize,
    ty: Type,
}

impl VarAnnotation {
    fn new(id: usize, ty: Type) -> Self {
        VarAnnotation { id, ty }
    }
}

#[derive(Debug, Clone)]
pub struct FuncAnnotation {
    id: usize,
    args: AHashMap<String, VarAnnotation>,
    locals: AHashMap<String, VarAnnotation>,
    locals_id_counter: usize,
    ret_type: Type,
}

impl FuncAnnotation {
    fn new(
        id: usize,
        args: AHashMap<String, VarAnnotation>,
        ret_type: Type,
        locals: AHashMap<String, VarAnnotation>,
    ) -> Self {
        FuncAnnotation {
            id,
            args,
            locals_id_counter: locals.len(),
            locals,
            ret_type,
        }
    }
}

trait InterpreterVisitor {
    fn interpret_node(&mut self, _: NodeProg) -> Result<(), ChalError>;
}

pub struct Chalcedony {
    vm: CVM,
    globals: AHashMap<String, VarAnnotation>,
    func_symtable: AHashMap<String, Rc<RefCell<FuncAnnotation>>>,

    // these members are used to track state between the node parsings into bytecode
    current_func: Option<Rc<RefCell<FuncAnnotation>>>,
    globals_id_counter: usize,
    func_id_counter: usize,

    is_expr_scope: bool,
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
        let mut print_args = AHashMap::new();
        print_args.insert("output".to_string(), VarAnnotation::new(0, Type::Str));

        let mut func_symtable = AHashMap::<String, Rc<RefCell<FuncAnnotation>>>::new();
        func_symtable.insert(
            "print".to_string(),
            Rc::new(RefCell::new(FuncAnnotation::new(
                1,
                print_args,
                Type::Void,
                AHashMap::new(),
            ))),
        );

        Chalcedony {
            vm: CVM::new(),
            globals: AHashMap::new(),
            func_symtable,

            current_func: None,
            globals_id_counter: 0,
            func_id_counter: 2,

            is_expr_scope: false,
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

        let mut failed = false;
        while !parser.is_empty() {
            match parser.advance() {
                Ok(node) if !failed => {
                    if let Err(err) = self.interpret_node(node) {
                        eprint!("{}", err);
                        return;
                    }
                }
                Ok(_) => {}
                Err(err) => {
                    failed = true;
                    errors.push(err);
                }
            }
        }

        if !errors.is_empty() {
            for err in errors {
                eprint!("{}", err);
            }
        }
    }

    fn create_function(&mut self, name: String, args: AHashMap<String, VarAnnotation>, ret: Type) {
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

    fn get_global_id(&mut self, node: &NodeVarDef) -> usize {
        if let Some(var) = self.globals.get(&node.name) {
            return var.id;
        }
        self.globals.insert(
            node.name.clone(),
            VarAnnotation::new(self.globals_id_counter, node.ty),
        );
        self.globals_id_counter += 1;
        self.globals_id_counter - 1
    }

    // NOTE: must only be called inside function context
    fn get_local_id(&mut self, node: &NodeVarDef) -> usize {
        let current_func = self
            .current_func
            .clone()
            .expect("getting a local variable id is only allowed inside function scope");

        let mut current_func = current_func.borrow_mut();
        if let Some(var) = current_func.locals.get(&node.name) {
            return var.id;
        }

        let id_counter = current_func.locals_id_counter;
        current_func
            .locals
            .insert(node.name.clone(), VarAnnotation::new(id_counter, node.ty));

        current_func.locals_id_counter += 1;
        current_func.locals_id_counter - 1
    }
}
