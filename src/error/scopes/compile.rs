use super::display_err;
use crate::error::span::Span;

use crate::common::Type;

enum CompileErrorKind {
    UnknownVariable(String),
    UnknownFunction(String),
    InvalidOperation(Type, Type), /* lhs, rhs */
    InvalidType(Type, Type),      /* exp, recv */
    InvalidUnOperation(Type),
    TooManyArguments(usize, usize), /* exp, recv */
    TooFewArguments(usize, usize),  /* exp, recv */
    NonVoidFunctionStmnt(Type),
    VoidFunctionExpr,
    NoDefaultReturnStmnt,
    StatefulFunction,
    RedefiningFunctionArg,
}

pub struct CompileError {
    kind: CompileErrorKind,
    span: Span,
}

impl CompileError {
    fn new(kind: CompileErrorKind, span: Span) -> Self {
        CompileError { span, kind }
    }

    pub fn unknown_variable(var: String, span: Span) -> Self {
        CompileError::new(CompileErrorKind::UnknownVariable(var), span)
    }

    pub fn unknown_function(func: String, span: Span) -> Self {
        CompileError::new(CompileErrorKind::UnknownFunction(func), span)
    }

    pub fn invalid_operation(lhs: Type, rhs: Type, span: Span) -> Self {
        CompileError::new(CompileErrorKind::InvalidOperation(lhs, rhs), span)
    }

    pub fn invalid_type(exp: Type, recv: Type, span: Span) -> Self {
        CompileError::new(CompileErrorKind::InvalidType(exp, recv), span)
    }

    pub fn invalid_un_operation(ty: Type, span: Span) -> Self {
        CompileError::new(CompileErrorKind::InvalidUnOperation(ty), span)
    }

    pub fn too_many_arguments(exp: usize, recv: usize, span: Span) -> Self {
        CompileError::new(CompileErrorKind::TooManyArguments(exp, recv), span)
    }

    pub fn too_few_arguments(exp: usize, recv: usize, span: Span) -> Self {
        CompileError::new(CompileErrorKind::TooFewArguments(exp, recv), span)
    }

    pub fn non_void_func_stmnt(ty: Type, span: Span) -> Self {
        CompileError::new(CompileErrorKind::NonVoidFunctionStmnt(ty), span)
    }

    pub fn void_func_expr(span: Span) -> Self {
        CompileError::new(CompileErrorKind::VoidFunctionExpr, span)
    }

    pub fn no_default_return_stmnt(span: Span) -> Self {
        CompileError::new(CompileErrorKind::NoDefaultReturnStmnt, span)
    }

    pub fn stateful_function(span: Span) -> Self {
        CompileError::new(CompileErrorKind::StatefulFunction, span)
    }

    pub fn redefining_function_arg(span: Span) -> Self {
        CompileError::new(CompileErrorKind::RedefiningFunctionArg, span)
    }
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            CompileErrorKind::UnknownVariable(var) => {
                let msg = &format!("unknown variable '{}'", var);
                display_err(&self.span, f, msg)
            }

            CompileErrorKind::UnknownFunction(func) => {
                let msg = &format!("unknown function '{}()'", func);
                display_err(&self.span, f, msg)
            }

            CompileErrorKind::InvalidOperation(lhs, rhs) => {
                let msg = &format!("invalid operation between {:?} and {:?}", lhs, rhs);
                display_err(&self.span, f, msg)
            }

            CompileErrorKind::InvalidType(exp, recv) => {
                let msg = &format!(
                    "invalid expression type (expected {:?}, received {:?})",
                    exp, recv
                );
                display_err(&self.span, f, msg)
            }

            CompileErrorKind::InvalidUnOperation(ty) => {
                let msg = &format!("invalid unary operation on type {:?}", ty);
                display_err(&self.span, f, msg)
            }

            CompileErrorKind::TooManyArguments(exp, recv) => {
                let msg = &format!(
                    "too many arguments to function call (expected {exp}, received {recv})"
                );
                display_err(&self.span, f, msg)
            }

            CompileErrorKind::TooFewArguments(exp, recv) => {
                let msg = &format!(
                    "too few arguments to function call (expected {exp}, received {recv})"
                );
                display_err(&self.span, f, msg)
            }

            CompileErrorKind::NonVoidFunctionStmnt(ty) => {
                let msg = &format!("calling a non-void ({:?}) function in a statement", ty);
                display_err(&self.span, f, msg)
            }

            CompileErrorKind::VoidFunctionExpr => display_err(
                &self.span,
                f,
                "calling a void function inside an expression",
            ),

            CompileErrorKind::NoDefaultReturnStmnt => {
                display_err(&self.span, f, "no default return statement inside function")
            }

            CompileErrorKind::StatefulFunction => display_err(
                &self.span,
                f,
                "functions with external state are not allowed",
            ),

            CompileErrorKind::RedefiningFunctionArg => {
                display_err(&self.span, f, "redefining the function's argument")
            }
        }
    }
}
