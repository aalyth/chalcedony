use super::Bytecode;
use crate::error::{span::Span, ChalError, CompileError};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Type {
    Int,
    Uint,
    Float,
    Str,
    Bool,
    Any,
    Void,
    Exception,
}

impl Type {
    /* NOTE: it is very important that this function goes after value calls inside the result */
    pub fn verify(
        exp: Type,
        recv: Type,
        code: &mut Vec<Bytecode>,
        span: Span,
    ) -> Result<(), ChalError> {
        if exp == Type::Any || exp == recv {
            return Ok(());
        }
        if exp == Type::Int && recv == Type::Uint {
            code.push(Bytecode::CastI);
            return Ok(());
        }

        if exp == Type::Float && (recv == Type::Uint || recv == Type::Int) {
            code.push(Bytecode::CastF);
            return Ok(());
        }

        Err(CompileError::invalid_type(exp, recv, span).into())
    }

    pub fn soft_eq(&self, other: &Self) -> bool {
        match (self, other) {
            /* Universal types */
            (Type::Void, _) | (_, Type::Void) => false,
            (Type::Any, _) => true,
            /* Actual types */
            (Type::Int, Type::Int)
            | (Type::Uint, Type::Uint)
            | (Type::Float, Type::Float)
            | (Type::Str, Type::Str)
            | (Type::Bool, Type::Bool) => true,
            /* type casts */
            (Type::Int, Type::Uint) => true,
            // (Type::Float, Type::Uint) | (Type::Float, Type::Int) => true,
            _ => false,
        }
    }
}
