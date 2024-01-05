mod builtins;
mod error;
mod object;

use builtins::{add, and, div, eq, gt, gt_eq, lt, lt_eq, modulo, mul, neg, not, or, sub};
use error::{CVMError, CVMErrorKind};
use object::CVMObject;

use crate::error::span::Position;
use crate::lexer::Type;

use crate::utils::{Bytecode, Stack};

pub struct CVM {
    stack: Stack<CVMObject>,
    var_heap: Vec<Vec<CVMObject>>,
    call_stack: Stack<usize>,
}

macro_rules! push_constant {
    ($cvm:ident, $type:ident, $val:ident, $current_idx:ident) => {{
        $cvm.stack.push(CVMObject::$type(*$val));
        Ok($current_idx)
    }};
}

impl CVM {
    pub fn new() -> Self {
        CVM {
            stack: Stack::<CVMObject>::with_capacity(100_000),
            var_heap: Vec::<Vec<CVMObject>>::new(),
            call_stack: Stack::<usize>::new(),
        }
    }

    pub fn execute(&mut self, start: usize, code: Vec<Bytecode>) -> Result<(), CVMError> {
        let mut current_idx = start;
        while current_idx < code.len() {
            current_idx = self.execute_next(current_idx, &code)?;
        }
        Ok(())
    }

    fn execute_next(
        &mut self,
        mut current_idx: usize,
        code: &Vec<Bytecode>,
    ) -> Result<usize, CVMError> {
        let Some(front) = code.get(current_idx) else {
            panic!("TODO: make this throw a proper error");
        };
        current_idx += 1;
        match front {
            Bytecode::ConstI(val) => push_constant!(self, Int, val, current_idx),
            Bytecode::ConstU(val) => push_constant!(self, Uint, val, current_idx),
            Bytecode::ConstF(val) => push_constant!(self, Float, val, current_idx),
            Bytecode::ConstS(val) => {
                self.stack.push(CVMObject::Str(val.clone()));
                Ok(current_idx)
            }
            Bytecode::ConstB(val) => push_constant!(self, Bool, val, current_idx),

            Bytecode::Debug => {
                /*
                println!("CVM_STACK: {:#?}\n", self.stack);
                println!("CVM_VAR_HEAP: {:#?}\n", self.var_heap);
                println!("CVM_CALL_STACK: {:#?}\n", self.call_stack);
                */
                Ok(current_idx)
            }
            Bytecode::CreateVar(var_id) => {
                while self.var_heap.len() <= *var_id as usize {
                    self.var_heap.push(Vec::<CVMObject>::with_capacity(1_000));
                }
                let var_value = self
                    .stack
                    .pop()
                    .expect("TODO: add proper error handling for missing stack value");
                let Some(var_bucket) = self.var_heap.get_mut(*var_id as usize) else {
                    panic!("TODO: add proper error handling for missing variable bucket")
                };
                var_bucket.push(var_value);
                Ok(current_idx)
            }

            Bytecode::DeleteVar(var_id) => {
                let Some(var_bucket) = self.var_heap.get_mut(*var_id as usize) else {
                    panic!("TODO: add proper error handling for missing variable bucket")
                };
                var_bucket.pop();
                Ok(current_idx)
            }

            Bytecode::Add => add(self, current_idx),
            Bytecode::Sub => sub(self, current_idx),
            Bytecode::Mul => mul(self, current_idx),
            Bytecode::Div => div(self, current_idx),
            Bytecode::Mod => modulo(self, current_idx),

            Bytecode::And => and(self, current_idx),
            Bytecode::Or => or(self, current_idx),

            Bytecode::Lt => lt(self, current_idx),
            Bytecode::Gt => gt(self, current_idx),
            Bytecode::LtEq => lt_eq(self, current_idx),
            Bytecode::GtEq => gt_eq(self, current_idx),

            Bytecode::Eq => eq(self, current_idx),
            Bytecode::Neg => neg(self, current_idx),
            Bytecode::Not => not(self, current_idx),

            Bytecode::GetVar(var_id) => {
                let Some(var_bucket) = self.var_heap.get(*var_id as usize) else {
                    panic!("TODO: make this throw a proper error");
                };
                if let Some(val) = var_bucket.last() {
                    self.stack.push(val.clone());
                }
                Ok(current_idx)
            }

            Bytecode::CallFunc(func_pos) => {
                self.call_stack.push(current_idx);
                Ok(*func_pos)
            }

            Bytecode::Return => {
                /* if there are no other call_stacks, we terminate the execution of main */
                if let Some(prev_idx) = self.call_stack.pop() {
                    Ok(prev_idx)
                } else {
                    Ok(code.len())
                }
            }

            Bytecode::Assert(_type) => {
                /*
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
                */
                Ok(current_idx)
            }

            Bytecode::If(jmp) => {
                let Some(cond_raw) = self.stack.pop() else {
                    return Err(self.error(CVMErrorKind::ExpectedObject));
                };
                let CVMObject::Bool(cond) = cond_raw else {
                    return Err(
                        self.error(CVMErrorKind::InvalidType(Type::Bool, cond_raw.as_type()))
                    );
                };

                if !cond {
                    return Ok(current_idx + jmp);
                }
                /* execute the body if the condition is true */
                Ok(current_idx)
            }

            Bytecode::Jmp(dist) => Ok((current_idx as isize + dist) as usize),

            Bytecode::Print => {
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

            _ => Err(self.error(CVMErrorKind::UnknownInstruction)),
        }
    }

    pub fn error(&self, kind: CVMErrorKind) -> CVMError {
        CVMError::new(kind, Position::new(1, 1), Position::new(1, 1), 0)
    }
}
