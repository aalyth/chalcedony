//! The final stage of the interpreting process, responsible for transforming
//! the parsed `Abstract Syntax Tree (AST)` into a stream of bytecode
//! unstructions, executed by the `Chalcedony Virtual Machine (CVM)`.

mod codegen;
pub use codegen::ToBytecode;

mod type_eval;

use crate::error::ChalError;
use crate::parser::ast::{NodeProg, NodeVarDef};
use crate::parser::Parser;
use crate::vm::Cvm;

use crate::common::{Bytecode, Type};

use std::iter::zip;
use std::rc::Rc;

/* ahash is the fastest hashing algorithm in terms of hashing strings */
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

#[derive(Debug, Clone, Copy)]
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
    is_unsafe: bool,
    id: usize,
    args: Vec<ArgAnnotation>,
    arg_lookup: AHashMap<String, ArgAnnotation>,
    ret_type: Type,
}

impl FuncAnnotation {
    fn new(id: usize, args: Vec<ArgAnnotation>, ret_type: Type, is_unsafe: bool) -> Self {
        let mut arg_lookup = AHashMap::<String, ArgAnnotation>::new();
        for arg in args.clone() {
            arg_lookup.insert(arg.name.clone(), arg);
        }
        FuncAnnotation {
            is_unsafe,
            id,
            args,
            arg_lookup,
            ret_type,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct LoopScope {
    current_length: usize,
    unfinished_breaks: Vec<usize>,
}

#[derive(Default, PartialEq)]
pub enum SafetyScope {
    #[default]
    Normal,
    Try,
    Catch,
}

/// The structure representing the interpreter, used to compile the received
/// `AST` into a stream of `Bytecode` instructions and respectively interpret
/// the instructions via the Chalcedony Virtual Machine (CVM).
#[derive(Default)]
pub struct Chalcedony {
    // The virtual machine used to execute the compiled bytecode instructions.
    vm: Cvm,

    // Used to keep track of the globally declared variables.
    globals: AHashMap<String, VarAnnotation>,

    // Used to keep track of the functions inside the program.
    func_symtable: AHashMap<String, Vec<Rc<FuncAnnotation>>>,
    func_id_counter: usize,

    // Contains the necessary information about the current function if inside a
    // function scope.
    current_func: Option<Rc<FuncAnnotation>>,

    // Contains the information about the current scope's "safety" type, i.e.
    // whether it is a `try` block, `catch` block, or a normal block.
    safety_scope: SafetyScope,

    // Contains the necessary information in order to implement control flow
    // logic in loop scopes.
    current_loop: Option<LoopScope>,

    // Keeps track of the current scope's local variables.
    locals: AHashMap<String, VarAnnotation>,

    // Keeps track whether the currently compiled scope is a statement - used
    // to perform checks such as wether a `void` function is used inside an
    // expression.
    inside_stmnt: bool,

    // Whether the interpreter has encountered an error, so even if an error is
    // encountered the rest of the script is still statically checked.
    failed: bool,
}

trait InterpreterVisitor {
    fn interpret_node(&mut self, _: NodeProg) -> Result<(), ChalError>;
}

impl InterpreterVisitor for Chalcedony {
    fn interpret_node(&mut self, node: NodeProg) -> Result<(), ChalError> {
        let bytecode = node.to_bytecode(self)?;
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

        let ftoi_args = vec![ArgAnnotation::new(0, "val".to_string(), Type::Float)];

        let mut func_symtable = AHashMap::new();
        func_symtable.insert(
            "print".to_string(),
            vec![Rc::new(FuncAnnotation::new(
                0,
                print_args,
                Type::Void,
                false,
            ))],
        );

        func_symtable.insert(
            "assert".to_string(),
            vec![Rc::new(FuncAnnotation::new(
                1,
                assert_args,
                Type::Void,
                false,
            ))],
        );

        func_symtable.insert(
            "ftoi".to_string(),
            vec![Rc::new(FuncAnnotation::new(2, ftoi_args, Type::Int, false))],
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
        let ftoi = vec![
            Bytecode::CreateFunc(1),
            Bytecode::GetArg(0),
            Bytecode::CastI,
            Bytecode::Return,
        ];

        builtins.push(print);
        builtins.push(assert);
        builtins.push(ftoi);
        for builtin in builtins {
            vm.execute(builtin);
        }

        Chalcedony {
            vm,
            globals: AHashMap::new(),
            func_symtable,
            func_id_counter: 3,
            current_func: None,
            safety_scope: SafetyScope::Normal,
            current_loop: None,
            locals: AHashMap::default(),
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

    // Used for tests to get the id of the next function so even if the standard
    // library changes, the proper function id is used.
    pub fn get_next_func_id(&self) -> usize {
        self.func_id_counter
    }

    pub fn execute(&mut self, code: Vec<Bytecode>) {
        self.vm.execute(code)
    }

    /* builds the function and sets the currennt function scope */
    fn create_function(&mut self, name: String, args: Vec<ArgAnnotation>, ret: Type) {
        let func = Rc::new(FuncAnnotation::new(
            self.func_id_counter,
            args,
            ret,
            name.ends_with('!'),
        ));
        self.func_id_counter += 1;

        self.current_func = Some(func.clone());
        self.locals = AHashMap::new();

        match self.func_symtable.get_mut(&name) {
            Some(func_bucket) => func_bucket.push(func),
            None => {
                self.func_symtable.insert(name, vec![func]);
            }
        }
    }

    /* receives the proper overloaded function annotation from the passed argument types */
    fn get_function(&self, name: &str, arg_types: &Vec<Type>) -> Option<&FuncAnnotation> {
        let func_bucket = self.func_symtable.get(name)?;
        /* inlining the clippy suggestion does not help due to the Rc inside */
        #[allow(clippy::manual_find)]
        for annotation in func_bucket {
            if valid_annotation(&annotation.args, arg_types) {
                return Some(annotation);
            }
        }
        None
    }

    /* retrieves the global variable's id and creates it if it does not exist */
    fn get_global_id(&mut self, node: &NodeVarDef) -> usize {
        if let Some(var) = self.globals.get(&node.name) {
            return var.id;
        }
        self.globals.insert(
            node.name.clone(),
            VarAnnotation::new(self.globals.len(), node.ty),
        );
        self.globals.len() - 1
    }

    /* retrieves the local variable's id and creates it if it does not exist */
    fn get_local_id(&mut self, node: &NodeVarDef) -> usize {
        self.get_local_id_internal(&node.name, node.ty)
    }

    fn get_local_id_internal(&mut self, name: &str, ty: Type) -> usize {
        if let Some(var) = self.locals.get(name) {
            return var.id;
        }

        let next_id = self.locals.len();
        self.locals
            .insert(name.to_string(), VarAnnotation::new(next_id, ty));
        next_id
    }

    fn remove_local(&mut self, name: &str) {
        self.locals.remove(name);
    }
}

/* checks whether the passed arguments match the function annotation */
fn valid_annotation(args: &Vec<ArgAnnotation>, received: &Vec<Type>) -> bool {
    if args.len() != received.len() {
        return false;
    }

    for (arg, recv) in zip(args, received) {
        if !arg.ty.soft_eq(recv) {
            return false;
        }
    }

    true
}
