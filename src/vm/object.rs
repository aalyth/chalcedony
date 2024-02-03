use crate::common::Type;
use crate::utils::PtrString;

#[derive(Debug, Clone)]
pub enum CVMObject {
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(PtrString),
    Bool(bool),
}

impl CVMObject {
    pub fn as_type(&self) -> Type {
        match self {
            CVMObject::Int(_) => Type::Int,
            CVMObject::Uint(_) => Type::Uint,
            CVMObject::Float(_) => Type::Float,
            CVMObject::Str(_) => Type::Str,
            CVMObject::Bool(_) => Type::Bool,
        }
    }
}

impl Default for CVMObject {
    fn default() -> Self {
        Self::Int(0)
    }
}

impl std::fmt::Display for CVMObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CVMObject::Int(val) => write!(f, "{}", val),
            CVMObject::Uint(val) => write!(f, "{}", val),
            CVMObject::Float(val) => write!(f, "{}", val),
            CVMObject::Str(val) => write!(f, "{}", val),
            CVMObject::Bool(val) => write!(f, "{}", val),
        }
    }
}
