mod builtins;
mod error;
mod object;

use builtins::{add, and, div, eq, gt, gt_eq, lt, lt_eq, modulo, mul, neg, not, or, sub};
use error::{CVMError, CVMErrorKind};
use object::CVMObject;

use crate::error::Position;
use crate::lexer::Type;

use crate::utils::{Bytecode, Stack};

use std::collections::{BTreeMap, VecDeque};
// use fxhash::BTreeMap;

pub struct CVM {
    stack: Stack<CVMObject>,
    var_heap: BTreeMap<String, VecDeque<CVMObject>>,
    call_stack: Stack<usize>,

    start: Position,
    end: Position,
    span_id: u16,
}

macro_rules! parse_constant {
    ( $cvm:expr, $type:ident, $current_idx:ident, $code:ident) => {{
        let type_size = std::mem::size_of::<$type>();
        let Ok(chunk) =
            &$code[$current_idx..$current_idx + type_size].try_into() as &Result<[u8; 8], _>
        else {
            return Err($cvm.error(CVMErrorKind::InvalidInstruction));
        };
        ($type::from_ne_bytes(*chunk), $current_idx + type_size)
    }};
}

macro_rules! push_constant {
    ( $cvm:expr, $type:ident, $obj_type:ident, $current_idx:ident, $code:ident) => {{
        let (val, next_idx) = parse_constant!($cvm, $type, $current_idx, $code);
        $cvm.stack.push(CVMObject::$obj_type(val));
        Ok(next_idx)
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
        CVM {
            stack: Stack::<CVMObject>::with_capacity(1000),
            var_heap: BTreeMap::<String, VecDeque<CVMObject>>::default(),
            call_stack: Stack::<usize>::new(),

            start: Position::new(1, 1),
            end: Position::new(1, 1),
            span_id: 0,
        }
    }

    /* the type at the top of the stack */
    pub fn query_type(&self) -> Option<Type> {
        Some(self.stack.peek()?.as_type())
    }

    pub fn execute(&mut self, start: usize, code: Vec<u8>) -> Result<(), CVMError> {
        let mut current_idx = start;
        while current_idx < code.len() {
            current_idx = self.execute_next(current_idx, &code)?;
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
            Bytecode::OpConstB => {
                let Ok(chunk) =
                    &code[current_idx..current_idx + 1].try_into() as &Result<[u8; 1], _>
                else {
                    return Err(self.error(CVMErrorKind::InvalidInstruction));
                };
                let byte = u8::from_ne_bytes(*chunk);
                self.stack.push(CVMObject::Bool(byte != 0));
                Ok(current_idx + 1)
            }

            Bytecode::OpDebug => {
                /*
                println!("CVM_STACK: {:#?}\n", self.stack);
                println!("CVM_VAR_HEAP: {:#?}\n", self.var_heap);
                println!("CVM_CALL_STACK: {:#?}\n", self.call_stack);
                */
                Ok(current_idx)
            }
            Bytecode::OpCreateVar => {
                let (var_name, next_idx) = parse_bytecode_str!(current_idx, code);
                let var_value = self
                    .stack
                    .pop()
                    .expect("TODO: add proper error handling for missing stack value");

                if let Some(var_bucket) = self.var_heap.get_mut(var_name) {
                    var_bucket.push_back(var_value);
                } else {
                    let mut new_bucket = VecDeque::new();
                    new_bucket.push_back(var_value);
                    self.var_heap.insert(var_name.to_string(), new_bucket);
                }
                Ok(next_idx)
            }

            Bytecode::OpDeleteVar => {
                let (var_name, next_opr) = parse_bytecode_str!(current_idx, code);
                // TODO: add checking whether the variable exists (the code currently handles it silently)
                self.var_heap.entry(var_name.to_string()).and_modify(|el| {
                    el.pop_back();
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
                    return Err(self.error(CVMErrorKind::UnknownVariable(var_name.to_string())));
                };
                self.stack.push(
                    var_bucket
                        .back()
                        .expect("TODO: add proper error handling for empty variable bucket")
                        .clone(),
                );
                Ok(next_idx)
            }

            Bytecode::OpCallFunc => {
                let (fn_pos, next_idx) = parse_constant!(self, u64, current_idx, code);
                self.call_stack.push(next_idx);
                Ok(fn_pos as usize)
            }

            Bytecode::OpReturn => {
                /* if there are no other call_stacks, we terminate the execution of main */
                if let Some(prev_idx) = self.call_stack.pop() {
                    Ok(prev_idx)
                } else {
                    Ok(code.len())
                }
            }

            Bytecode::OpAssertType => {
                let Some(assert_raw) = code.get(current_idx) else {
                    return Err(self.error(CVMErrorKind::InvalidInstruction));
                };
                let assert_raw: Bytecode = unsafe { std::mem::transmute(*assert_raw) };
                let Ok(assert) = assert_raw.try_into() else {
                    return Err(self.error(CVMErrorKind::InvalidInstruction));
                };

                let Some(peek_raw) = self.stack.peek() else {
                    return Err(self.error(CVMErrorKind::ExpectedObject));
                };
                let peek_type = peek_raw.as_type();

                if assert != peek_type {
                    return Err(self.error(CVMErrorKind::TypeAssertionFail(assert, peek_type)));
                }
                Ok(current_idx + 1)
            }

            Bytecode::OpIf => {
                let Some(cond_raw) = self.stack.pop() else {
                    return Err(self.error(CVMErrorKind::ExpectedObject));
                };
                let CVMObject::Bool(cond) = cond_raw else {
                    return Err(
                        self.error(CVMErrorKind::InvalidType(Type::Bool, cond_raw.as_type()))
                    );
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
                    return Err(self.error(CVMErrorKind::ExpectedObject));
                };
                let CVMObject::Str(output) = output_obj else {
                    return Err(
                        self.error(CVMErrorKind::InvalidType(Type::Str, output_obj.as_type()))
                    );
                };
                println!("{}", output);
                Ok(current_idx)
            }

            Bytecode::OpStartLn => {
                let (ln, next_idx) = parse_constant!(self, u64, current_idx, code);
                self.start.ln = ln as usize;
                Ok(next_idx)
            }

            Bytecode::OpStartCol => {
                let (col, next_idx) = parse_constant!(self, u64, current_idx, code);
                self.start.col = col as usize;
                Ok(next_idx)
            }

            Bytecode::OpEndLn => {
                let (ln, next_idx) = parse_constant!(self, u64, current_idx, code);
                self.end.ln = ln as usize;
                Ok(next_idx)
            }

            Bytecode::OpEndCol => {
                let (col, next_idx) = parse_constant!(self, u64, current_idx, code);
                self.end.col = col as usize;
                Ok(next_idx)
            }

            _ => Err(self.error(CVMErrorKind::UnknownInstruction)),
        }
    }

    pub fn error(&self, kind: CVMErrorKind) -> CVMError {
        CVMError::new(kind, self.start, self.end, self.span_id)
    }
}
