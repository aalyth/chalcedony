use crate::common::Type;
use crate::utils::PtrString;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum CvmObject {
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(PtrString),
    Bool(bool),
    List(Rc<RefCell<VecDeque<CvmObject>>>),
}

impl CvmObject {
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
                write!(f, "\x08\x08]")
            }
        }
    }
}

impl std::cmp::PartialEq for CvmObject {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CvmObject::Int(lval), CvmObject::Int(rval)) => lval == rval,
            (CvmObject::Uint(lval), CvmObject::Uint(rval)) => lval == rval,
            /* due to floating imprecisions, they are checked with precision of 10^12 */
            (CvmObject::Float(lval), CvmObject::Float(rval)) => {
                (lval - rval).abs() < 0.000_000_000_000_1
            }
            (CvmObject::Str(lval), CvmObject::Str(rval)) => lval == rval,
            (CvmObject::Bool(lval), CvmObject::Bool(rval)) => lval == rval,
            _ => false,
        }
    }
}
