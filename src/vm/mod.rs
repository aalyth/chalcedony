mod builtins;
mod object;

use builtins::{add, and, assert, div, eq, gt, gt_eq, lt, lt_eq, modulo, mul, neg, not, or, sub};
use object::CVMObject;

use crate::common::Bytecode;
use crate::utils::Stack;

use std::rc::Rc;

#[derive(Debug)]
struct CVMFunctionObject {
    arg_count: usize,
    code: Rc<Vec<Bytecode>>,
}

#[derive(Debug)]
struct CVMCallFrame {
    prev_idx: u32,
    args_len: u16,
    stack_len: u32,

    code: Rc<Vec<Bytecode>>,
}

pub struct CVM {
    stack: Stack<CVMObject>,
    globals: Vec<CVMObject>,
    functions: Vec<Rc<CVMFunctionObject>>,
    call_stack: Stack<CVMCallFrame>,
}

macro_rules! push_constant {
    ($cvm:ident, $type:ident, $val:ident, $next_idx:ident) => {{
        $cvm.stack.push(CVMObject::$type(*$val));
        $next_idx
    }};
}

impl CVM {
    pub fn new() -> Self {
        CVM {
            stack: Stack::<CVMObject>::with_capacity(100_000),
            globals: Vec::<CVMObject>::new(),
            functions: Vec::<Rc<CVMFunctionObject>>::new(),
            call_stack: Stack::<CVMCallFrame>::with_capacity(10_000),
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
    fn execute_next(&mut self, current_idx: usize, code: &Vec<Bytecode>) -> usize {
        // let next_instr = code.get(curr_idx).unwrap();
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
                self.stack.push(CVMObject::Str(val.clone()));
                next_idx
            }
            Bytecode::ConstB(val) => push_constant!(self, Bool, val, next_idx),

            Bytecode::SetGlobal(var_id) => {
                let var_value = self.stack.pop().expect("expected a value on the stack");
                while *var_id >= self.globals.len() {
                    self.globals.push(CVMObject::Int(0));
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
                *self
                    .stack
                    .get_mut(frame.stack_len as usize + *arg_id)
                    .unwrap() = value;
                next_idx
            }

            Bytecode::GetArg(arg_id) => {
                let frame = self.call_stack.peek().expect("expected a stack frame");
                let value = self
                    .stack
                    .get(frame.stack_len as usize + arg_id)
                    .unwrap()
                    .clone();
                self.stack.push(value);
                next_idx
            }

            Bytecode::SetLocal(mut var_id) => {
                let value = self.stack.pop().expect("expected a value on the stack");

                /* since local variables can exist outside of a function scope, a check is
                 * performed whether there is a call frame */
                if let Some(frame) = self.call_stack.peek() {
                    var_id = frame.stack_len as usize + frame.args_len as usize + var_id;
                }

                if let Some(var) = self.stack.get_mut(var_id) {
                    *var = value;
                } else {
                    while self.stack.len() <= var_id {
                        self.stack.push(CVMObject::default());
                    }
                    self.stack.push(value);
                }
                next_idx
            }
            Bytecode::GetLocal(mut var_id) => {
                if let Some(frame) = self.call_stack.peek() {
                    var_id = frame.stack_len as usize + frame.args_len as usize + var_id;
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
                let func_obj = CVMFunctionObject {
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

                // NOTE: the arguments to the function call are already in place
                // and local variables are automatically handled

                let frame = CVMCallFrame {
                    prev_idx: next_idx as u32,
                    stack_len: (self.stack.len() - func_obj.arg_count) as u32,
                    args_len: func_obj.arg_count as u16,
                    code: func_obj.code.clone(),
                };
                self.call_stack.push(frame);

                0
            }

            Bytecode::Return => {
                let value = self.stack.pop().unwrap();
                let frame = self.call_stack.pop().unwrap();
                self.stack.truncate(frame.stack_len as usize);
                self.stack.push(value);
                frame.prev_idx as usize
            }

            Bytecode::ReturnVoid => {
                let frame = self.call_stack.pop().unwrap();
                self.stack.truncate(frame.stack_len as usize);
                frame.prev_idx as usize
            }

            Bytecode::If(jmp) => {
                let cond_raw = self.stack.pop().expect("expected an object on the stack");
                let CVMObject::Bool(cond) = cond_raw else {
                    panic!("expected a bool when checking if statements")
                };

                if !cond {
                    return next_idx + jmp;
                }
                /* execute the body if the condition is true */
                next_idx
            }

            Bytecode::Jmp(dist) => (next_idx as isize + dist) as usize,

            Bytecode::Print => {
                let obj = self.stack.pop().expect("expected an object on the stack");
                println!("{}", obj);
                next_idx
            }

            Bytecode::Assert => assert(self, next_idx),
        }
    }

    #[inline(always)]
    fn push(&mut self, val: CVMObject) {
        self.stack.push(val)
    }
}
