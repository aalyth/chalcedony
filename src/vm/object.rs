use crate::common::Type;
use crate::utils::PtrString;

#[derive(Debug, Clone)]
pub enum CvmObject {
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(PtrString),
    Bool(bool),
    Exception(PtrString),
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
