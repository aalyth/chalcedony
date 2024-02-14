use crate::common::Type;
use crate::utils::PtrString;

#[derive(Debug, Clone, PartialEq)]
pub enum CvmObject {
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(PtrString),
    Bool(bool),
}

impl CvmObject {
    pub fn as_type(&self) -> Type {
        match self {
            CvmObject::Int(_) => Type::Int,
            CvmObject::Uint(_) => Type::Uint,
            CvmObject::Float(_) => Type::Float,
            CvmObject::Str(_) => Type::Str,
            CvmObject::Bool(_) => Type::Bool,
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
        }
    }
}
