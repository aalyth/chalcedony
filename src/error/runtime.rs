use crate::error::format::err;
use crate::error::{Position, Span};
use crate::lexer::Type;

use std::rc::Rc;

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
    start: Position,
    end: Position,
    span: Rc<Span>,
    kind: RuntimeErrorKind,
}

impl RuntimeError {
    fn new(kind: RuntimeErrorKind, start: Position, end: Position, span: Rc<Span>) -> Self {
        RuntimeError {
            start,
            end,
            span,
            kind,
        }
    }

    pub fn unknown_variable(var: String, start: Position, end: Position, span: Rc<Span>) -> Self {
        RuntimeError::new(RuntimeErrorKind::UnknownVariable(var), start, end, span)
    }

    pub fn unknown_function(func: String, start: Position, end: Position, span: Rc<Span>) -> Self {
        RuntimeError::new(RuntimeErrorKind::UnknownFunction(func), start, end, span)
    }

    pub fn invalid_operation(
        lhs: Type,
        rhs: Type,
        start: Position,
        end: Position,
        span: Rc<Span>,
    ) -> Self {
        RuntimeError::new(
            RuntimeErrorKind::InvalidOperation(lhs, rhs),
            start,
            end,
            span,
        )
    }

    pub fn invalid_type(
        exp: Type,
        recv: Type,
        start: Position,
        end: Position,
        span: Rc<Span>,
    ) -> Self {
        RuntimeError::new(RuntimeErrorKind::InvalidType(exp, recv), start, end, span)
    }

    pub fn invalid_un_operation(ty: Type, start: Position, end: Position, span: Rc<Span>) -> Self {
        RuntimeError::new(RuntimeErrorKind::InvalidUnOperation(ty), start, end, span)
    }

    pub fn too_many_arguments(
        exp: usize,
        recv: usize,
        start: Position,
        end: Position,
        span: Rc<Span>,
    ) -> Self {
        RuntimeError::new(
            RuntimeErrorKind::TooManyArguments(exp, recv),
            start,
            end,
            span,
        )
    }

    pub fn too_few_arguments(
        exp: usize,
        recv: usize,
        start: Position,
        end: Position,
        span: Rc<Span>,
    ) -> Self {
        RuntimeError::new(
            RuntimeErrorKind::TooFewArguments(exp, recv),
            start,
            end,
            span,
        )
    }

    pub fn non_void_func_stmnt(ty: Type, start: Position, end: Position, span: Rc<Span>) -> Self {
        RuntimeError::new(RuntimeErrorKind::NonVoidFunctionStmnt(ty), start, end, span)
    }

    pub fn void_func_expr(start: Position, end: Position, span: Rc<Span>) -> Self {
        RuntimeError::new(RuntimeErrorKind::VoidFunctionExpr, start, end, span)
    }

    pub fn no_default_return_stmnt(start: Position, end: Position, span: Rc<Span>) -> Self {
        RuntimeError::new(RuntimeErrorKind::NoDefaultReturnStmnt, start, end, span)
    }

    // TODO: extract this function to a more generic one
    fn display_err(&self, f: &mut std::fmt::Formatter, msg: &str) -> std::fmt::Result {
        write!(
            f,
            "{}:\n{}\n",
            err(msg),
            self.span.context(&self.start, &self.end)
        )
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            RuntimeErrorKind::UnknownVariable(var) => {
                let msg = &format!("unknown variable '{}'", var);
                self.display_err(f, msg)
            }

            RuntimeErrorKind::UnknownFunction(func) => {
                let msg = &format!("unknown function '{}()'", func);
                self.display_err(f, msg)
            }

            RuntimeErrorKind::InvalidOperation(lhs, rhs) => {
                let msg = &format!("invalid operation between {:?} and {:?}", lhs, rhs);
                self.display_err(f, msg)
            }

            RuntimeErrorKind::InvalidType(exp, recv) => {
                let msg = &format!(
                    "invalid expression type (expected {:?}, received {:?})",
                    exp, recv
                );
                self.display_err(f, msg)
            }

            RuntimeErrorKind::InvalidUnOperation(ty) => {
                let msg = &format!("invalid unary operation on type {:?}", ty);
                self.display_err(f, msg)
            }

            RuntimeErrorKind::TooManyArguments(exp, recv) => {
                let msg = &format!(
                    "too many arguments to function call (expected {exp}, received {recv})"
                );
                self.display_err(f, msg)
            }

            RuntimeErrorKind::TooFewArguments(exp, recv) => {
                let msg = &format!(
                    "too few arguments to function call (expected {exp}, received {recv})"
                );
                self.display_err(f, msg)
            }

            RuntimeErrorKind::NonVoidFunctionStmnt(ty) => {
                let msg = &format!("calling a non-void ({:?}) function in a statement", ty);
                self.display_err(f, msg)
            }

            RuntimeErrorKind::VoidFunctionExpr => {
                self.display_err(f, "calling a void function inside an expression")
            }

            RuntimeErrorKind::NoDefaultReturnStmnt => {
                self.display_err(f, "no default return statement inside function")
            }
        }
    }
}
