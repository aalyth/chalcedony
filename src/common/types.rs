use super::Bytecode;
use crate::error::{span::Span, ChalError, CompileError, CompileErrorKind};

use std::fmt;

/// The structure, representing a type inside the interpreter. Used to assert
/// the type strictness of the script before it's execution.
#[derive(PartialEq, Debug, Clone, Default)]
pub enum Type {
    Int,
    Uint,
    Float,
    Str,
    Bool,
    Any,
    #[default]
    Void,
    List(Box<Type>),
    Exception,
    Custom(Box<String>),
}

impl Type {
    /// NOTE: it is very important that this function goes after value calls
    /// inside the result.
    pub fn verify(
        exp: Type,
        recv: Type,
        code: &mut Vec<Bytecode>,
        span: Span,
    ) -> Result<(), ChalError> {
        match (exp, recv) {
            (Type::Any, _) => Ok(()),
            (Type::Int, Type::Uint) => {
                code.push(Bytecode::CastI);
                Ok(())
            }
            (Type::Float, Type::Int) | (Type::Float, Type::Uint) => {
                code.push(Bytecode::CastF);
                Ok(())
            }
            (exp @ Type::List(_), recv @ Type::List(_)) => {
                if !Type::list_eq(&exp, &recv) {
                    return Err(
                        CompileError::new(CompileErrorKind::InvalidType(exp, recv), span).into(),
                    );
                }
                Ok(())
            }
            (exp, recv) => {
                if exp == recv {
                    return Ok(());
                }
                Err(CompileError::new(CompileErrorKind::InvalidType(exp, recv), span).into())
            }
        }
    }

    // Used to compare the types between list elements.
    pub fn implicit_eq(left: &Type, right: &Type) -> bool {
        if *left == Type::Any || *right == Type::Any {
            return true;
        }

        match (left, right) {
            (Type::List(lhs), Type::List(rhs)) => Type::implicit_eq(lhs, rhs),
            _ => left == right,
        }
    }

    /// Used to check whether an overloaded function's definition is applicable
    /// to the passed argument's types.
    pub fn soft_eq(&self, other: &Self) -> bool {
        match (self, other) {
            /* universal types */
            (Type::Void, _) | (_, Type::Void) => false,
            (Type::Any, _) => true,
            /* actual types */
            (Type::Int, Type::Int)
            | (Type::Uint, Type::Uint)
            | (Type::Float, Type::Float)
            | (Type::Str, Type::Str)
            | (Type::Bool, Type::Bool) => true,
            (Type::Custom(lhs), Type::Custom(rhs)) => lhs == rhs,
            (Type::List(lhs), Type::List(rhs)) => Type::implicit_eq(lhs, rhs),
            /* implicit type casts */
            (Type::Int, Type::Uint) => true,
            _ => false,
        }
    }

    pub fn as_class(&self) -> String {
        match self {
            Type::Int => "Int".to_string(),
            Type::Uint => "Uint".to_string(),
            Type::Float => "Float".to_string(),
            Type::Str => "String".to_string(),
            Type::Bool => "Bool".to_string(),
            Type::Exception => "Exception".to_string(),
            Type::List(_) => "List".to_string(),
            Type::Custom(class) => *class.clone(),
            Type::Any => "Any".to_string(),
            Type::Void => "Void".to_string(),
        }
    }
    // Used to retrieve the bottom type of a list type.
    pub fn root_type(&self) -> Type {
        match self {
            Type::List(ty) => ty.root_type(),
            _ => self.clone(),
        }
    }

    // Used to compare lists recursively. The left list could be an internal
    // type expectation (Type::List(Type::Any)).
    fn list_eq(left: &Type, right: &Type) -> bool {
        match (left, right) {
            (Type::Any, _) | (_, Type::Any) => true,
            (Type::List(lhs), Type::List(rhs)) => Type::list_eq(lhs, rhs),
            (left, right) => left == right,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Uint => write!(f, "uint"),
            Type::Float => write!(f, "float"),
            Type::Str => write!(f, "str"),
            Type::Bool => write!(f, "bool"),
            Type::Any => write!(f, "any"),
            Type::Void => write!(f, "void"),
            Type::Exception => write!(f, "exception"),
            Type::List(ty) => write!(f, "[{}]", ty),
            Type::Custom(ty) => write!(f, "{}", ty),
        }
    }
}
