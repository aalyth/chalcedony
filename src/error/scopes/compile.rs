use super::display_err;
use crate::error::span::Span;

use crate::common::Type;

enum CompileErrorKind {
    UnknownVariable(String),
    UnknownFunction(String),
    InvalidBinOpr(String, Type, Type), /* opr_name, lhs, rhs */
    InvalidUnaryOpr(String, Type),     /* opr_name, val*/
    InvalidType(Type, Type),           /* exp, recv */
    TooManyArguments(usize, usize),    /* exp, recv */
    TooFewArguments(usize, usize),     /* exp, recv */
    NonVoidFunctionStmnt(Type),
    VoidFunctionExpr,
    NoDefaultReturnStmnt,
    MutatingExternalState,
    RedefiningFunctionArg,
    VoidArgument,
    OverwrittenFunction,
    RedefiningVariable,
    ReturnOutsideFunc,
    CtrlFlowOutsideWhile,
    NestedTryCatch,
    UnsafeCatch,
    ThrowInSafeFunc,
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

    pub fn invalid_bin_opr(opr_name: String, lhs: Type, rhs: Type, span: Span) -> Self {
        CompileError::new(CompileErrorKind::InvalidBinOpr(opr_name, lhs, rhs), span)
    }

    pub fn invalid_unary_opr(opr_name: String, val: Type, span: Span) -> Self {
        CompileError::new(CompileErrorKind::InvalidUnaryOpr(opr_name, val), span)
    }

    pub fn invalid_type(exp: Type, recv: Type, span: Span) -> Self {
        CompileError::new(CompileErrorKind::InvalidType(exp, recv), span)
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

    pub fn mutating_external_state(span: Span) -> Self {
        CompileError::new(CompileErrorKind::MutatingExternalState, span)
    }

    pub fn redefining_function_arg(span: Span) -> Self {
        CompileError::new(CompileErrorKind::RedefiningFunctionArg, span)
    }

    pub fn void_argument(span: Span) -> Self {
        CompileError::new(CompileErrorKind::VoidArgument, span)
    }

    pub fn overwritten_function(span: Span) -> Self {
        CompileError::new(CompileErrorKind::OverwrittenFunction, span)
    }

    pub fn redefining_variable(span: Span) -> Self {
        CompileError::new(CompileErrorKind::RedefiningVariable, span)
    }

    pub fn return_outside_func(span: Span) -> Self {
        CompileError::new(CompileErrorKind::ReturnOutsideFunc, span)
    }

    pub fn control_flow_outside_while(span: Span) -> Self {
        CompileError::new(CompileErrorKind::CtrlFlowOutsideWhile, span)
    }

    pub fn nested_try_catch(span: Span) -> Self {
        CompileError::new(CompileErrorKind::NestedTryCatch, span)
    }

    pub fn unsafe_catch(span: Span) -> Self {
        CompileError::new(CompileErrorKind::UnsafeCatch, span)
    }

    pub fn throw_in_safe_func(span: Span) -> Self {
        CompileError::new(CompileErrorKind::ThrowInSafeFunc, span)
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

            CompileErrorKind::InvalidBinOpr(opr_name, lhs, rhs) => {
                let msg = &format!(
                    "invalid binary operation `{}` between {:?} and {:?}",
                    opr_name, lhs, rhs
                );
                display_err(&self.span, f, msg)
            }

            CompileErrorKind::InvalidUnaryOpr(opr_name, val) => {
                let msg = &format!("invalid unary operation `{}` on {:?}", opr_name, val);
                display_err(&self.span, f, msg)
            }

            CompileErrorKind::InvalidType(exp, recv) => {
                let msg = &format!(
                    "invalid expression type (expected {:?}, received {:?})",
                    exp, recv
                );
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

            CompileErrorKind::MutatingExternalState => display_err(
                &self.span,
                f,
                "functions are not allowed to mutate any external state",
            ),

            CompileErrorKind::RedefiningFunctionArg => {
                display_err(&self.span, f, "redefining the function's argument")
            }

            CompileErrorKind::VoidArgument => {
                display_err(&self.span, f, "function arguments must be non-void")
            }

            CompileErrorKind::OverwrittenFunction => {
                display_err(&self.span, f, "overwriting already defined function")
            }

            CompileErrorKind::RedefiningVariable => {
                display_err(&self.span, f, "redefining variable")
            }

            CompileErrorKind::ReturnOutsideFunc => {
                display_err(&self.span, f, "return statement outside a function scope")
            }

            CompileErrorKind::CtrlFlowOutsideWhile => {
                display_err(&self.span, f, "control flow outside a while scope")
            }

            CompileErrorKind::NestedTryCatch => {
                display_err(&self.span, f, "redundant nested try-catch block")
            }

            CompileErrorKind::UnsafeCatch => display_err(
                &self.span,
                f,
                "unsafe oprations are not allowed in `catch` blocks",
            ),

            CompileErrorKind::ThrowInSafeFunc => {
                display_err(&self.span, f, "unguarded `throw` statements are only allowed in unsafe functions (ending with `!`)")
            }
        }
    }
}
