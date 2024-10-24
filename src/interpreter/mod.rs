//! The final stage of the interpreting process, responsible for transforming
//! the parsed `Abstract Syntax Tree (AST)` into a stream of bytecode
//! unstructions, executed by the `Chalcedony Virtual Machine (CVM)`.

mod codegen;
pub use codegen::ToBytecode;

mod type_eval;

use crate::error::{err, span::Span, ChalError, CompileError, CompileErrorKind};
use crate::parser::ast::{NodeFuncDef, NodeProg, NodeVarDef};
use crate::parser::Parser;
use crate::vm::Cvm;

use crate::common::{Bytecode, Type};

use std::collections::VecDeque;
use std::iter::zip;
use std::path::{Path, PathBuf};
use std::rc::Rc;

/* ahash is the fastest hashing algorithm in terms of hashing strings */
use ahash::{AHashMap, AHashSet};

#[derive(Debug, Clone, PartialEq)]
pub struct ArgAnnotation {
    id: usize,
    ty: Type,
    name: String,
}

impl ArgAnnotation {
    pub fn new(id: usize, name: String, ty: Type) -> Self {
        ArgAnnotation { id, ty, name }
    }
}

#[derive(Debug, Clone)]
struct VarAnnotation {
    id: usize,
    ty: Type,
    is_const: bool,
}

impl VarAnnotation {
    fn new(id: usize, ty: Type, is_const: bool) -> Self {
        VarAnnotation { id, ty, is_const }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncAnnotation {
    is_unsafe: bool,
    id: usize,
    args: Vec<ArgAnnotation>,
    arg_lookup: AHashMap<String, ArgAnnotation>,
    ret_type: Type,
}

impl FuncAnnotation {
    pub fn new(id: usize, args: Vec<ArgAnnotation>, ret_type: Type, is_unsafe: bool) -> Self {
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

#[derive(Debug, Clone)]
pub struct BuiltinAnnotation {
    pub args: Vec<ArgAnnotation>,
    pub ret_type: Type,
    pub bytecode: Vec<Bytecode>,
}

#[derive(Default)]
struct RawFuncAnnotation {
    args: VecDeque<ArgAnnotation>,
    ret_type: Type,
    bytecode: Vec<Bytecode>,
}

impl From<BuiltinAnnotation> for RawFuncAnnotation {
    fn from(value: BuiltinAnnotation) -> Self {
        RawFuncAnnotation {
            args: value.args.into(),
            ret_type: value.ret_type,
            bytecode: value.bytecode,
        }
    }
}

impl From<&FuncAnnotation> for RawFuncAnnotation {
    fn from(value: &FuncAnnotation) -> Self {
        RawFuncAnnotation {
            args: value.args.clone().into(),
            ret_type: value.ret_type.clone(),
            bytecode: vec![Bytecode::CallFunc(value.id)],
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
    /// Unsafe oprations are not checked against any rules. Used by default in
    /// the global interpreter space.
    #[default]
    Normal,
    /// Any type of unsafe oprations are allowed and guarded inside a try block.
    Guarded,
    /// Unsafe operations are not allowed in any shape or form. Used in the
    /// scope of safe functions and catch blocks.
    Safe,
}

#[derive(Default, Clone, Debug, Copy)]
pub enum ScriptType {
    #[default]
    Main,
    Imported,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MemberAnnotation {
    pub id: usize,
    pub name: String,
    pub ty: Type,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ClassNamespace {
    // pub members: AHashMap<String, MemberAnnotation>,
    pub members: Vec<MemberAnnotation>,
    pub methods: AHashMap<String, Vec<Rc<FuncAnnotation>>>,
}

impl ClassNamespace {
    fn get_member(&self, name: &str) -> Option<&MemberAnnotation> {
        self.members.iter().find(|&member| member.name == name)
    }
}

/// The structure representing the interpreter, used to compile the received
/// `AST` into a stream of `Bytecode` instructions and respectively interpret
/// the instructions via the Chalcedony Virtual Machine (CVM).
#[derive(Default)]
pub struct Chalcedony {
    // The virtual machine used to execute the compiled bytecode instructions.
    vm: Cvm,

    // Used for setting the `__name__` variable and managing relative imports.
    script_type: ScriptType,
    current_path: PathBuf,

    // Used to resolve issues around multiple imports of the same script
    // (similar to the Multiple Inheritance problem).
    imported_scripts: AHashSet<PathBuf>,

    // Used to keep track of the globally declared variables.
    globals: AHashMap<String, VarAnnotation>,
    // Used to keep track of internal for loop iterators.
    globals_id_counter: usize,

    // A lookup for the builtin pre-compiled functions. The first lookup is for
    // the namespace and the second one is for the actual functions inside.
    builtins: AHashMap<String, AHashMap<String, Vec<BuiltinAnnotation>>>,

    // Used to keep track of the functions inside the program.
    func_symtable: AHashMap<String, Vec<Rc<FuncAnnotation>>>,

    // Since functions and methods fundamentally are compiled to the bytecode
    // instruction `CallFunc(usize)`, this variable keeps track of the next id
    // across each function and method definition.
    func_id_counter: usize,

    // Currently namespaces only refer to classes and contain their
    // corresponding methods' definitions. This design approach is used to serve
    // as a base for future implementation a complete namespace system.
    namespaces: AHashMap<String, ClassNamespace>,

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
        let mut res = Chalcedony {
            vm: Cvm::new(),
            script_type: ScriptType::Main,
            current_path: PathBuf::new(),
            imported_scripts: AHashSet::new(),
            globals: AHashMap::new(),
            globals_id_counter: 1, // the `__main__` constant
            builtins: get_builtins(),
            func_symtable: AHashMap::new(),
            func_id_counter: 0,
            namespaces: AHashMap::new(),
            current_func: None,
            safety_scope: SafetyScope::Normal,
            current_loop: None,
            locals: AHashMap::default(),
            inside_stmnt: false,
            failed: false,
        };

        let script_const_id = res.get_global_id_internal("__name__", Type::Str, true);
        res.vm.execute(vec![
            Bytecode::ConstS("__main__".to_string().into()),
            Bytecode::SetGlobal(script_const_id),
        ]);

        res
    }

    pub fn interpret(&mut self, code: &str) {
        let mut parser = Parser::new(code);
        self.interpret_internal(&mut parser);
    }

    pub fn interpret_script(&mut self, filename: String) {
        let Some(mut parser) = Parser::from_file(filename.clone()) else {
            eprintln!(
                "{}",
                err(&format!("could not open the script `{}`", filename))
            );
            std::process::exit(1);
        };
        self.current_path = PathBuf::from(filename)
            .parent()
            .unwrap_or(Path::new(""))
            .to_owned();
        self.interpret_internal(&mut parser);
    }

    fn interpret_internal(&mut self, parser: &mut Parser) {
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

    /* builds the function and sets the currennt function scope */
    fn create_function(&mut self, node: &NodeFuncDef, args: Vec<ArgAnnotation>) {
        let func = Rc::new(FuncAnnotation::new(
            self.func_id_counter,
            args,
            node.ret_type.clone(),
            node.name.ends_with('!'),
        ));
        self.func_id_counter += 1;

        self.current_func = Some(func.clone());
        self.locals = AHashMap::new();

        let mut func_symtable = &mut self.func_symtable;
        if let Some(class) = node.namespace.clone() {
            func_symtable = &mut self
                .namespaces
                .get_mut(&class)
                .expect("classes should create their own namespaces")
                .methods;
        }

        match func_symtable.get_mut(&node.name) {
            Some(func_bucket) => func_bucket.push(func),
            None => {
                func_symtable.insert(node.name.clone(), vec![func]);
            }
        }
    }

    fn get_builtin(
        &self,
        name: &str,
        arg_types: &VecDeque<Type>,
        namespace: Option<&str>,
    ) -> Option<BuiltinAnnotation> {
        let namespace = namespace.unwrap_or("Global");
        let bucket = self.builtins.get(namespace)?.get(name)?;
        /* inlining the clippy suggestion does not help due to the Rc inside */
        #[allow(clippy::manual_find)]
        for annotation in bucket {
            if valid_annotation(&annotation.args, arg_types) {
                if namespace == "List" {
                    match name {
                        "insert!" | "push_front" | "push_back" | "set!" => {
                            let Type::List(inner_ty) = arg_types.front().unwrap() else {
                                panic!("improper func arg checks")
                            };
                            let val_ty = arg_types.get(1).unwrap();
                            if &**inner_ty != val_ty {
                                return None;
                            }
                        }
                        "remove!" | "pop_front!" | "pop_back!" | "get!" | "__next__!" => {
                            let Type::List(inner_ty) = arg_types.front().unwrap() else {
                                panic!("improper func arg checks")
                            };
                            let res = BuiltinAnnotation {
                                args: annotation.args.clone(),
                                ret_type: *inner_ty.clone(),
                                bytecode: annotation.bytecode.clone(),
                            };
                            return Some(res);
                        }
                        "__iter__" | "copy" => {
                            return Some(BuiltinAnnotation {
                                args: annotation.args.clone(),
                                ret_type: arg_types.front().unwrap().clone(),
                                bytecode: annotation.bytecode.clone(),
                            })
                        }
                        _ => {}
                    };
                }

                if namespace == "General" && name == "copy" {
                    return Some(BuiltinAnnotation {
                        args: annotation.args.clone(),
                        ret_type: arg_types.front().unwrap().clone(),
                        bytecode: annotation.bytecode.clone(),
                    });
                }
                return Some(annotation.clone());
            }
        }
        None
    }

    /* receives the proper overloaded function annotation from the passed argument types */
    fn get_function(
        &self,
        name: &str,
        arg_types: &VecDeque<Type>,
        namespace: Option<&String>,
    ) -> Option<&FuncAnnotation> {
        let mut func_symtable = &self.func_symtable;
        if let Some(class) = namespace {
            func_symtable = &self.namespaces.get(class)?.methods;
        }

        let func_bucket = func_symtable.get(name)?;
        /* inlining the clippy suggestion does not help due to the Rc inside */
        #[allow(clippy::manual_find)]
        for annotation in func_bucket {
            if valid_annotation(&annotation.args, arg_types) {
                return Some(annotation);
            }
        }
        None
    }

    // Retrieves the annotation of a builtin or a function.
    fn get_function_universal(
        &self,
        name: &str,
        arg_types: &VecDeque<Type>,
        namespace: Option<&String>,
    ) -> Option<RawFuncAnnotation> {
        if let Some(ann) = self.get_builtin(name, arg_types, namespace.map(|val| val.as_str())) {
            Some(ann.into())
        } else {
            self.get_function(name, arg_types, namespace)
                .map(|ann| ann.into())
        }
    }

    /* retrieves the global variable's id and creates it if it does not exist */
    fn get_global_id(&mut self, node: &NodeVarDef) -> usize {
        self.get_global_id_internal(&node.name, node.ty.clone(), node.is_const)
    }

    fn get_global_id_internal(&mut self, name: &str, ty: Type, is_const: bool) -> usize {
        if let Some(var) = self.globals.get(name) {
            return var.id;
        }
        self.globals.insert(
            name.to_string(),
            VarAnnotation::new(self.globals_id_counter, ty, is_const),
        );
        self.globals_id_counter += 1;
        self.globals_id_counter - 1
    }

    /* retrieves the local variable's id and creates it if it does not exist */
    fn get_local_id(&mut self, node: &NodeVarDef) -> usize {
        self.get_local_id_internal(&node.name, node.ty.clone(), node.is_const)
    }

    fn get_local_id_internal(&mut self, name: &str, ty: Type, is_const: bool) -> usize {
        let mut arg_count = 0;
        if let Some(func) = &self.current_func {
            arg_count = func.args.len();
        }
        if let Some(var) = self.locals.get(name) {
            return var.id;
        }

        let next_id = self.locals.len() + arg_count;
        self.locals
            .insert(name.to_string(), VarAnnotation::new(next_id, ty, is_const));
        next_id
    }

    fn remove_local(&mut self, name: &str) {
        self.locals.remove(name);
    }

    fn verify_type(&self, ty: &Type, span: &Span) -> Result<(), ChalError> {
        match ty {
            Type::Custom(ty) => {
                if !(self.builtins.contains_key(&**ty) || self.namespaces.contains_key(&**ty)) {
                    return Err(CompileError::new(
                        CompileErrorKind::TypeDoesNotExits(*ty.clone()),
                        span.clone(),
                    )
                    .into());
                };
                Ok(())
            }
            Type::Exception => Err(CompileError::new(
                CompileErrorKind::ExceptionTyOutsideCatch,
                span.clone(),
            )
            .into()),
            _ => Ok(()),
        }
    }
}

/// Wrapper functions, used for tests.
#[cfg(feature = "testing")]
impl Chalcedony {
    // Used for tests to get the id of the next function so even if the standard
    // library changes, the proper function id is used.
    pub fn get_next_func_id(&self) -> usize {
        self.func_id_counter
    }

    pub fn execute(&mut self, code: Vec<Bytecode>) {
        self.vm.execute(code)
    }

    pub fn get_namespace(&self, name: &str) -> Option<&ClassNamespace> {
        self.namespaces.get(name)
    }
}

macro_rules! builtin_map {
    ($($key:expr => $value:expr),* $(,)?) => {{
        AHashMap::from([$(($key.to_string(), $value),)*])
    }};
}

// Used to keep the code for initializing the interpreter clean.
#[inline(always)]
fn get_builtins() -> AHashMap<String, AHashMap<String, Vec<BuiltinAnnotation>>> {
    let print = BuiltinAnnotation {
        args: vec![ArgAnnotation::new(0, "val".to_string(), Type::Any)],
        ret_type: Type::Void,
        bytecode: vec![Bytecode::Print],
    };

    let assert = BuiltinAnnotation {
        args: vec![ArgAnnotation::new(0, "expr".to_string(), Type::Bool)],
        ret_type: Type::Void,
        bytecode: vec![Bytecode::Assert],
    };

    let utoi = BuiltinAnnotation {
        args: vec![ArgAnnotation::new(0, "val".to_string(), Type::Uint)],
        ret_type: Type::Int,
        bytecode: vec![Bytecode::CastI],
    };
    let ftoi = BuiltinAnnotation {
        args: vec![ArgAnnotation::new(0, "val".to_string(), Type::Float)],
        ret_type: Type::Int,
        bytecode: vec![Bytecode::CastI],
    };

    let itou = BuiltinAnnotation {
        args: vec![ArgAnnotation::new(0, "val".to_string(), Type::Int)],
        ret_type: Type::Uint,
        bytecode: vec![Bytecode::CastU],
    };
    let ftou = BuiltinAnnotation {
        args: vec![ArgAnnotation::new(0, "val".to_string(), Type::Float)],
        ret_type: Type::Uint,
        bytecode: vec![Bytecode::CastU],
    };

    let itof = BuiltinAnnotation {
        args: vec![ArgAnnotation::new(0, "val".to_string(), Type::Int)],
        ret_type: Type::Float,
        bytecode: vec![Bytecode::CastF],
    };
    let utof = BuiltinAnnotation {
        args: vec![ArgAnnotation::new(0, "val".to_string(), Type::Uint)],
        ret_type: Type::Float,
        bytecode: vec![Bytecode::CastF],
    };

    let len_str = BuiltinAnnotation {
        args: vec![ArgAnnotation::new(0, "val".to_string(), Type::Str)],
        ret_type: Type::Uint,
        bytecode: vec![Bytecode::Len],
    };
    let len_list = BuiltinAnnotation {
        args: vec![ArgAnnotation::new(
            0,
            "val".to_string(),
            Type::List(Box::new(Type::Any)),
        )],
        ret_type: Type::Uint,
        bytecode: vec![Bytecode::Len],
    };
    let copy = BuiltinAnnotation {
        args: vec![ArgAnnotation::new(0, "val".to_string(), Type::Any)],
        ret_type: Type::Any,
        bytecode: vec![Bytecode::Copy],
    };

    // List::insert!(), List::push_back(), List::push_front()
    let insert = BuiltinAnnotation {
        args: vec![
            ArgAnnotation::new(0, "list".to_string(), Type::List(Box::new(Type::Any))),
            ArgAnnotation::new(1, "val".to_string(), Type::Any),
            ArgAnnotation::new(2, "idx".to_string(), Type::Int),
        ],
        ret_type: Type::Void,
        bytecode: vec![Bytecode::ListInsert],
    };
    let push_back = BuiltinAnnotation {
        args: vec![
            ArgAnnotation::new(0, "list".to_string(), Type::List(Box::new(Type::Any))),
            ArgAnnotation::new(1, "val".to_string(), Type::Any),
        ],
        ret_type: Type::Void,
        bytecode: vec![Bytecode::ConstI(-1), Bytecode::ListInsert],
    };
    let push_front = BuiltinAnnotation {
        args: vec![
            ArgAnnotation::new(0, "list".to_string(), Type::List(Box::new(Type::Any))),
            ArgAnnotation::new(1, "val".to_string(), Type::Any),
        ],
        ret_type: Type::Void,
        bytecode: vec![Bytecode::ConstI(0), Bytecode::ListInsert],
    };

    // List::remove!(), List::pop_front!(), List::pop_back!(),
    let remove = BuiltinAnnotation {
        args: vec![
            ArgAnnotation::new(0, "list".to_string(), Type::List(Box::new(Type::Any))),
            ArgAnnotation::new(1, "idx".to_string(), Type::Int),
        ],
        ret_type: Type::Void,
        bytecode: vec![Bytecode::ListRemove],
    };
    let pop_back = BuiltinAnnotation {
        args: vec![ArgAnnotation::new(
            0,
            "list".to_string(),
            Type::List(Box::new(Type::Any)),
        )],
        ret_type: Type::Any,
        bytecode: vec![Bytecode::ConstI(-1), Bytecode::ListRemove],
    };
    let pop_front = BuiltinAnnotation {
        args: vec![ArgAnnotation::new(
            0,
            "list".to_string(),
            Type::List(Box::new(Type::Any)),
        )],
        ret_type: Type::Any,
        bytecode: vec![Bytecode::ConstI(0), Bytecode::ListRemove],
    };

    let list_get = BuiltinAnnotation {
        args: vec![
            ArgAnnotation::new(0, "list".to_string(), Type::List(Box::new(Type::Any))),
            ArgAnnotation::new(1, "idx".to_string(), Type::Int),
        ],
        ret_type: Type::Any,
        bytecode: vec![Bytecode::ListGet],
    };
    let list_set = BuiltinAnnotation {
        args: vec![
            ArgAnnotation::new(0, "list".to_string(), Type::List(Box::new(Type::Any))),
            ArgAnnotation::new(1, "val".to_string(), Type::Any),
            ArgAnnotation::new(2, "idx".to_string(), Type::Int),
        ],
        ret_type: Type::Void,
        bytecode: vec![Bytecode::ListSet],
    };
    let list_new = BuiltinAnnotation {
        args: vec![],
        ret_type: Type::List(Box::new(Type::Any)),
        bytecode: vec![Bytecode::ConstL(0)],
    };

    // List iterators
    let list_iter = BuiltinAnnotation {
        args: vec![ArgAnnotation::new(
            0,
            "list".to_string(),
            Type::List(Box::new(Type::Any)),
        )],
        ret_type: Type::List(Box::new(Type::Any)),
        bytecode: vec![Bytecode::Copy],
    };
    let list_next = BuiltinAnnotation {
        args: vec![ArgAnnotation::new(
            0,
            "list".to_string(),
            Type::List(Box::new(Type::Any)),
        )],
        ret_type: Type::Any,
        bytecode: vec![Bytecode::ConstI(0), Bytecode::ListRemove],
    };

    let global_map = builtin_map!(
        "print" => vec![print],
        "assert" => vec![assert],
        "utoi" => vec![utoi],
        "ftoi" => vec![ftoi],
        "itou" => vec![itou],
        "ftou" => vec![ftou],
        "itof" => vec![itof],
        "utof" => vec![utof],
        "len" => vec![len_list.clone(), len_str],
        "copy" => vec![copy.clone()],
    );
    let list_map = builtin_map!(
        "len" => vec![len_list],
        "copy" => vec![copy],
        "insert!" => vec![insert],
        "push_back" => vec![push_back],
        "push_front" => vec![push_front],
        "remove!" => vec![remove],
        "pop_back!" => vec![pop_back],
        "pop_front!" => vec![pop_front],
        "get!" => vec![list_get],
        "new" => vec![list_new],
        "set!" => vec![list_set],
        "__iter__" => vec![list_iter],
        "__next__!" => vec![list_next],
    );

    builtin_map!(
        "Global" => global_map,
        "List" => list_map,
    )
}

/* checks whether the passed arguments match the function annotation */
fn valid_annotation(args: &[ArgAnnotation], received: &VecDeque<Type>) -> bool {
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
