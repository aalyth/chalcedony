mod object;

use object::CVMObject;

use crate::utils::Stack;

const OP_CONSTI: u8 = 1;
const OP_CONSTU: u8 = 2;
const OP_CONSTF: u8 = 3;
const OP_CONSTS: u8 = 4;

const OP_ADD: u8 = 10;
const OP_SUB: u8 = 11;
const OP_MUL: u8 = 12;
const OP_DIV: u8 = 13;
const OP_MOD: u8 = 14;

const OP_CREATE_VAR: u8 = 20;
const OP_DELETE_VAR: u8 = 21;
const OP_CREATE_FUNC: u8 = 22;

const OP_DEBUG: u8 = 200;

use std::collections::HashMap;

pub struct CVM {
    stack: Stack<CVMObject>,
    var_heap: HashMap<String, Vec<CVMObject>>,
    func_heap: HashMap<String, Box<[u8]>>,
    registers: [CVMObject; 16],
}

macro_rules! push_constant {
    ( $cvm:expr, $type:ident, $obj_type:ident, $current_idx:ident, $code:ident) => {{
        let chunk = &$code[$current_idx..$current_idx + 8];
        let val = $type::from_ne_bytes(chunk.try_into().expect("TODO: add proper error handling"));
        $cvm.stack.push(CVMObject::$obj_type(val));
        $current_idx + 8
    }};
}

macro_rules! push_var {
    ( $cvm:expr, $var_name:ident, $type:ident, $obj_type:ident, $val_idx:expr, $code:ident) => {{
        let chunk = &$code[$val_idx..$val_idx + 8];
        let val = $type::from_ne_bytes(chunk.try_into().expect("TODO: add proper error handling"));
        $cvm.var_heap
            .entry($var_name)
            .and_modify(|el| el.push(CVMObject::$obj_type(val)))
            .or_insert(Vec::<CVMObject>::new());
        $val_idx + 8
    }};
}

fn get_arg_name(current_idx: usize, code: &Vec<u8>) -> (String, usize) {
    let mut tmp_idx = current_idx;
    while code[tmp_idx] != 0 {
        tmp_idx += 1;
    }
    let var_name = std::str::from_utf8(&code[current_idx..tmp_idx])
        .expect("TODO: addd proper error checking")
        .to_string();
    (var_name, tmp_idx + 1)
}

impl CVM {
    pub fn new() -> Self {
        CVM {
            stack: Stack::<CVMObject>::new(),
            var_heap: HashMap::<String, Vec<CVMObject>>::new(),
            func_heap: HashMap::<String, Box<[u8]>>::new(),
            registers: std::array::from_fn(|_| CVMObject::Int(0)),
        }
    }

    pub fn interpret(&mut self, mut code: Vec<u8>) {
        let mut current_idx = 0 as usize;
        let code_len = code.len();
        while current_idx < code_len {
            current_idx = self.interpret_next(current_idx, &mut code);
        }
    }

    fn interpret_next(&mut self, mut current_idx: usize, code: &mut Vec<u8>) -> usize {
        let Some(front) = code.get(current_idx) else {
            panic!("TODO: make this throw a proper error");
        };
        current_idx += 1;
        match *front {
            OP_CONSTI => push_constant!(self, i64, Int, current_idx, code),
            OP_CONSTU => push_constant!(self, u64, Uint, current_idx, code),
            OP_CONSTF => push_constant!(self, f64, Float, current_idx, code),

            OP_DEBUG => {
                println!("CVM_STACK: {:#?}\n", self.stack);
                println!("CVM_VAR_HEAP: {:#?}\n", self.var_heap);
                current_idx + 1
            }

            OP_CREATE_VAR => {
                let (var_name, type_idx) = get_arg_name(current_idx, code);
                let var_type = code[type_idx];

                match var_type {
                    OP_CONSTI => push_var!(self, var_name, i64, Int, type_idx + 1, code),
                    OP_CONSTU => push_var!(self, var_name, u64, Uint, type_idx + 1, code),
                    OP_CONSTF => push_var!(self, var_name, f64, Float, type_idx + 1, code),

                    _ => panic!("TODO: add more verbose message for improper variable type"),
                }
            }

            OP_DELETE_VAR => {
                let (var_name, next_opr) = get_arg_name(current_idx, code);
                // TODO: add checking whether the variable exists (the code currently handles it silently)
                self.var_heap.entry(var_name).and_modify(|el| {
                    el.pop();
                });
                next_opr
            }

            _ => panic!("unknown instruction"),
        }
    }
}
