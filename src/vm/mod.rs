mod builtins;
mod error;
mod object;

use builtins::{add, and, div, eq, gt, gt_eq, lt, lt_eq, modulo, mul, neg, not, or, sub};
use error::{CVMError, CVMErrorKind};
use object::CVMObject;

use crate::error::span::Position;
use crate::lexer::Type;

use crate::utils::{Bytecode, Stack};

use std::rc::Rc;

#[derive(Debug)]
struct CVMFunctionObject {
    arg_count: usize,
    locals_count: usize,
    code: Rc<Vec<Bytecode>>,
}

struct CVMCallFrame {
    args: Vec<CVMObject>,
    locals: Vec<CVMObject>,
}

impl From<Rc<CVMFunctionObject>> for CVMCallFrame {
    fn from(func: Rc<CVMFunctionObject>) -> CVMCallFrame {
        CVMCallFrame {
            args: Vec::with_capacity(func.arg_count),
            locals: vec![CVMObject::default(); func.locals_count],
        }
    }
}

pub struct CVM {
    stack: Stack<CVMObject>,
    globals: Vec<CVMObject>,
    functions: Vec<Rc<CVMFunctionObject>>,
    call_stack: Stack<CVMCallFrame>,
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
            globals: Vec::<CVMObject>::new(),
            functions: Vec::<Rc<CVMFunctionObject>>::new(),
            call_stack: Stack::<CVMCallFrame>::with_capacity(50_000),
        }
    }

    pub fn execute(&mut self, code: &Vec<Bytecode>) -> Result<(), CVMError> {
        let mut current_idx = 0;
        while current_idx < code.len() {
            current_idx = self.execute_next(current_idx, &code)?;
        }
        Ok(())
    }

    /* in this case inlining the function speeds up the interpreter */
    #[inline]
    fn execute_next(&mut self, curr_idx: usize, code: &Vec<Bytecode>) -> Result<usize, CVMError> {
        let next_instr = code.get(curr_idx).unwrap();
        let current_idx = curr_idx + 1;
        match next_instr {
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
            Bytecode::SetGlobal(var_id) => {
                let var_value = self
                    .stack
                    .pop()
                    .expect("TODO: add proper error handling for missing stack value");
                while *var_id >= self.globals.len() {
                    self.globals.push(CVMObject::Int(0));
                }
                *self.globals.get_mut(*var_id).unwrap() = var_value;
                Ok(current_idx)
            }
            Bytecode::GetGlobal(var_id) => {
                let var_value = self
                    .globals
                    .get(*var_id)
                    .expect("TODO: add proper error checking for invalid variable id")
                    .clone();
                self.stack.push(var_value);
                Ok(current_idx)
            }

            Bytecode::GetArg(var_id) => {
                let frame = self.call_stack.peek().expect("TODO: add proper check");
                let value = frame.args.get(*var_id).unwrap().clone();
                self.stack.push(value);
                Ok(current_idx)
            }

            Bytecode::SetLocal(var_id) => {
                let var_value = self
                    .stack
                    .pop()
                    .expect("TODO: add proper error handling for missing stack value");
                let top_frame = self
                    .call_stack
                    .top()
                    .expect("TODO: check for having a call frame");
                // SAFETY: the locals should be allocated upon pushing the call frame
                *top_frame.locals.get_mut(*var_id).unwrap() = var_value;
                Ok(current_idx)
            }
            Bytecode::GetLocal(var_id) => {
                let var_value = self
                    .call_stack
                    .peek()
                    .expect("TODO: add proper error checking for missing call frame")
                    .locals
                    .get(*var_id)
                    .unwrap()
                    .clone();
                self.stack.push(var_value);
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

            Bytecode::CreateFunc(arg_count, locals_count) => {
                let func_obj = CVMFunctionObject {
                    arg_count: *arg_count,
                    locals_count: *locals_count,
                    code: Rc::new(code[current_idx..].into()),
                };
                self.functions.push(Rc::new(func_obj));
                Ok(code.len())
            }

            Bytecode::CallFunc(func_id) => {
                let func_obj = self
                    .functions
                    .get(*func_id)
                    .expect("TODO: proper error checking for this")
                    .clone();
                // println!("FUNCTION OBJECT: {:#?}\n", func_obj);
                let mut frame = CVMCallFrame::from(func_obj.clone());

                for _ in 0..func_obj.arg_count {
                    frame
                        .args
                        .push(self.stack.pop().expect("TODO: add proper stack checks"));
                }

                self.call_stack.push(frame);
                self.execute(&func_obj.code);
                Ok(current_idx)
            }

            Bytecode::Return => {
                self.call_stack.pop();
                Ok(code.len())
            }

            // TODO: remove the whole operation - check types compile time
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
