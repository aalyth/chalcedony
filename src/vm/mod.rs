mod builtins;
mod object;

use builtins::{
    add, and, assert, div, eq, gt, gt_eq, list_insert, lt, lt_eq, modulo, mul, neg, not, or, sub,
};
use object::CvmObject;

use crate::common::Bytecode;
use crate::error::unhandled_exception;
use crate::utils::Stack;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
struct CvmFunctionObject {
    arg_count: usize,
    code: Rc<Vec<Bytecode>>,
}

#[derive(Debug, Default)]
struct CvmCallFrame {
    prev_idx: usize,
    args_len: usize,
    stack_len: usize,
    catch_idx: Option<usize>,

    code: Rc<Vec<Bytecode>>,
}

#[derive(Default)]
pub struct Cvm {
    stack: Stack<CvmObject>,
    globals: Vec<CvmObject>,
    functions: Vec<Rc<CvmFunctionObject>>,
    call_stack: Stack<CvmCallFrame>,

    catch_idx: Option<usize>,
}

macro_rules! push_constant {
    ($cvm:ident, $type:ident, $val:ident, $next_idx:ident) => {{
        $cvm.stack.push(CvmObject::$type(*$val));
        $next_idx
    }};
}

impl Cvm {
    pub fn new() -> Self {
        Cvm {
            stack: Stack::<CvmObject>::with_capacity(100_000),
            globals: Vec::<CvmObject>::new(),
            functions: Vec::<Rc<CvmFunctionObject>>::new(),
            call_stack: Stack::<CvmCallFrame>::with_capacity(10_000),
            catch_idx: None,
        }
    }

    pub fn execute(&mut self, code: Vec<Bytecode>) {
        let mut current_idx = 0;
        while !self.call_stack.is_empty() || current_idx < code.len() {
            current_idx = self.execute_next(current_idx, &code);
        }
        /* remove any leftover local variables inside the global scope */
        self.stack.truncate(0);
    }

    #[inline(always)]
    fn execute_next(&mut self, current_idx: usize, code: &[Bytecode]) -> usize {
        let next_instr: &Bytecode;
        if let Some(frame) = self.call_stack.peek() {
            next_instr = frame.code.get(current_idx).expect("invalid current idx");
        } else {
            next_instr = code.get(current_idx).expect("invalid current idx");
        }
        let next_idx = current_idx + 1;
        match next_instr {
            Bytecode::ConstI(val) => push_constant!(self, Int, val, next_idx),
            Bytecode::ConstU(val) => push_constant!(self, Uint, val, next_idx),
            Bytecode::ConstF(val) => push_constant!(self, Float, val, next_idx),
            Bytecode::ConstS(val) => {
                self.stack.push(CvmObject::Str(val.clone()));
                next_idx
            }
            Bytecode::ConstB(val) => push_constant!(self, Bool, val, next_idx),
            Bytecode::ConstL => {
                self.stack
                    .push(CvmObject::List(Rc::new(RefCell::new(VecDeque::new()))));
                next_idx
            }

            Bytecode::ThrowException => {
                let obj = self.stack.pop().expect("expected a value on the stack");
                let CvmObject::Str(val) = obj else {
                    panic!("invalid exception type");
                };
                self.stack.push(CvmObject::Exception(val));
                self.handle_exception()
            }

            Bytecode::CastI => {
                let obj = self.stack.pop().expect("expected a value on the stack");
                let CvmObject::Uint(val) = obj else {
                    panic!("casting non-uint to int")
                };
                self.stack.push(CvmObject::Int(val as i64));
                next_idx
            }

            Bytecode::CastF => {
                let obj = self.stack.pop().expect("expected a value on the stack");
                match obj {
                    CvmObject::Int(val) => self.stack.push(CvmObject::Float(val as f64)),
                    CvmObject::Uint(val) => self.stack.push(CvmObject::Float(val as f64)),
                    _ => panic!("casting non-uint or int to float"),
                }
                next_idx
            }

            Bytecode::Len => {
                let obj = self.stack.pop().expect("expected a value on the stack");
                match obj {
                    CvmObject::List(list) => {
                        self.stack.push(CvmObject::Uint(list.borrow().len() as u64))
                    }
                    CvmObject::Str(val) => self.stack.push(CvmObject::Uint(val.len() as u64)),
                    _ => panic!("getting the length of non string/list"),
                }
                next_idx
            }

            Bytecode::GetIdx => {
                let CvmObject::Uint(idx) = self.stack.pop().expect("expected a value on the stack")
                else {
                    panic!("invalid index type");
                };
                let CvmObject::List(list) =
                    self.stack.pop().expect("expected a value on the stack")
                else {
                    panic!("invalid list type");
                };
                let list = list.borrow();
                self.stack
                    .push(list.get(idx as usize).expect("invalid index").clone());
                next_idx
            }

            Bytecode::SetGlobal(var_id) => {
                let var_value = self.stack.pop().expect("expected a value on the stack");
                while *var_id >= self.globals.len() {
                    self.globals.push(CvmObject::Int(0));
                }
                *self.globals.get_mut(*var_id).unwrap() = var_value;
                next_idx
            }
            Bytecode::GetGlobal(var_id) => {
                let var_value = self
                    .globals
                    .get(*var_id)
                    .expect("expected a valid variable id")
                    .clone();
                self.stack.push(var_value);
                next_idx
            }

            Bytecode::SetArg(arg_id) => {
                let frame = self.call_stack.peek().expect("expected a stack frame");
                let value = self.stack.pop().expect("expected a value on the stack");
                *self.stack.get_mut(frame.stack_len + *arg_id).unwrap() = value;
                next_idx
            }

            Bytecode::GetArg(arg_id) => {
                let frame = self.call_stack.peek().expect("expected a stack frame");
                let value = self.stack.get(frame.stack_len + arg_id).unwrap().clone();
                self.stack.push(value);
                next_idx
            }

            Bytecode::SetLocal(mut var_id) => {
                let value = self.stack.pop().expect("expected a value on the stack");

                /* since local variables can exist outside of a function scope, a check is
                 * performed whether there is a call frame */
                if let Some(frame) = self.call_stack.peek() {
                    var_id += frame.stack_len + frame.args_len;
                }

                if let Some(var) = self.stack.get_mut(var_id) {
                    *var = value;
                } else {
                    while self.stack.len() < var_id {
                        self.stack.push(CvmObject::default());
                    }
                    self.stack.push(value);
                }
                next_idx
            }
            Bytecode::GetLocal(mut var_id) => {
                if let Some(frame) = self.call_stack.peek() {
                    var_id += frame.stack_len + frame.args_len;
                }
                let value = self.stack.get(var_id).unwrap().clone();
                self.stack.push(value);
                next_idx
            }

            Bytecode::Add => add(self, next_idx),
            Bytecode::Sub => sub(self, next_idx),
            Bytecode::Mul => mul(self, next_idx),
            Bytecode::Div => div(self, next_idx),
            Bytecode::Mod => modulo(self, next_idx),

            Bytecode::And => and(self, next_idx),
            Bytecode::Or => or(self, next_idx),

            Bytecode::Lt => lt(self, next_idx),
            Bytecode::Gt => gt(self, next_idx),
            Bytecode::LtEq => lt_eq(self, next_idx),
            Bytecode::GtEq => gt_eq(self, next_idx),

            Bytecode::Eq => eq(self, next_idx),
            Bytecode::Neg => neg(self, next_idx),
            Bytecode::Not => not(self, next_idx),

            Bytecode::CreateFunc(arg_count) => {
                let func_obj = CvmFunctionObject {
                    arg_count: *arg_count,
                    code: Rc::new(code[next_idx..].into()),
                };
                self.functions.push(Rc::new(func_obj));
                code.len()
            }

            Bytecode::CallFunc(func_id) => {
                let func_obj = self
                    .functions
                    .get(*func_id)
                    .expect("expected a valid function id")
                    .clone();

                /* NOTE: the arguments to the function call are already in place
                 * and local variables are automatically handled */

                let frame = CvmCallFrame {
                    prev_idx: next_idx,
                    stack_len: self.stack.len() - func_obj.arg_count,
                    args_len: func_obj.arg_count,
                    code: func_obj.code.clone(),
                    catch_idx: self.catch_idx,
                };
                self.call_stack.push(frame);
                self.catch_idx = None;

                0
            }

            Bytecode::Return => {
                let value = self.stack.pop().unwrap();
                let frame = self.call_stack.pop().unwrap();
                self.stack.truncate(frame.stack_len);
                self.catch_idx = frame.catch_idx;
                self.stack.push(value);
                frame.prev_idx
            }

            Bytecode::ReturnVoid => {
                let frame = self.call_stack.pop().unwrap();
                self.stack.truncate(frame.stack_len);
                self.catch_idx = frame.catch_idx;
                frame.prev_idx
            }

            Bytecode::If(jmp) => {
                let cond_raw = self.stack.pop().expect("expected a value on the stack");
                let CvmObject::Bool(cond) = cond_raw else {
                    panic!("expected a bool when checking if statements")
                };

                if !cond {
                    return next_idx + jmp;
                }
                /* execute the body if the condition is true */
                next_idx
            }

            Bytecode::Jmp(dist) => (next_idx as isize + dist) as usize,

            Bytecode::LInsert(idx) => list_insert(self, *idx, next_idx),
            Bytecode::LPop(_idx) => todo!(), // TODO:

            Bytecode::TryScope(offset) => {
                /* SAFETY: it is guaranteed there are no nested try-catch blocks */
                self.catch_idx = Some(next_idx + offset);
                next_idx
            }

            Bytecode::CatchJmp(dist) => {
                self.catch_idx = None;
                next_idx + dist
            }

            Bytecode::Print => {
                let obj = self.stack.pop().expect("expected an object on the stack");
                println!("{}", obj);
                next_idx
            }

            Bytecode::Assert => assert(self, next_idx),

            Bytecode::Nop => next_idx,
        }
    }

    #[inline(always)]
    fn push(&mut self, val: CvmObject) {
        self.stack.push(val)
    }

    /* returns the index of the next instruction */
    fn handle_exception(&mut self) -> usize {
        if let Some(catch_idx) = self.catch_idx {
            self.catch_idx = None;
            return catch_idx;
        }

        while let Some(frame) = self.call_stack.pop() {
            let exc = self.stack.pop().expect("expected the exception");
            self.stack.truncate(frame.stack_len);
            self.stack.push(exc);
            if let Some(catch_idx) = frame.catch_idx {
                return catch_idx;
            }
        }

        let obj = self.stack.pop().expect("expected a value on the stack");
        let CvmObject::Exception(exc) = obj else {
            panic!("invalid exception");
        };

        unhandled_exception(format!("{}", exc));
        unreachable!();
    }
}
