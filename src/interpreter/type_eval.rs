use super::Chalcedony;
use crate::error::{span::Span, ChalError, CompileError};
use crate::parser::ast::{NodeExpr, NodeExprInner, NodeValue};

use crate::common::operators::{BinOprType, UnaryOprType};
use crate::common::Type;
use crate::utils::Stack;

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
            (left, right) => Err(CompileError::invalid_bin_opr(
                $opr_name.to_string(),
                left,
                right,
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
        Err(CompileError::invalid_bin_opr("-".to_string(), Type::Str, right, span.clone()).into())
    }
    bin_opr_eval!(eval_stack, sub, "-", span)
}

fn opr_mul(eval_stack: &mut Stack<Type>, span: &Span) -> Result<Type, ChalError> {
    fn mul(right: Type, span: &Span) -> Result<Type, ChalError> {
        if right == Type::Uint {
            return Ok(Type::Str);
        }
        Err(CompileError::invalid_bin_opr("*".to_string(), Type::Str, right, span.clone()).into())
    }
    bin_opr_eval!(eval_stack, mul, "*", span)
}

fn opr_div(eval_stack: &mut Stack<Type>, span: &Span) -> Result<Type, ChalError> {
    fn div(right: Type, span: &Span) -> Result<Type, ChalError> {
        Err(CompileError::invalid_bin_opr("/".to_string(), Type::Str, right, span.clone()).into())
    }
    bin_opr_eval!(eval_stack, div, "/", span)
}

fn opr_mod(eval_stack: &mut Stack<Type>, span: &Span) -> Result<Type, ChalError> {
    fn _mod(right: Type, span: &Span) -> Result<Type, ChalError> {
        Err(CompileError::invalid_bin_opr("mod".to_string(), Type::Str, right, span.clone()).into())
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
        (left, right) => {
            Err(CompileError::invalid_bin_opr(opr.to_string(), left, right, span.clone()).into())
        }
    }
}

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
            (left, right) => Err(CompileError::invalid_bin_opr(
                $opr_name.to_string(),
                left,
                right,
                $span.clone(),
            )
            .into()),
        }
    }};
}

/* matches != and == */
fn opr_eq(eval_stack: &mut Stack<Type>, opr: &str, span: &Span) -> Result<Type, ChalError> {
    let cmp_eq = |val: Type, span: &Span| -> Result<Type, ChalError> {
        match val {
            Type::Int | Type::Uint | Type::Float | Type::Bool => Ok(Type::Bool),
            ty => Err(
                CompileError::invalid_bin_opr(opr.to_string(), Type::Bool, ty, span.clone()).into(),
            ),
        }
    };
    opr_cmp_internal!(eval_stack, cmp_eq, opr, span)
}

/* matches lt, gt, lteq, gteq */
fn opr_cmp(eval_stack: &mut Stack<Type>, opr: &str, span: &Span) -> Result<Type, ChalError> {
    let cmp_operator = |right: Type, span: &Span| -> Result<Type, ChalError> {
        Err(CompileError::invalid_bin_opr(opr.to_string(), Type::Bool, right, span.clone()).into())
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
        ty => Err(CompileError::invalid_unary_opr("-".to_string(), ty, span.clone()).into()),
    }
}

fn opr_not(eval_stack: &mut Stack<Type>, span: &Span) -> Result<Type, ChalError> {
    let val = eval_stack.pop().expect("expected a value on the stack");
    match val {
        Type::Int | Type::Uint | Type::Float | Type::Bool => Ok(Type::Bool),
        ty => Err(CompileError::invalid_unary_opr("!".to_string(), ty, span.clone()).into()),
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
            NodeExprInner::VarCall(node) => {
                if let Some(func) = interpreter.current_func.clone() {
                    if let Some(var) = func.arg_lookup.get(&node.name) {
                        return Ok(var.ty.clone());
                    }
                }

                if let Some(var) = interpreter.locals.borrow().get(&node.name) {
                    return Ok(var.ty.clone());
                }

                if let Some(var) = interpreter.globals.get(&node.name) {
                    return Ok(var.ty.clone());
                }

                Err(CompileError::unknown_variable(node.name.clone(), node.span.clone()).into())
            }

            NodeExprInner::FuncCall(node) => {
                let arg_types: Result<Vec<Type>, ChalError> = node
                    .args
                    .iter()
                    .map(|arg| arg.as_type(interpreter))
                    .collect();
                let arg_types = match arg_types {
                    Ok(ok) => ok,
                    Err(err) => return Err(err),
                };
                if let Some(func) = interpreter.get_function(&node.name, &arg_types) {
                    if func.ret_type == Type::Void {
                        return Err(CompileError::void_func_expr(node.span.clone()).into());
                    }
                    return Ok(func.ret_type.clone());
                }
                Err(CompileError::unknown_function(node.name.clone(), node.span.clone()).into())
            }

            NodeExprInner::BinOpr(opr) => opr.as_type(eval_stack, span),
            NodeExprInner::UnaryOpr(opr) => opr.as_type(eval_stack, span),

            NodeExprInner::List(node) => {
                /* an empty list is equal to `[Any]` */
                if node.elements.is_empty() {
                    return Ok(Type::List(Box::new(Type::Any)));
                }

                let mut prev_type = Type::Any;

                for el in node.elements.iter() {
                    let ty = el.as_type(interpreter)?;

                    if !Type::implicit_eq(&ty, &prev_type) {
                        return Err(CompileError::incoherent_list(
                            node.span.clone(),
                            ty,
                            prev_type,
                        )
                        .into());
                    }
                    prev_type = ty;
                }

                /* since each type should be the same, we can just return the last type */
                Ok(Type::List(Box::new(prev_type)))
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
