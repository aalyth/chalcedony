use crate::common::Type;
use crate::utils::PtrString;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::iter::zip;
use std::rc::{Rc, Weak};

pub type CvmList = Rc<RefCell<VecDeque<CvmObject>>>;
pub type CvmObj = Gc<Vec<CvmObject>>;

#[derive(Debug)]
pub struct GcInner<Data> {
    pub data: Data,
    pub depth: usize,
}

impl<Data> GcInner<Data> {
    pub fn new(data: Data) -> Self {
        GcInner { data, depth: 0 }
    }
}

// This implementation of an RC-based Garbage Collection could be optimized a
// bit, reimplementing a custom modified version of Reference Counted (RC)
// pointers, but the benefits of such optimization are in a matter of bytes.
#[derive(Debug)]
pub enum Gc<Data> {
    Strong(Rc<RefCell<GcInner<Data>>>),
    Weak(Weak<RefCell<GcInner<Data>>>),
}

impl<Data> Gc<Data> {
    pub fn new(data: Data) -> Self {
        Gc::Strong(Rc::new(RefCell::new(GcInner::new(data))))
    }

    pub fn get_ref(&self) -> Rc<RefCell<GcInner<Data>>> {
        match self {
            Gc::Strong(obj_ref) => obj_ref.clone(),
            Gc::Weak(weak_ref) => weak_ref
                .upgrade()
                .expect("weak ref to a deallocated object"),
        }
    }
}

impl<Data> Clone for Gc<Data> {
    fn clone(&self) -> Self {
        let obj = self.get_ref();
        obj.borrow_mut().depth += 1;
        Gc::Strong(obj.clone())
    }
}

impl<Data> Drop for Gc<Data> {
    fn drop(&mut self) {
        // This warning is not relevant to the current implementation. Using
        // value references instead of actual values still achieves the expected
        // result - decrementing the RC's strong reference count.
        #[allow(dropping_references)]
        match self {
            Gc::Strong(obj) => {
                let mut obj = obj.borrow_mut();
                if obj.depth > 0 {
                    obj.depth -= 1;
                }
                drop(obj)
            }
            Gc::Weak(weak) => {
                if let Some(obj) = weak.upgrade() {
                    let mut obj = obj.borrow_mut();
                    if obj.depth > 0 {
                        obj.depth -= 1;
                    }
                }
                drop(weak)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum CvmObject {
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(PtrString),
    Bool(bool),
    List(CvmList),
    Exception(PtrString),
    Object(CvmObj),
}

impl CvmObject {
    /* used for debugging */
    pub fn as_type(&self) -> Type {
        match self {
            CvmObject::Int(_) => Type::Int,
            CvmObject::Uint(_) => Type::Uint,
            CvmObject::Float(_) => Type::Float,
            CvmObject::Str(_) => Type::Str,
            CvmObject::Bool(_) => Type::Bool,
            CvmObject::List(list) => {
                let list = list.borrow();
                if let Some(obj) = list.front() {
                    return Type::List(Box::new(obj.as_type()));
                }
                Type::List(Box::new(Type::Any))
            }
            CvmObject::Exception(_) => Type::Exception,
            CvmObject::Object(_) => Type::Custom(Box::new("Object".to_string())),
        }
    }

    pub fn deep_copy(self) -> Self {
        match self {
            CvmObject::List(data) => {
                if Rc::strong_count(&data) == 1 {
                    CvmObject::List(data)
                } else {
                    let data = data.borrow();
                    let mut new_vec = VecDeque::<CvmObject>::with_capacity(data.len());
                    for el in data.clone().into_iter() {
                        new_vec.push_back(el.deep_copy());
                    }
                    CvmObject::List(Rc::new(RefCell::new(new_vec)))
                }
            }
            CvmObject::Object(obj) => {
                let obj_ref = obj.get_ref();
                let obj_ref = obj_ref.borrow();
                if obj_ref.depth == 0 {
                    CvmObject::Object(obj)
                } else {
                    let mut new_vec = Vec::<CvmObject>::with_capacity(obj_ref.data.len());
                    for el in obj_ref.data.clone().into_iter() {
                        new_vec.push(el.deep_copy());
                    }
                    CvmObject::Object(Gc::new(new_vec))
                }
            }
            _ => self,
        }
    }
}

impl Default for CvmObject {
    fn default() -> Self {
        Self::Int(0)
    }
}

impl std::fmt::Display for CvmObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CvmObject::Int(val) => write!(f, "{}", val),
            CvmObject::Uint(val) => write!(f, "{}", val),
            CvmObject::Float(val) => write!(f, "{}", val),
            CvmObject::Str(val) => write!(f, "{}", val),
            CvmObject::Bool(val) => write!(f, "{}", val),
            CvmObject::List(list) => {
                write!(f, "[")?;
                let list = list.borrow();
                for el in list.iter() {
                    write!(f, "{}, ", el)?;
                }
                /* `\x08` is the same as `\b` */
                if !list.is_empty() {
                    write!(f, "\x08\x08")?;
                }
                write!(f, "]")
            }
            CvmObject::Exception(val) => write!(f, "{}", val),
            CvmObject::Object(obj) => {
                let obj = obj.get_ref();
                let obj = obj.borrow();
                write!(f, "{{")?;
                for val in &*obj.data {
                    write!(f, "{}, ", val)?;
                }
                /* `\x08` is the same as `\b` */
                write!(f, "\x08\x08}}")
            }
        }
    }
}

impl std::cmp::PartialEq for CvmObject {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CvmObject::Int(lval), CvmObject::Int(rval)) => lval == rval,
            (CvmObject::Uint(lval), CvmObject::Uint(rval)) => lval == rval,
            // due to float imprecisions, they are checked up to 10^12 precision
            (CvmObject::Float(lval), CvmObject::Float(rval)) => {
                (lval - rval).abs() < 0.000_000_000_000_1
            }
            (CvmObject::Str(lval), CvmObject::Str(rval)) => lval == rval,
            (CvmObject::Bool(lval), CvmObject::Bool(rval)) => lval == rval,
            (CvmObject::List(left), CvmObject::List(right)) => {
                let left = left.borrow();
                let right = right.borrow();
                if left.len() != right.len() {
                    return false;
                }

                for (l, r) in zip(left.iter(), right.iter()) {
                    if l != r {
                        return false;
                    }
                }
                true
            }
            (CvmObject::Object(left), CvmObject::Object(right)) => {
                let left = left.get_ref();
                let left = left.borrow();

                let right = right.get_ref();
                let right = right.borrow();

                for (l, r) in zip(left.data.iter(), right.data.iter()) {
                    if l != r {
                        return false;
                    }
                }
                true
            }
            _ => false,
        }
    }
}
