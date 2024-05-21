//! The `Chalcedony Virtual Machine (CVM)`, responsible for the execution of the
//! compiled bytecode instructions.
//!
//! An integral part of the virtual machine is that it avoids the usage of
//! denoting runtime errors - the philosophy of Chalcedony is that the code
//! should've been checked during it's compilation, not during it's execution.
//!
//! With regards to the implementation, CVM is a stack-based virtual machine,
//! meaning all expressions are computed on the [`stack`].

mod builtins;
mod object;

use builtins::{
    add, and, div, eq, gt, gt_eq, list_get, list_insert, list_remove, list_set, lt, lt_eq, modulo,
    mul, neg, not, or, sub,
};
use object::{CvmList, CvmObject, Gc};

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
        $cvm.stack.push(CvmObject::$type($val));
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
        let next_instr: Bytecode;
        if let Some(frame) = self.call_stack.peek() {
            next_instr = frame
                .code
                .get(current_idx)
                .expect("invalid current idx")
                .clone();
        } else {
            next_instr = code.get(current_idx).expect("invalid current idx").clone();
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

            Bytecode::ConstL(len) => {
                let mut list = VecDeque::<CvmObject>::with_capacity(len);
                for _ in 0..len {
                    list.push_front(self.stack.pop().expect("expected a value on the stack"));
                }
                self.stack
                    .push(CvmObject::List(Rc::new(RefCell::new(list))));
                next_idx
            }

            Bytecode::ConstObj(el_count) => {
                self.stack.push(CvmObject::Object(Gc::new(vec![
                    CvmObject::default();
                    el_count
                ])));
                next_idx
            }

            Bytecode::ThrowException => {
                let obj = self.pop();
                let CvmObject::Str(val) = obj else {
                    panic!("invalid exception type");
                };
                self.stack.push(CvmObject::Exception(val));
                self.handle_exception()
            }

            Bytecode::Dup => {
                let val = self.stack.peek().expect("expected a value on the stack");
                self.stack.push(val.clone());
                next_idx
            }

            Bytecode::Copy => {
                let val = self.pop();
                self.stack.push(val.deep_copy());
                next_idx
            }

            Bytecode::Pop => {
                self.pop();
                next_idx
            }

            Bytecode::CastI => {
                match self.pop() {
                    CvmObject::Uint(val) => self.stack.push(CvmObject::Int(val as i64)),
                    CvmObject::Float(val) => self.stack.push(CvmObject::Int(val as i64)),
                    _ => panic!("invalid cast to Int"),
                };
                next_idx
            }

            Bytecode::CastF => {
                match self.pop() {
                    CvmObject::Int(val) => self.stack.push(CvmObject::Float(val as f64)),
                    CvmObject::Uint(val) => self.stack.push(CvmObject::Float(val as f64)),
                    _ => panic!("invalid cast to Float"),
                }
                next_idx
            }

            Bytecode::CastU => {
                match self.pop() {
                    CvmObject::Int(val) => self.stack.push(CvmObject::Uint(val as u64)),
                    CvmObject::Float(val) => self.stack.push(CvmObject::Uint(val as u64)),
                    _ => panic!("invalid cast to Uint"),
                }
                next_idx
            }

            Bytecode::SetGlobal(var_id) => {
                let var_value = self.pop();
                while var_id >= self.globals.len() {
                    self.globals.push(CvmObject::Int(0));
                }
                *self.globals.get_mut(var_id).unwrap() = var_value;
                next_idx
            }
            Bytecode::GetGlobal(var_id) => {
                let var_value = self
                    .globals
                    .get(var_id)
                    .expect("expected a valid variable id")
                    .clone();
                self.stack.push(var_value);
                next_idx
            }

            Bytecode::SetLocal(mut var_id) => {
                if let Some(frame) = self.call_stack.peek() {
                    var_id += frame.stack_len;
                }

                let value = self.pop();
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
                    var_id += frame.stack_len;
                }
                let value = self.stack.get(var_id).unwrap().clone();
                self.stack.push(value);
                next_idx
            }

            Bytecode::GetAttr(attr_id) => {
                let CvmObject::Object(obj) =
                    self.stack.pop().expect("expected a value on the stack")
                else {
                    panic!("calling `GetAttr` on a non-object")
                };

                let obj = obj.get_ref();
                self.stack
                    .push(obj.borrow().data.get(attr_id).unwrap().clone());

                next_idx
            }

            Bytecode::SetAttr(attr_id) => {
                let val = self.stack.pop().expect("expected a value on the stack");
                let CvmObject::Object(dest_obj) =
                    self.stack.top().expect("expected a value on the stack")
                else {
                    panic!("calling `SetAttr` on a non-object");
                };

                let dest_obj = dest_obj.get_ref();
                let mut dest_obj = dest_obj.borrow_mut();

                /* dest_obj = val_obj */
                if let CvmObject::Object(val_obj) = val {
                    let val_obj = val_obj.get_ref();

                    // The `depth > 1` check is so if an inline object is set as
                    // an object's attribute, the subobject is not deallocated
                    // right after the parent object's initialization.
                    if dest_obj.depth == 0 && val_obj.borrow().depth == 0 {
                        *dest_obj.data.get_mut(attr_id).unwrap() =
                            CvmObject::Object(Gc::Strong(val_obj.clone()));
                    } else if dest_obj.depth <= val_obj.borrow().depth {
                        *dest_obj.data.get_mut(attr_id).unwrap() =
                            CvmObject::Object(Gc::Weak(Rc::downgrade(&val_obj)));
                    } else {
                        *dest_obj.data.get_mut(attr_id).unwrap() =
                            CvmObject::Object(Gc::Strong(val_obj.clone()));
                        dest_obj.depth = val_obj.borrow().depth;
                    }
                } else {
                    *dest_obj.data.get_mut(attr_id).unwrap() = val;
                }

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
                    arg_count,
                    code: Rc::new(code[next_idx..].into()),
                };
                self.functions.push(Rc::new(func_obj));
                code.len()
            }

            Bytecode::CallFunc(func_id) => {
                let func_obj = self
                    .functions
                    .get(func_id)
                    .expect("expected a valid function id")
                    .clone();

                // NOTE: the arguments to the function call are already in place
                // and local variables are automatically handled

                let frame = CvmCallFrame {
                    prev_idx: next_idx,
                    stack_len: self.stack.len() - func_obj.arg_count,
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
                self.catch_idx = frame.catch_idx;
                frame.prev_idx
            }

            Bytecode::ReturnVoid => {
                let frame = self.call_stack.pop().unwrap();
                self.stack.truncate(frame.stack_len);
                self.catch_idx = frame.catch_idx;
                frame.prev_idx
            }

            Bytecode::If(jmp) => {
                let CvmObject::Bool(cond) = self.pop() else {
                    panic!("expected a bool when checking if statements")
                };

                if !cond {
                    return next_idx + jmp;
                }
                /* execute the body if the condition is true */
                next_idx
            }

            Bytecode::Jmp(dist) => (next_idx as isize + dist) as usize,

            Bytecode::Len => {
                let obj = self.pop();
                match obj {
                    CvmObject::List(list) => {
                        self.stack.push(CvmObject::Uint(list.borrow().len() as u64))
                    }
                    CvmObject::Str(val) => self.stack.push(CvmObject::Uint(val.len() as u64)),
                    _ => panic!("getting the length of non string/list"),
                }
                next_idx
            }

            Bytecode::ListInsert => list_insert(self, next_idx),
            Bytecode::ListRemove => list_remove(self, next_idx),
            Bytecode::ListSet => list_set(self, next_idx),
            Bytecode::ListGet => list_get(self, next_idx),

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
                let obj = self.pop();
                println!("{}", obj);
                next_idx
            }

            Bytecode::Assert => {
                let CvmObject::Bool(successful) = self.pop() else {
                    self.stack
                        .push(CvmObject::Exception("assertion failed".to_string().into()));
                    return self.handle_exception();
                };

                if !successful {
                    self.stack
                        .push(CvmObject::Exception("assertion failed".to_string().into()));
                    return self.handle_exception();
                }

                next_idx
            }

            Bytecode::Nop => next_idx,
        }
    }

    #[inline(always)]
    fn push(&mut self, val: CvmObject) {
        self.stack.push(val)
    }

    #[inline(always)]
    fn pop(&mut self) -> CvmObject {
        self.stack.pop().expect("expected a value on the stack")
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
