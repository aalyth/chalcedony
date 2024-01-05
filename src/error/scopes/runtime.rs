use crate::error::span::Span;
use crate::lexer::Type;

use super::display_err;

enum RuntimeErrorKind {
    UnknownVariable(String),
    UnknownFunction(String),
    InvalidOperation(Type, Type), /* lhs, rhs */
    InvalidType(Type, Type),      /* exp, recv */
    InvalidUnOperation(Type),
    TooManyArguments(usize, usize), /* exp, recv */
    TooFewArguments(usize, usize),  /* exp, recv*/
    NonVoidFunctionStmnt(Type),
    VoidFunctionExpr,
    NoDefaultReturnStmnt,
}

pub struct RuntimeError {
    kind: RuntimeErrorKind,
    span: Span,
}

impl RuntimeError {
    fn new(kind: RuntimeErrorKind, span: Span) -> Self {
        RuntimeError { span, kind }
    }

    pub fn unknown_variable(var: String, span: Span) -> Self {
        RuntimeError::new(RuntimeErrorKind::UnknownVariable(var), span)
    }

    pub fn unknown_function(func: String, span: Span) -> Self {
        RuntimeError::new(RuntimeErrorKind::UnknownFunction(func), span)
    }

    pub fn invalid_operation(lhs: Type, rhs: Type, span: Span) -> Self {
        RuntimeError::new(RuntimeErrorKind::InvalidOperation(lhs, rhs), span)
    }

    pub fn invalid_type(exp: Type, recv: Type, span: Span) -> Self {
        RuntimeError::new(RuntimeErrorKind::InvalidType(exp, recv), span)
    }

    pub fn invalid_un_operation(ty: Type, span: Span) -> Self {
        RuntimeError::new(RuntimeErrorKind::InvalidUnOperation(ty), span)
    }

    pub fn too_many_arguments(exp: usize, recv: usize, span: Span) -> Self {
        RuntimeError::new(RuntimeErrorKind::TooManyArguments(exp, recv), span)
    }

    pub fn too_few_arguments(exp: usize, recv: usize, span: Span) -> Self {
        RuntimeError::new(RuntimeErrorKind::TooFewArguments(exp, recv), span)
    }

    pub fn non_void_func_stmnt(ty: Type, span: Span) -> Self {
        RuntimeError::new(RuntimeErrorKind::NonVoidFunctionStmnt(ty), span)
    }

    pub fn void_func_expr(span: Span) -> Self {
        RuntimeError::new(RuntimeErrorKind::VoidFunctionExpr, span)
    }

    pub fn no_default_return_stmnt(span: Span) -> Self {
        RuntimeError::new(RuntimeErrorKind::NoDefaultReturnStmnt, span)
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            RuntimeErrorKind::UnknownVariable(var) => {
                let msg = &format!("unknown variable '{}'", var);
                display_err(&self.span, f, msg)
            }

            RuntimeErrorKind::UnknownFunction(func) => {
                let msg = &format!("unknown function '{}()'", func);
                display_err(&self.span, f, msg)
            }

            RuntimeErrorKind::InvalidOperation(lhs, rhs) => {
                let msg = &format!("invalid operation between {:?} and {:?}", lhs, rhs);
                display_err(&self.span, f, msg)
            }

            RuntimeErrorKind::InvalidType(exp, recv) => {
                let msg = &format!(
                    "invalid expression type (expected {:?}, received {:?})",
                    exp, recv
                );
                display_err(&self.span, f, msg)
            }

            RuntimeErrorKind::InvalidUnOperation(ty) => {
                let msg = &format!("invalid unary operation on type {:?}", ty);
                display_err(&self.span, f, msg)
            }

            RuntimeErrorKind::TooManyArguments(exp, recv) => {
                let msg = &format!(
                    "too many arguments to function call (expected {exp}, received {recv})"
                );
                display_err(&self.span, f, msg)
            }

            RuntimeErrorKind::TooFewArguments(exp, recv) => {
                let msg = &format!(
                    "too few arguments to function call (expected {exp}, received {recv})"
                );
                display_err(&self.span, f, msg)
            }

            RuntimeErrorKind::NonVoidFunctionStmnt(ty) => {
                let msg = &format!("calling a non-void ({:?}) function in a statement", ty);
                display_err(&self.span, f, msg)
            }

            RuntimeErrorKind::VoidFunctionExpr => display_err(
                &self.span,
                f,
                "calling a void function inside an expression",
            ),

            RuntimeErrorKind::NoDefaultReturnStmnt => {
                display_err(&self.span, f, "no default return statement inside function")
            }
        }
    }
}
