use super::Chalcedony;
use crate::error::{ChalError, CompileError};
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

// works with the resulting type of the 3 basic operations: add, sub, mul
macro_rules! basic_bin_opr {
    ($stack:ident, $str_handler:ident) => {{
        let (left, right) = get_eval_args($stack);
        match (left, right) {
            (Type::Int, Type::Int) => Ok(Type::Int),
            (Type::Int, Type::Uint) => Ok(Type::Int),
            (Type::Int, Type::Float) => Ok(Type::Float),

            (Type::Uint, Type::Int) => Ok(Type::Int),
            (Type::Uint, Type::Uint) => Ok(Type::Uint),
            (Type::Uint, Type::Float) => Ok(Type::Float),

            (Type::Float, Type::Int) => Ok(Type::Float),
            (Type::Float, Type::Uint) => Ok(Type::Float),
            (Type::Float, Type::Float) => Ok(Type::Float),

            (Type::Str, right @ _) => $str_handler(right),
            _ => panic!("TODO: implement spanning for the operation and return a proper error"),
        }
    }};
}

fn opr_add(eval_stack: &mut Stack<Type>) -> Result<Type, ChalError> {
    // anything added to a string returns a string
    fn add(_: Type) -> Result<Type, ChalError> {
        Ok(Type::Str)
    }
    basic_bin_opr!(eval_stack, add)
}

fn opr_sub(eval_stack: &mut Stack<Type>) -> Result<Type, ChalError> {
    // anything added to a string returns a string
    fn sub(_: Type) -> Result<Type, ChalError> {
        panic!("TODO: throw proper invalid operation error")
    }
    basic_bin_opr!(eval_stack, sub)
}

fn opr_mul(eval_stack: &mut Stack<Type>) -> Result<Type, ChalError> {
    // anything added to a string returns a string
    fn mul(right: Type) -> Result<Type, ChalError> {
        if right == Type::Uint {
            return Ok(Type::Str);
        }
        panic!("TODO: throw proper invalid operation error")
    }
    basic_bin_opr!(eval_stack, mul)
}

fn opr_div(eval_stack: &mut Stack<Type>) -> Result<Type, ChalError> {
    let (left, right) = get_eval_args(eval_stack);
    match (left, right) {
        (Type::Int, Type::Int)
        | (Type::Int, Type::Uint)
        | (Type::Int, Type::Float)
        | (Type::Uint, Type::Int)
        | (Type::Uint, Type::Uint)
        | (Type::Uint, Type::Float)
        | (Type::Float, Type::Int)
        | (Type::Float, Type::Uint)
        | (Type::Float, Type::Float) => Ok(Type::Float),

        _ => panic!("TODO: implement spanning for the operation and return a proper error"),
    }
}

fn opr_mod(eval_stack: &mut Stack<Type>) -> Result<Type, ChalError> {
    fn _mod(_: Type) -> Result<Type, ChalError> {
        panic!("TODO: throw proper invalid operation error")
    }
    basic_bin_opr!(eval_stack, _mod)
}

// logical || or &&
fn opr_logical(eval_stack: &mut Stack<Type>) -> Result<Type, ChalError> {
    let right = eval_stack.pop().expect("expected a type on the eval stack");
    let left = eval_stack.pop().expect("expected a type on the eval stack");

    // TODO: simplify this ???
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
        _ => panic!("TODO: throw poeper invalid operation error"),
    }
}

macro_rules! opr_cmp_internal {
    ($stack:ident, $cmp_func:ident) => {{
        let right = $stack.pop().expect("expected a type on the eval stack");
        let left = $stack.pop().expect("expected a type on the eval stack");

        match (left, right) {
            (Type::Int, Type::Int) => Ok(Type::Bool),
            (Type::Int, Type::Uint) => Ok(Type::Bool),
            (Type::Int, Type::Float) => Ok(Type::Bool),
            (Type::Int, Type::Bool) => $cmp_func(left),

            (Type::Uint, Type::Int) => Ok(Type::Bool),
            (Type::Uint, Type::Uint) => Ok(Type::Bool),
            (Type::Uint, Type::Float) => Ok(Type::Bool),
            (Type::Uint, Type::Bool) => $cmp_func(left),

            (Type::Float, Type::Int) => Ok(Type::Bool),
            (Type::Float, Type::Uint) => Ok(Type::Bool),
            (Type::Float, Type::Float) => Ok(Type::Bool),
            (Type::Float, Type::Bool) => $cmp_func(left),

            (Type::Str, Type::Str) => Ok(Type::Bool),
            (Type::Bool, _) => $cmp_func(right),
            _ => panic!("TODO: make this return a proper error"),
        }
    }};
}

fn opr_eq(eval_stack: &mut Stack<Type>) -> Result<Type, ChalError> {
    fn cmp_eq(val: Type) -> Result<Type, ChalError> {
        match val {
            Type::Int | Type::Uint | Type::Float | Type::Bool => Ok(Type::Bool),
            _ => panic!("TODO: make this return a proper error"),
        }
    }
    opr_cmp_internal!(eval_stack, cmp_eq)
}

// matches lt, gt, le, ge
fn opr_cmp(eval_stack: &mut Stack<Type>) -> Result<Type, ChalError> {
    fn cmp_operator(_: Type) -> Result<Type, ChalError> {
        panic!("TODO: make this return a proper error")
    }
    opr_cmp_internal!(eval_stack, cmp_operator)
}

impl BinOprType {
    fn as_type(&self, eval_stack: &mut Stack<Type>) -> Result<Type, ChalError> {
        match self {
            BinOprType::Add => opr_add(eval_stack),
            BinOprType::Sub => opr_sub(eval_stack),
            BinOprType::Mul => opr_mul(eval_stack),
            BinOprType::Div => opr_div(eval_stack),
            BinOprType::Mod => opr_mod(eval_stack),

            BinOprType::And | BinOprType::Or => opr_logical(eval_stack),

            BinOprType::EqEq | BinOprType::BangEq => opr_eq(eval_stack),
            BinOprType::Lt | BinOprType::Gt | BinOprType::LtEq | BinOprType::GtEq => {
                opr_cmp(eval_stack)
            }
        }
    }
}

fn opr_neg(eval_stack: &mut Stack<Type>) -> Result<Type, ChalError> {
    let val = eval_stack.pop().expect("expected a value on the stack");
    match val {
        Type::Int => Ok(Type::Int),
        Type::Uint => Ok(Type::Int),
        Type::Float => Ok(Type::Float),
        _ => panic!("TODO: make this return a proper error"),
    }
}

fn opr_not(eval_stack: &mut Stack<Type>) -> Result<Type, ChalError> {
    let val = eval_stack.pop().expect("expected a value on the stack");
    match val {
        Type::Int | Type::Uint | Type::Float | Type::Bool => Ok(Type::Bool),
        _ => panic!("TODO: make this return a proper error"),
    }
}

impl UnaryOprType {
    fn as_type(&self, eval_stack: &mut Stack<Type>) -> Result<Type, ChalError> {
        match self {
            UnaryOprType::Neg => opr_neg(eval_stack),
            UnaryOprType::Bang => opr_not(eval_stack),
        }
    }
}

impl NodeExprInner {
    fn as_type(
        &self,
        eval_stack: &mut Stack<Type>,
        interpreter: &Chalcedony,
    ) -> Result<Type, ChalError> {
        match self {
            NodeExprInner::Value(node) => Ok(node.as_type()),
            NodeExprInner::VarCall(node) => {
                if let Some(func) = interpreter.current_func.clone() {
                    if let Some(var) = func.borrow().args.get(&node.name) {
                        return Ok(var.ty);
                    }

                    if let Some(var) = func.borrow().locals.get(&node.name) {
                        return Ok(var.ty);
                    }
                }

                if let Some(var) = interpreter.globals.get(&node.name) {
                    return Ok(var.ty);
                }

                Err(CompileError::unknown_variable(node.name.clone(), node.span.clone()).into())
            }

            NodeExprInner::FuncCall(node) => {
                if let Some(func) = interpreter.func_symtable.get(&node.name).clone() {
                    let func = func.borrow();
                    if func.ret_type == Type::Void {
                        return Err(CompileError::void_func_expr(node.span.clone()).into());
                    }
                    return Ok(func.ret_type);
                }
                Err(CompileError::unknown_function(node.name.clone(), node.span.clone()).into())
            }

            NodeExprInner::BinOpr(opr) => opr.as_type(eval_stack),
            NodeExprInner::UnaryOpr(opr) => opr.as_type(eval_stack),
        }
    }
}

impl NodeExpr {
    pub fn as_type(&self, interpreter: &Chalcedony) -> Result<Type, ChalError> {
        if self.expr.len() == 0 {
            return Ok(Type::Void);
        }

        let mut eval_stack = Stack::<Type>::new();
        for el in &self.expr {
            let val = el.as_type(&mut eval_stack, interpreter)?;
            eval_stack.push(val);
        }
        if eval_stack.len() != 1 {
            panic!("expected only 1 element from the expression")
        }
        Ok(eval_stack.pop().expect("expected a value on the stack"))
    }
}
