mod builtins;
mod error;
mod object;

use builtins::{add, and, div, eq, gt, gt_eq, lt, lt_eq, modulo, mul, neg, not, or, sub};
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
    ($cvm:ident, $type:ident, $val:ident, $current_idx:ident) => {{
        $cvm.stack.push(CVMObject::$type(*$val));
        $current_idx
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

    pub fn execute(&mut self, code: &Vec<Bytecode>) {
        let mut current_idx = 0;
        while !self.call_stack.is_empty() || current_idx < code.len() {
            current_idx = self.execute_next(current_idx, &code);
        }
    }

    #[inline(always)]
    fn execute_next(&mut self, curr_idx: usize, code: &Vec<Bytecode>) -> usize {
        // let next_instr = code.get(curr_idx).unwrap();
        let next_instr: &Bytecode;
        if let Some(frame) = self.call_stack.peek() {
            next_instr = frame.code.get(curr_idx).unwrap();
            // println!("NEXT INSTR IS {:?} from frame {:#?}\n", next_instr, frame);
        } else {
            next_instr = code.get(curr_idx).unwrap();
            // println!("NEXT INSTR IS {:?} from code {:#?}\n", next_instr, code);
        }
        let current_idx = curr_idx + 1;
        match next_instr {
            Bytecode::ConstI(val) => push_constant!(self, Int, val, current_idx),
            Bytecode::ConstU(val) => push_constant!(self, Uint, val, current_idx),
            Bytecode::ConstF(val) => push_constant!(self, Float, val, current_idx),
            Bytecode::ConstS(val) => {
                self.stack.push(CVMObject::Str(val.clone()));
                current_idx
            }
            Bytecode::ConstB(val) => push_constant!(self, Bool, val, current_idx),

            Bytecode::Debug => {
                println!("CVM STACK: {:#?}\n", self.stack);
                println!("CVM FUNCTIONS: {:#?}\n", self.functions);
                println!("CVM CALL STACK: {:#?}\n", self.call_stack);
                current_idx
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
                current_idx
            }
            Bytecode::GetGlobal(var_id) => {
                let var_value = self
                    .globals
                    .get(*var_id)
                    .expect("TODO: add proper error checking for invalid variable id")
                    .clone();
                self.stack.push(var_value);
                current_idx
            }

            Bytecode::GetArg(arg_id) => {
                let frame = self.call_stack.peek().expect("expected a stack frame");
                let value = self
                    .stack
                    .get(frame.stack_len as usize + arg_id)
                    .unwrap()
                    .clone();
                self.stack.push(value);
                current_idx
            }

            Bytecode::SetLocal(var_id) => {
                let value = self
                    .stack
                    .pop()
                    .expect("TODO: add proper error handling for missing stack value");
                let frame = self
                    .call_stack
                    .peek()
                    .expect("TODO: check for having a call frame");

                if let Some(var) = self
                    .stack
                    .get_mut(frame.stack_len as usize + frame.args_len as usize + var_id)
                {
                    *var = value;
                } else {
                    self.stack.push(value);
                }
                current_idx
            }
            Bytecode::GetLocal(var_id) => {
                let frame = self.call_stack.peek().expect("missing call frame");
                let value = self
                    .stack
                    .get(frame.stack_len as usize + frame.args_len as usize + var_id)
                    .unwrap()
                    .clone();
                self.stack.push(value);
                current_idx
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

            Bytecode::CreateFunc(arg_count, _) => {
                let func_obj = CVMFunctionObject {
                    arg_count: *arg_count,
                    code: Rc::new(code[current_idx..].into()),
                };
                self.functions.push(Rc::new(func_obj));
                code.len()
            }

            Bytecode::CallFunc(func_id) => {
                let func_obj = self
                    .functions
                    .get(*func_id)
                    .expect("TODO: proper error checking for this")
                    .clone();

                // NOTE: the arguments to the function call are already in place
                // and local variables are automatically handled

                let frame = CVMCallFrame {
                    prev_idx: current_idx as u32,
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
                    return current_idx + jmp;
                }
                /* execute the body if the condition is true */
                current_idx
            }

            Bytecode::Jmp(dist) => (current_idx as isize + dist) as usize,

            Bytecode::Print => {
                let obj = self.stack.pop().expect("expected an object on the stack");
                println!("{}", obj);
                current_idx
            }

            _ => panic!("unknown bytecode instruction"),
        }
    }

    #[inline(always)]
    fn push(&mut self, val: CVMObject) {
        self.stack.push(val)
    }
}
