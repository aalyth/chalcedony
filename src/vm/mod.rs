mod builtins;
mod object;

use builtins::{add, and, div, eq, gt, gt_eq, lt, lt_eq, modulo, mul, neg, not, or, sub};
use object::CVMObject;

use crate::lexer::Type;

use crate::utils::{Bytecode, Stack};

use std::collections::BTreeMap;
// use fxhash::BTreeMap;

#[derive(Debug)]
pub enum CVMError {
    ExpectedObject,
    InvalidInstruction,
    InvalidOperation,
    UnknownVariable,
    UnknownFunction,
    TypeAssertionFail(Type, Type), /* exp, recv */
    InvalidType(Type, Type),       /* exp, recv */
}

pub struct CVM {
    stack: Stack<CVMObject>,
    var_heap: BTreeMap<String, Vec<CVMObject>>,
    func_heap: BTreeMap<String, Vec<u8>>,
}

macro_rules! push_constant {
    ( $cvm:expr, $type:ident, $obj_type:ident, $current_idx:ident, $code:ident) => {{
        let chunk = &$code[$current_idx..$current_idx + 8];
        let val = $type::from_ne_bytes(chunk.try_into().expect("TODO: add proper error handling"));
        $cvm.stack.push(CVMObject::$obj_type(val));
        Ok($current_idx + 8)
    }};
}

macro_rules! push_var {
    ( $cvm:expr, $var_name:ident, $type:ident, $obj_type:ident, $val_idx:expr, $code:ident) => {{
        let chunk = &$code[$val_idx..$val_idx + 8];
        let val = $type::from_ne_bytes(chunk.try_into().expect("TODO: add proper error handling"));
        Ok($val_idx + 8)
    }};
}

macro_rules! parse_bytecode_str {
    ( $current_idx:ident, $code:ident) => {{
        let mut tmp_idx = $current_idx;
        while $code[tmp_idx] != 0 {
            tmp_idx += 1;
        }
        let var_name = unsafe { std::str::from_utf8_unchecked(&$code[$current_idx..tmp_idx]) };
        (var_name, tmp_idx + 1)
    }};
}

impl CVM {
    pub fn new() -> Self {
        let mut func_heap = BTreeMap::<String, Vec<u8>>::default();
        func_heap.insert(
            String::from("print"),
            vec![
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
            ],
        );
        CVM {
            stack: Stack::<CVMObject>::new(),
            var_heap: BTreeMap::<String, Vec<CVMObject>>::default(),
            func_heap,
        }
    }

    /* the type at the top of the stack */
    pub fn query_type(&self) -> Option<Type> {
        Some(self.stack.peek()?.as_type())
    }

    pub fn execute(&mut self, code: &Vec<u8>) -> Result<(), CVMError> {
        let mut current_idx = 0 as usize;
        let code_len = code.len();
        while current_idx < code_len {
            current_idx = self.execute_next(current_idx, code)?;
        }
        Ok(())
    }

    fn execute_next(&mut self, mut current_idx: usize, code: &Vec<u8>) -> Result<usize, CVMError> {
        let Some(front_raw) = code.get(current_idx) else {
            panic!("TODO: make this throw a proper error");
        };
        current_idx += 1;
        // SAFETY: Bytecode enum uses `repr(u8)` which itself makes it have the same layout as a u8
        let front: Bytecode = unsafe { std::mem::transmute(*front_raw) };
        match front {
            Bytecode::OpConstI => push_constant!(self, i64, Int, current_idx, code),
            Bytecode::OpConstU => push_constant!(self, u64, Uint, current_idx, code),
            Bytecode::OpConstF => push_constant!(self, f64, Float, current_idx, code),
            Bytecode::OpConstS => {
                let (val, next_idx) = parse_bytecode_str!(current_idx, code);
                self.stack.push(CVMObject::Str(val.to_string()));
                Ok(next_idx)
            }

            Bytecode::OpDebug => {
                /*
                println!("CVM_STACK: {:#?}\n", self.stack);
                println!("CVM_FUNC_HEAP: {:#?}\n", self.func_heap);
                println!("CVM_VAR_HEAP: {:#?}\n", self.var_heap);
                */
                Ok(current_idx)
            }
            Bytecode::OpCreateVar => {
                let (var_name, next_idx) = parse_bytecode_str!(current_idx, code);
                let var_value = self
                    .stack
                    .pop()
                    .expect("TODO: add proper error handling for missing stack value");

                // TODO: add proper occurance checking without cloning
                self.var_heap
                    .entry(var_name.to_string())
                    .and_modify(|el| el.push(var_value.clone()))
                    .or_insert(vec![var_value]);
                Ok(next_idx)
            }

            Bytecode::OpDeleteVar => {
                let (var_name, next_opr) = parse_bytecode_str!(current_idx, code);
                // TODO: add checking whether the variable exists (the code currently handles it silently)
                self.var_heap.entry(var_name.to_string()).and_modify(|el| {
                    el.pop();
                });
                Ok(next_opr)
            }

            Bytecode::OpAdd => add(self, current_idx),
            Bytecode::OpSub => sub(self, current_idx),
            Bytecode::OpMul => mul(self, current_idx),
            Bytecode::OpDiv => div(self, current_idx),
            Bytecode::OpMod => modulo(self, current_idx),

            Bytecode::OpAnd => and(self, current_idx),
            Bytecode::OpOr => or(self, current_idx),

            Bytecode::OpLt => lt(self, current_idx),
            Bytecode::OpGt => gt(self, current_idx),
            Bytecode::OpLtEq => lt_eq(self, current_idx),
            Bytecode::OpGtEq => gt_eq(self, current_idx),

            Bytecode::OpEq => eq(self, current_idx),
            Bytecode::OpNeg => neg(self, current_idx),
            Bytecode::OpNot => not(self, current_idx),

            Bytecode::OpGetVar => {
                let (var_name, next_idx) = parse_bytecode_str!(current_idx, code);
                let Some(var_bucket) = self.var_heap.get(var_name) else {
                    return Err(CVMError::UnknownVariable);
                };
                self.stack.push(
                    var_bucket
                        .last()
                        .expect("TODO: add proper error handling for empty variable bucket")
                        .clone(),
                );
                Ok(next_idx)
            }

            Bytecode::OpCreateFunc => {
                let (fn_name, next_arg_idx) = parse_bytecode_str!(current_idx, code);
                /*
                let body_len = *code
                    .get(next_arg_idx)
                    .expect("TODO: add proper error handling for invalid bytecode syntax")
                    as usize;
                */
                let body_len = u64::from_ne_bytes(
                    code[next_arg_idx..next_arg_idx + 8]
                        .try_into()
                        .expect("TODO: add proper error handling for invalid instruction"),
                );

                let mut body_raw = Vec::with_capacity(body_len as usize);
                let body_start = next_arg_idx + 8;
                let body_end = body_start + body_len as usize;
                body_raw.extend_from_slice(&code[body_start..body_end]);

                // TODO: add checks for conflicting function implementations
                self.func_heap
                    .entry(fn_name.to_string())
                    .or_insert(body_raw);
                Ok(body_end)
            }

            Bytecode::OpCallFunc => {
                let (fn_name, next_idx) = parse_bytecode_str!(current_idx, code);
                if !self.func_heap.contains_key(fn_name) {
                    return Err(CVMError::UnknownFunction);
                }
                // TODO: check if this could be done without a clone
                let function = self.func_heap.get(fn_name).unwrap().clone();
                self.execute(&function)?;
                Ok(next_idx)
            }

            /* terminates the current function's execution */
            Bytecode::OpReturn => Ok(code.len()),
            Bytecode::OpAssertType => {
                let Some(assert_raw) = code.get(current_idx) else {
                    return Err(CVMError::InvalidInstruction);
                };
                let assert_raw: Bytecode = unsafe { std::mem::transmute(*assert_raw) };
                let Ok(assert) = assert_raw.try_into() else {
                    return Err(CVMError::InvalidInstruction);
                };

                let Some(peek_raw) = self.stack.peek() else {
                    return Err(CVMError::ExpectedObject);
                };
                let peek_type = peek_raw.as_type();

                if assert != peek_type {
                    return Err(CVMError::TypeAssertionFail(peek_type, assert));
                }
                Ok(current_idx + 1)
            }

            Bytecode::OpIf => {
                let Some(cond_raw) = self.stack.pop() else {
                    return Err(CVMError::ExpectedObject);
                };
                let CVMObject::Bool(cond) = cond_raw else {
                    return Err(CVMError::InvalidType(Type::Bool, cond_raw.as_type()));
                };

                if !cond {
                    let jmp: usize = u64::from_ne_bytes(
                        code[current_idx..current_idx + 8]
                            .try_into()
                            .expect("TODO: add proper error handling for invalid instruction"),
                    ) as usize;
                    return Ok(current_idx + jmp + 8);
                }
                /* execute the body if the condition is true */
                Ok(current_idx + 8)
            }

            Bytecode::OpJmp => {
                let dist = i64::from_ne_bytes(
                    code[current_idx..current_idx + 8]
                        .try_into()
                        .expect("TODO: add proper error handling for invalid instruction"),
                );
                Ok((current_idx as i64 + dist) as usize)
            }

            Bytecode::OpPrint => {
                let Some(output_obj) = self.stack.pop() else {
                    return Err(CVMError::ExpectedObject);
                };
                let CVMObject::Str(output) = output_obj else {
                    return Err(CVMError::InvalidType(Type::Str, output_obj.as_type()));
                };
                print!("{}", output);
                Ok(current_idx)
            }

            _ => panic!("unknown instruction"),
        }
    }
}
