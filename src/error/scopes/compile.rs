use super::display_err;
use crate::error::span::Span;

use crate::common::Type;

/// The types of errors that could be encountered during the process of
/// compiling the Abstract Syntax Tree into bytecode. For each error's meaning
/// refer to implementation of `std::fmt::Display` for `CompileError`.
pub enum CompileErrorKind {
    /// `<var-name>`
    UnknownVariable(String),
    /// `<func-name>`
    UnknownFunction(String),
    /// `<opr-literal>`, `<lhs-type>`, `<rhs-type>`
    InvalidBinOpr(String, Type, Type),
    /// `<opr-literal>`, `<expr-type>`
    InvalidUnaryOpr(String, Type),
    /// `<exp-type>`, `<recv-type>`
    InvalidType(Type, Type),
    /// `<exp-type>`, `<recv-type>`
    TooManyArguments(usize, usize),
    /// `<exp-type>`, `<recv-type>`
    TooFewArguments(usize, usize),
    /// `<func-return-type>`
    NonVoidFunctionStmnt(Type),
    /// `<filename>`
    ScriptNotFound(String),
    /// `<class-name>`
    ClassAlreadyExists(String),
    /// `<class-name>`
    UnknownClass(String),
    /// `<member-names>`
    MissingMembers(Vec<String>),
    /// `<member-names>`
    UndefinedMembers(Vec<String>),
    /// `<member>`
    UnknownMember(String),
    /// `<namespace-name>`
    UnknownNamespace(String),
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
    MutatingConstant,
    MemberAlreadyExists,
}

pub struct CompileError {
    kind: CompileErrorKind,
    span: Span,
}

impl CompileError {
    pub fn new(kind: CompileErrorKind, span: Span) -> Self {
        CompileError { span, kind }
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

            CompileErrorKind::ScriptNotFound(name) => {
                let msg = &format!("could not find the script `{}`", name);
                display_err(&self.span, f, msg)
            }

            CompileErrorKind::ClassAlreadyExists(name) => {
                let msg = &format!("class already exists `{}`", name);
                display_err(&self.span, f, msg)
            }

            CompileErrorKind::UnknownClass(name) => {
                let msg = &format!("unknown class `{}`", name);
                display_err(&self.span, f, msg)
            }

            CompileErrorKind::MissingMembers(members) => {
                let mut msg = "missing class members: \n".to_string();
                for member in members {
                    msg.push_str(&format!("  - {}\n", member))
                }
                /* remove the trailing newline */
                msg.pop();
                display_err(&self.span, f, &msg)
            }

            CompileErrorKind::UndefinedMembers(members) => {
                let mut msg = "undefined class members: \n".to_string();
                for member in members {
                    msg.push_str(&format!("  - {}\n", member))
                }
                /* remove the trailing newline */
                msg.pop();
                display_err(&self.span, f, &msg)
            }

            CompileErrorKind::UnknownMember(name) => {
                let msg = &format!("unknown member `{:?}`", name);
                display_err(&self.span, f, msg)
            }

            CompileErrorKind::UnknownNamespace(name) => {
                let msg = &format!("unknown namespace `{}`", name);
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

            CompileErrorKind::MutatingConstant => {
                display_err(&self.span, f, "mutating a constant variable")
            }

            CompileErrorKind::MemberAlreadyExists => {
                display_err(&self.span, f, "member already exists")
            }
        }
    }
}
