use super::Chalcedony;
use crate::error::{span::Span, ChalError, CompileError, CompileErrorKind};
use crate::parser::ast::{
    NodeAttrRes, NodeAttribute, NodeExpr, NodeExprInner, NodeFuncCall, NodeValue, NodeVarCall,
};

use crate::common::operators::{BinOprType, UnaryOprType};
use crate::common::Type;
use crate::utils::Stack;

use std::collections::VecDeque;

impl NodeValue {
    fn as_type(&self) -> Type {
        match self {
            NodeValue::Int(_) => Type::Int,
            NodeValue::Uint(_) => Type::Uint,
            NodeValue::Float(_) => Type::Float,
            NodeValue::Str(_) => Type::Str,
            NodeValue::Bool(_) => Type::Bool,
        }
    }
}

fn get_eval_args(eval_stack: &mut Stack<Type>) -> (Type, Type) {
    let right = eval_stack.pop().expect("expected a type on the eval stack");
    let left = eval_stack.pop().expect("expected a type on the eval stack");
    (left, right)
}

/// The types in Chalcedony have the property of trying to perserve their own
/// type between binary operations. The only downside of this property is the
/// potentially unexpected rounding to 0 in operations such as:
/// ```
/// # the value of `a` becomes 0, not 0.5
/// let a = 1 / 2
/// ```
macro_rules! bin_opr_eval {
    ($stack:ident, $str_handler:ident, $opr_name:expr, $span:ident) => {{
        let (left, right) = get_eval_args($stack);
        match (left, right) {
            (Type::Int, Type::Int) => Ok(Type::Int),
            (Type::Int, Type::Uint) => Ok(Type::Int),
            (Type::Int, Type::Float) => Ok(Type::Int),

            (Type::Uint, Type::Int) => Ok(Type::Int),
            (Type::Uint, Type::Uint) => Ok(Type::Uint),
            (Type::Uint, Type::Float) => Ok(Type::Int),

            (Type::Float, Type::Int) => Ok(Type::Float),
            (Type::Float, Type::Uint) => Ok(Type::Float),
            (Type::Float, Type::Float) => Ok(Type::Float),

            (Type::Str, right) => $str_handler(right, $span),
            (left, right) => Err(CompileError::new(
                CompileErrorKind::InvalidBinOpr($opr_name.to_string(), left, right),
                $span.clone(),
            )
            .into()),
        }
    }};
}

fn opr_add(eval_stack: &mut Stack<Type>, span: &Span) -> Result<Type, ChalError> {
    /* anything added to a string returns a string */
    fn add(_: Type, _: &Span) -> Result<Type, ChalError> {
        Ok(Type::Str)
    }
    bin_opr_eval!(eval_stack, add, "+", span)
}

fn opr_sub(eval_stack: &mut Stack<Type>, span: &Span) -> Result<Type, ChalError> {
    fn sub(right: Type, span: &Span) -> Result<Type, ChalError> {
        Err(CompileError::new(
            CompileErrorKind::InvalidBinOpr("-".to_string(), Type::Str, right),
            span.clone(),
        )
        .into())
    }
    bin_opr_eval!(eval_stack, sub, "-", span)
}

fn opr_mul(eval_stack: &mut Stack<Type>, span: &Span) -> Result<Type, ChalError> {
    fn mul(right: Type, span: &Span) -> Result<Type, ChalError> {
        if right == Type::Uint {
            return Ok(Type::Str);
        }
        Err(CompileError::new(
            CompileErrorKind::InvalidBinOpr("*".to_string(), Type::Str, right),
            span.clone(),
        )
        .into())
    }
    bin_opr_eval!(eval_stack, mul, "*", span)
}

fn opr_div(eval_stack: &mut Stack<Type>, span: &Span) -> Result<Type, ChalError> {
    fn div(right: Type, span: &Span) -> Result<Type, ChalError> {
        Err(CompileError::new(
            CompileErrorKind::InvalidBinOpr("/".to_string(), Type::Str, right),
            span.clone(),
        )
        .into())
    }
    bin_opr_eval!(eval_stack, div, "/", span)
}

fn opr_mod(eval_stack: &mut Stack<Type>, span: &Span) -> Result<Type, ChalError> {
    fn _mod(right: Type, span: &Span) -> Result<Type, ChalError> {
        Err(CompileError::new(
            CompileErrorKind::InvalidBinOpr("mod".to_string(), Type::Str, right),
            span.clone(),
        )
        .into())
    }
    bin_opr_eval!(eval_stack, _mod, "%", span)
}

/* logical || or && */
fn opr_logical(eval_stack: &mut Stack<Type>, opr: &str, span: &Span) -> Result<Type, ChalError> {
    let right = eval_stack.pop().expect("expected a type on the eval stack");
    let left = eval_stack.pop().expect("expected a type on the eval stack");

    match (left, right) {
        (Type::Int, Type::Int)
        | (Type::Int, Type::Uint)
        | (Type::Int, Type::Float)
        | (Type::Int, Type::Bool)
        | (Type::Uint, Type::Int)
        | (Type::Uint, Type::Uint)
        | (Type::Uint, Type::Float)
        | (Type::Uint, Type::Bool)
        | (Type::Float, Type::Int)
        | (Type::Float, Type::Uint)
        | (Type::Float, Type::Float)
        | (Type::Float, Type::Bool)
        | (Type::Bool, Type::Int)
        | (Type::Bool, Type::Uint)
        | (Type::Bool, Type::Float)
        | (Type::Bool, Type::Bool) => Ok(Type::Bool),
        (left, right) => Err(CompileError::new(
            CompileErrorKind::InvalidBinOpr(opr.to_string(), left, right),
            span.clone(),
        )
        .into()),
    }
}

// Every comparison operator yields a boolean value.
macro_rules! opr_cmp_internal {
    ($stack:ident, $cmp_func:ident, $opr_name:expr, $span:ident) => {{
        let right = $stack.pop().expect("expected a type on the eval stack");
        let left = $stack.pop().expect("expected a type on the eval stack");

        match (left, right) {
            (Type::Int, Type::Int) => Ok(Type::Bool),
            (Type::Int, Type::Uint) => Ok(Type::Bool),
            (Type::Int, Type::Float) => Ok(Type::Bool),
            (left @ Type::Int, Type::Bool) => $cmp_func(left, $span),

            (Type::Uint, Type::Int) => Ok(Type::Bool),
            (Type::Uint, Type::Uint) => Ok(Type::Bool),
            (Type::Uint, Type::Float) => Ok(Type::Bool),
            (left @ Type::Uint, Type::Bool) => $cmp_func(left, $span),

            (Type::Float, Type::Int) => Ok(Type::Bool),
            (Type::Float, Type::Uint) => Ok(Type::Bool),
            (Type::Float, Type::Float) => Ok(Type::Bool),
            (left @ Type::Float, Type::Bool) => $cmp_func(left, $span),

            (Type::Str, Type::Str) => Ok(Type::Bool),
            (Type::Bool, right) => $cmp_func(right, $span),
            (left, right) => Err(CompileError::new(
                CompileErrorKind::InvalidBinOpr($opr_name.to_string(), left, right),
                $span.clone(),
            )
            .into()),
        }
    }};
}

// Matches the operators `!=` and `==`.
fn opr_eq(eval_stack: &mut Stack<Type>, opr: &str, span: &Span) -> Result<Type, ChalError> {
    let cmp_eq = |val: Type, span: &Span| -> Result<Type, ChalError> {
        match val {
            Type::Int | Type::Uint | Type::Float | Type::Bool => Ok(Type::Bool),
            ty => Err(CompileError::new(
                CompileErrorKind::InvalidBinOpr(opr.to_string(), Type::Bool, ty),
                span.clone(),
            )
            .into()),
        }
    };
    opr_cmp_internal!(eval_stack, cmp_eq, opr, span)
}

// Matches the operators `<`, `>`, `<=`, `>=`.
fn opr_cmp(eval_stack: &mut Stack<Type>, opr: &str, span: &Span) -> Result<Type, ChalError> {
    let cmp_operator = |right: Type, span: &Span| -> Result<Type, ChalError> {
        Err(CompileError::new(
            CompileErrorKind::InvalidBinOpr(opr.to_string(), Type::Bool, right),
            span.clone(),
        )
        .into())
    };
    opr_cmp_internal!(eval_stack, cmp_operator, opr, span)
}

impl BinOprType {
    fn as_type(&self, eval_stack: &mut Stack<Type>, span: &Span) -> Result<Type, ChalError> {
        match self {
            BinOprType::Add => opr_add(eval_stack, span),
            BinOprType::Sub => opr_sub(eval_stack, span),
            BinOprType::Mul => opr_mul(eval_stack, span),
            BinOprType::Div => opr_div(eval_stack, span),
            BinOprType::Mod => opr_mod(eval_stack, span),

            BinOprType::And => opr_logical(eval_stack, "&&", span),
            BinOprType::Or => opr_logical(eval_stack, "||", span),

            BinOprType::EqEq => opr_eq(eval_stack, "==", span),
            BinOprType::BangEq => opr_eq(eval_stack, "!=", span),

            BinOprType::Lt => opr_cmp(eval_stack, "<", span),
            BinOprType::Gt => opr_cmp(eval_stack, ">", span),
            BinOprType::LtEq => opr_cmp(eval_stack, "<=", span),
            BinOprType::GtEq => opr_cmp(eval_stack, ">=", span),
        }
    }
}

fn opr_neg(eval_stack: &mut Stack<Type>, span: &Span) -> Result<Type, ChalError> {
    let val = eval_stack.pop().expect("expected a value on the stack");
    match val {
        Type::Int => Ok(Type::Int),
        Type::Uint => Ok(Type::Int),
        Type::Float => Ok(Type::Float),
        ty => Err(CompileError::new(
            CompileErrorKind::InvalidUnaryOpr("-".to_string(), ty),
            span.clone(),
        )
        .into()),
    }
}

fn opr_not(eval_stack: &mut Stack<Type>, span: &Span) -> Result<Type, ChalError> {
    let val = eval_stack.pop().expect("expected a value on the stack");
    match val {
        Type::Int | Type::Uint | Type::Float | Type::Bool => Ok(Type::Bool),
        ty => Err(CompileError::new(
            CompileErrorKind::InvalidUnaryOpr("!".to_string(), ty),
            span.clone(),
        )
        .into()),
    }
}

impl UnaryOprType {
    fn as_type(&self, eval_stack: &mut Stack<Type>, span: &Span) -> Result<Type, ChalError> {
        match self {
            UnaryOprType::Neg => opr_neg(eval_stack, span),
            UnaryOprType::Bang => opr_not(eval_stack, span),
        }
    }
}

impl NodeExprInner {
    fn as_type(
        &self,
        span: &Span,
        eval_stack: &mut Stack<Type>,
        interpreter: &Chalcedony,
    ) -> Result<Type, ChalError> {
        match self {
            NodeExprInner::Value(node) => Ok(node.as_type()),
            NodeExprInner::Resolution(node) => node.as_type(interpreter),

            NodeExprInner::BinOpr(opr) => opr.as_type(eval_stack, span),
            NodeExprInner::UnaryOpr(opr) => opr.as_type(eval_stack, span),
            NodeExprInner::InlineClass(class) => {
                if !interpreter.namespaces.contains_key(&class.class) {
                    return Err(CompileError::new(
                        CompileErrorKind::UnknownClass(class.class.clone()),
                        class.span.clone(),
                    )
                    .into());
                }

                Ok(Type::Custom(Box::new(class.class.clone())))
            }
        }
    }
}

impl NodeExpr {
    pub fn as_type(&self, interpreter: &Chalcedony) -> Result<Type, ChalError> {
        if self.expr.is_empty() {
            return Ok(Type::Void);
        }

        let mut eval_stack = Stack::<Type>::new();
        for el in &self.expr {
            let val = el.as_type(&self.span, &mut eval_stack, interpreter)?;
            eval_stack.push(val);
        }
        if eval_stack.len() != 1 {
            panic!("expected only 1 element from the expression")
        }
        Ok(eval_stack.pop().expect("expected a value on the stack"))
    }
}

impl NodeVarCall {
    pub fn as_type(
        &self,
        interpreter: &Chalcedony,
        parent_type: Option<Type>,
    ) -> Result<Type, ChalError> {
        if let Some(ty) = parent_type {
            let class_name = ty.as_class();
            let Some(class) = interpreter.namespaces.get(&class_name) else {
                return Err(CompileError::new(
                    CompileErrorKind::UnknownNamespace(class_name),
                    self.span.clone(),
                )
                .into());
            };

            let Some(annotation) = class.members.get(&self.name) else {
                return Err(CompileError::new(
                    CompileErrorKind::UnknownMember(self.name.clone()),
                    self.span.clone(),
                )
                .into());
            };

            return Ok(annotation.ty.clone());
        }

        if let Some(func) = &interpreter.current_func {
            if let Some(annotation) = func.arg_lookup.get(&self.name) {
                return Ok(annotation.ty.clone());
            }
        }

        if let Some(annotation) = interpreter.globals.get(&self.name) {
            return Ok(annotation.ty.clone());
        }

        if let Some(annotation) = interpreter.locals.get(&self.name) {
            return Ok(annotation.ty.clone());
        }

        Err(CompileError::new(
            CompileErrorKind::UnknownVariable(self.name.clone()),
            self.span.clone(),
        )
        .into())
    }
}

impl NodeFuncCall {
    pub fn as_type(
        &self,
        interpreter: &Chalcedony,
        parent_type: Option<Type>,
    ) -> Result<Type, ChalError> {
        let arg_types: Result<VecDeque<Type>, ChalError> = self
            .args
            .iter()
            .map(|arg| arg.as_type(interpreter))
            .collect();
        let mut arg_types = match arg_types {
            Ok(ok) => ok,
            Err(err) => return Err(err),
        };

        let mut namespace = self.namespace.clone();

        /* the function is called as a method */
        if let Some(ty) = &parent_type {
            arg_types.push_front(ty.clone());
            if self.namespace.is_some() {
                panic!("calling a namespace function also as a method");
            }
            namespace = Some(ty.as_class());
        }

        if let Some(ns) = &namespace {
            if !interpreter.namespaces.contains_key(ns) {
                return Err(CompileError::new(
                    CompileErrorKind::UnknownNamespace(ns.clone()),
                    self.span.clone(),
                )
                .into());
            }
        }

        if let Some(func) = interpreter.get_function(&self.name, &arg_types, namespace.as_ref()) {
            return Ok(func.ret_type.clone());
        }
        Err(CompileError::new(
            CompileErrorKind::UnknownFunction(self.name.clone()),
            self.span.clone(),
        )
        .into())
    }
}

impl NodeAttrRes {
    pub fn as_type(&self, interpreter: &Chalcedony) -> Result<Type, ChalError> {
        let mut parent_type: Option<Type> = None;

        for node in &self.resolution {
            match node {
                NodeAttribute::VarCall(node) => {
                    parent_type = Some(node.as_type(interpreter, parent_type.clone())?);
                }
                NodeAttribute::FuncCall(node) => {
                    parent_type = Some(node.as_type(interpreter, parent_type.clone())?);
                }
            }
        }

        Ok(parent_type.unwrap())
    }
}
