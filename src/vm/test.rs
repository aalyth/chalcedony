use builtins::{add, and, div, eq, gt, gt_eq, lt, lt_eq, modulo, mul, neg, not, or, sub};
use error::{CVMError, CVMErrorKind};
use object::CVMObject;

use crate::error::Position;
use crate::lexer::Type;

use crate::utils::{Bytecode, Stack, BytecodeOpr};

pub struct CVMTest {
    stack: Stack<CVMObject>,
    var_heap: Vec<Vec<CVMObject>>,
    call_stack: Stack<usize>,
}

impl CVMTest {
    pub fn new() -> Self {
        CVMTest {
            stack: Stack::<CVMObject>::with_capacity(100_000),
            var_heap: Vec::<Vec<CVMObject>>::new(),
            call_stack: Stack::<usize>::new(),
        }
    }

    pub fn execute(&mut self, start: usize, code: Vec<BytecodeOpr>) -> Result<(), CVMError> {
        let mut current_idx = start;
        while current_idx < code.len() {
            current_idx = self.execute_next(current_idx, &code)?;
        }
        Ok(())
    }

    fn execute_next(&mut self, mut current_idx: usize, code: &Vec<BytecodeOpr>) -> Result<usize, CVMError> {
        let Some(front) = code.get(current_idx) else {
            panic!("TODO: make this throw a proper error");
        };
        current_idx += 1;
        match *front {
            BytecodeOpr::ConstI(val) => self.stack.push(CVMObject::Int(val)),
            BytecodeOpr::ConstU(val) => self.stack.push(CVMObject::Uint(val)),
            BytecodeOpr::ConstF(val) => self.stack.push(CVMObject::Float(val)),
            BytecodeOpr::ConstS(val) => self.stack.push(CVMObject::Str(val)),
            BytecodeOpr::ConstB(val) => self.stack.push(CVMObject::Bool(val)),

            BytecodeOpr::Debug => {
                /*
                println!("CVM_STACK: {:#?}\n", self.stack);
                println!("CVM_VAR_HEAP: {:#?}\n", self.var_heap);
                println!("CVM_CALL_STACK: {:#?}\n", self.call_stack);
                */
                Ok(current_idx)
            }
            BytecodeOpr::CreateVar(var_id) => {
                while self.var_heap.len() <= var_id as usize {
                    self.var_heap.push(Vec::<CVMObject>::with_capacity(1_000));
                }
                let var_value = self
                    .stack
                    .pop()
                    .expect("TODO: add proper error handling for missing stack value");
                let Some(var_bucket) = self.var_heap.get_mut(var_id as usize) else {
                    panic!("TODO: add proper error handling for missing variable bucket")
                };
                var_bucket.push(var_value);
                Ok(current_idx)
            }

            BytecodeOpr::DeleteVar(var_id) => {
                let Some(var_bucket) = self.var_heap.get_mut(var_id as usize) else {
                    panic!("TODO: add proper error handling for missing variable bucket")
                };
                var_bucket.pop();
                Ok(current_idx)
            }

            BytecodeOpr::Add => add(self, current_idx),
            BytecodeOpr::Sub => sub(self, current_idx),
            BytecodeOpr::Mul => mul(self, current_idx),
            BytecodeOpr::Div => div(self, current_idx),
            BytecodeOpr::Mod => modulo(self, current_idx),

            BytecodeOpr::And => and(self, current_idx),
            BytecodeOpr::Or => or(self, current_idx),

            BytecodeOpr::Lt => lt(self, current_idx),
            BytecodeOpr::Gt => gt(self, current_idx),
            BytecodeOpr::LtEq => lt_eq(self, current_idx),
            BytecodeOpr::GtEq => gt_eq(self, current_idx),

            BytecodeOpr::Eq => eq(self, current_idx),
            BytecodeOpr::Neg => neg(self, current_idx),
            BytecodeOpr::Not => not(self, current_idx),

            BytecodeOpr::GetVar(var_id) => {
                let Some(var_bucket) = self.var_heap.get(var_id as usize) else {
                    panic!("TODO: make this throw a proper error");
                };
                if let Some(val) = var_bucket.last() {
                    self.stack.push(val.clone());
                }
                Ok(current_idx)
            }

            BytecodeOpr::CallFunc(func_pos) => {
                self.call_stack.push(current_idx);
                Ok(func_pos)
            }

            BytecodeOpr::Return => {
                /* if there are no other call_stacks, we terminate the execution of main */
                if let Some(prev_idx) = self.call_stack.pop() {
                    Ok(prev_idx)
                } else {
                    Ok(code.len())
                }
            }

            BytecodeOpr::Assert(Type) => {
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

            BytecodeOpr::If(jmp) => {
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

            BytecodeOpr::Jmp(dist) => {
                Ok(current_idx+ dist)
            }

            BytecodeOpr::Print => {
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
