use crate::common::Type;
use crate::utils::PtrString;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

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
    Exception(PtrString),
    /* an instance of a class */
    Instance(Gc<Vec<CvmObject>>),
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
            CvmObject::Exception(_) => Type::Exception,
            CvmObject::Instance(_) => Type::Custom(Box::new("Object".to_string())),
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
            CvmObject::Exception(val) => write!(f, "{}", val),
            CvmObject::Instance(obj) => {
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
            _ => false,
        }
    }
}
