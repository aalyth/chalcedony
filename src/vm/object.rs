use crate::lexer::Type;

#[derive(Debug, Clone)]
pub enum CVMObject {
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(String),
    Bool(bool),
}

impl CVMObject {
    pub fn to_type(&self) -> Type {
        match self {
            CVMObject::Int(_) => Type::Int,
            CVMObject::Uint(_) => Type::Uint,
            CVMObject::Float(_) => Type::Float,
            CVMObject::Str(_) => Type::Str,
            CVMObject::Bool(_) => Type::Bool,
        }
    }
}
