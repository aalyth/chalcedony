use super::Bytecode;
use crate::error::{span::Span, ChalError, CompileError};

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Int,
    Uint,
    Float,
    Str,
    Bool,
    Any,
    Void,
    List(Box<Type>),
}

impl Type {
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

    /* compares the two types implicitly using `Type::Any` as wildcards */
    pub fn implicit_eq(left: &Type, right: &Type) -> bool {
        if *left == Type::Any || *right == Type::Any {
            return true;
        }

        match (left, right) {
            (Type::List(lhs), Type::List(rhs)) => lhs == rhs,
            _ => left == right,
        }
    }
}
