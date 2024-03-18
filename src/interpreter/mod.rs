mod codegen;
use codegen::ToBytecode;

mod type_eval;

use crate::error::ChalError;
use crate::parser::ast::{NodeProg, NodeVarDef};
use crate::parser::Parser;
use crate::vm::Cvm;

use crate::common::{Bytecode, Type};

use std::cell::RefCell;
use std::rc::Rc;

/* ahash is the fastest hashing algorithm in terms of hashing strings (faster than fxhash) */
use ahash::AHashMap;

#[derive(Debug, Clone)]
struct ArgAnnotation {
    id: usize,
    ty: Type,
    name: String,
}

impl ArgAnnotation {
    fn new(id: usize, name: String, ty: Type) -> Self {
        ArgAnnotation { id, ty, name }
    }
}

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
    args: Vec<ArgAnnotation>,
    arg_lookup: AHashMap<String, ArgAnnotation>,
    ret_type: Type,
}

impl FuncAnnotation {
    fn new(id: usize, args: Vec<ArgAnnotation>, ret_type: Type) -> Self {
        let mut arg_lookup = AHashMap::<String, ArgAnnotation>::new();
        for arg in args.clone() {
            arg_lookup.insert(arg.name.clone(), arg);
        }
        FuncAnnotation {
            id,
            args,
            arg_lookup,
            ret_type,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct WhileScope {
    current_length: usize,
    unfinished_breaks: Vec<usize>,
}

trait InterpreterVisitor {
    fn interpret_node(&mut self, _: NodeProg) -> Result<(), ChalError>;
}

#[derive(Default)]
pub struct Chalcedony {
    /* The virtual machine used to execute the resulting bytecode*/
    vm: Cvm,

    /* Used to keep track of the globally declared variables */
    globals: AHashMap<String, VarAnnotation>,

    /* Used to keep track of the functions inside the program */
    func_symtable: AHashMap<String, Rc<FuncAnnotation>>,

    /* Contains the necessary function information used while parsing statements
     * inside a function's scope */
    current_func: Option<Rc<FuncAnnotation>>,

    /* Contains the necessary information in order to implement control flow logic in while loops */
    current_while: Option<WhileScope>,

    /* Keeps track of the current scope's local variables */
    locals: RefCell<AHashMap<String, VarAnnotation>>,

    /* Keeps track whether the currently compiled scope is a statement */
    inside_stmnt: bool,

    /* Whether the interpreter has failed */
    failed: bool,
}

impl InterpreterVisitor for Chalcedony {
    fn interpret_node(&mut self, node: NodeProg) -> Result<(), ChalError> {
        let bytecode = node.to_bytecode(self)?;
        print!("BYTECODE: {:#?}", bytecode);
        /* this is so all of the errors in the code are displayed */
        if !self.failed {
            self.vm.execute(bytecode);
        }
        Ok(())
    }
}

impl Chalcedony {
    pub fn new() -> Self {
        let print_args = vec![ArgAnnotation::new(0, "output".to_string(), Type::Any)];

        let assert_args = vec![
            ArgAnnotation::new(0, "exp".to_string(), Type::Any),
            ArgAnnotation::new(1, "recv".to_string(), Type::Any),
        ];

        let mut func_symtable = AHashMap::<String, Rc<FuncAnnotation>>::new();
        func_symtable.insert(
            "print".to_string(),
            Rc::new(FuncAnnotation::new(0, print_args, Type::Void)),
        );

        func_symtable.insert(
            "assert".to_string(),
            Rc::new(FuncAnnotation::new(1, assert_args, Type::Void)),
        );

        let mut vm = Cvm::new();

        let mut builtins = Vec::<Vec<Bytecode>>::new();
        let print = vec![
            Bytecode::CreateFunc(1),
            Bytecode::GetArg(0),
            Bytecode::Print,
            Bytecode::ReturnVoid,
        ];
        let assert = vec![
            Bytecode::CreateFunc(2),
            Bytecode::Assert,
            Bytecode::ReturnVoid,
        ];

        builtins.push(print);
        builtins.push(assert);
        for builtin in builtins {
            vm.execute(builtin);
        }

        Chalcedony {
            vm,
            globals: AHashMap::new(),
            func_symtable,
            current_func: None,
            current_while: None,
            locals: RefCell::new(AHashMap::default()),
            inside_stmnt: false,
            failed: false,
        }
    }

    pub fn interpret(&mut self, code: &str) {
        let mut parser = Parser::new(code);

        let mut errors = Vec::<ChalError>::new();

        self.failed = false;
        while !parser.is_empty() {
            match parser.advance() {
                Ok(node) => {
                    if let Err(err) = self.interpret_node(node) {
                        self.failed = true;
                        errors.push(err);
                    }
                }
                Err(err) => {
                    self.failed = true;
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

    fn create_function(&mut self, name: String, args: Vec<ArgAnnotation>, ret: Type) {
        let result = Rc::new(FuncAnnotation::new(self.func_symtable.len(), args, ret));
        self.func_symtable.insert(name, result.clone());
        self.current_func = Some(result);
        self.locals = RefCell::new(AHashMap::new());
    }

    fn get_global_id(&mut self, node: &NodeVarDef) -> usize {
        if let Some(var) = self.globals.get(&node.name) {
            return var.id;
        }
        self.globals.insert(
            node.name.clone(),
            VarAnnotation::new(self.globals.len(), node.ty.clone()),
        );
        self.globals.len() - 1
    }

    fn get_local_id(&mut self, node: &NodeVarDef) -> usize {
        if let Some(var) = self.locals.borrow().get(&node.name) {
            return var.id;
        }

        let next_id = self.locals.borrow().len();
        self.locals.borrow_mut().insert(
            node.name.clone(),
            VarAnnotation::new(next_id, node.ty.clone()),
        );
        next_id
    }
}
