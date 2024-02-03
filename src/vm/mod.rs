mod builtins;
mod error;
mod object;

use builtins::{add, and, div, eq, gt, gt_eq, lt, lt_eq, modulo, mul, neg, not, or, sub};
use object::CVMObject;

use crate::common::Bytecode;
use crate::utils::{PtrArray, Stack};

use std::rc::Rc;

#[derive(Debug)]
struct CVMFunctionObject {
    arg_count: usize,
    local_count: usize,
    code: Rc<Vec<Bytecode>>,
}

#[derive(Debug)]
struct CVMCallFrame {
    arg_count: usize,
    variables: PtrArray<CVMObject>,
}

impl From<Rc<CVMFunctionObject>> for CVMCallFrame {
    fn from(func: Rc<CVMFunctionObject>) -> Self {
        CVMCallFrame {
            arg_count: func.arg_count,
            variables: PtrArray::new(func.arg_count + func.local_count),
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
        while current_idx < code.len() {
            current_idx = self.execute_next(current_idx, &code);
        }
    }

    #[inline(always)]
    fn execute_next(&mut self, curr_idx: usize, code: &Vec<Bytecode>) -> usize {
        let next_instr = code.get(curr_idx).unwrap();
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
                println!("CVM_STACK: {:#?}\n", self.stack);
                println!("CVM_VAR_HEAP: {:#?}\n", self.functions);
                println!("CVM_CALL_STACK: {:#?}\n", self.call_stack);
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

            Bytecode::GetArg(var_id) => {
                let frame = self.call_stack.peek().expect("TODO: add proper check");
                let value = frame.variables.get(*var_id).clone();
                self.stack.push(value);
                current_idx
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
                /*
                while var_id <= &top_frame.locals.len() {
                    top_frame.locals.push(CVMObject::default());
                }
                */
                // *top_frame.locals.get_mut(*var_id).unwrap() = var_value;
                top_frame.variables.set(*var_id, var_value);
                current_idx
            }
            Bytecode::GetLocal(var_id) => {
                let frame = self.call_stack.peek().expect("missing call frame");
                let var_value = frame.variables.get(frame.arg_count + *var_id).clone();
                self.stack.push(var_value);
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

            Bytecode::CreateFunc(arg_count, local_count) => {
                let func_obj = CVMFunctionObject {
                    arg_count: *arg_count,
                    local_count: *local_count,
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
                let frame = CVMCallFrame::from(func_obj.clone());

                for i in 0..frame.arg_count {
                    frame.variables.set(
                        i,
                        self.stack.pop().expect("expected an object on the stack"),
                    )
                }

                self.call_stack.push(frame);
                self.execute(&func_obj.code);
                current_idx
            }

            Bytecode::Return => {
                self.call_stack.pop();
                code.len()
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
