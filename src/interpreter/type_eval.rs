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
    ($stack:ident, $str_handler:ident, $list_handler:ident, $opr_name:expr, $span:ident) => {{
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
            (Type::List(left), right) => $list_handler(*left, right, $span),

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
    fn add_str(_: Type, _: &Span) -> Result<Type, ChalError> {
        Ok(Type::Str)
    }
    fn add_list(left: Type, right: Type, span: &Span) -> Result<Type, ChalError> {
        if left != right {
            return Err(CompileError::new(
                CompileErrorKind::InvalidType(left, right),
                span.clone(),
            )
            .into());
        }
        Ok(Type::List(Box::new(left)))
    }
    bin_opr_eval!(eval_stack, add_str, add_list, "+", span)
}

fn opr_sub(eval_stack: &mut Stack<Type>, span: &Span) -> Result<Type, ChalError> {
    fn sub_str(right: Type, span: &Span) -> Result<Type, ChalError> {
        Err(CompileError::new(
            CompileErrorKind::InvalidBinOpr("-".to_string(), Type::Str, right),
            span.clone(),
        )
        .into())
    }
    fn sub_list(left: Type, right: Type, span: &Span) -> Result<Type, ChalError> {
        Err(CompileError::new(
            CompileErrorKind::InvalidBinOpr("-".to_string(), Type::List(Box::new(left)), right),
            span.clone(),
        )
        .into())
    }
    bin_opr_eval!(eval_stack, sub_str, sub_list, "-", span)
}

fn opr_mul(eval_stack: &mut Stack<Type>, span: &Span) -> Result<Type, ChalError> {
    fn mul_str(right: Type, span: &Span) -> Result<Type, ChalError> {
        if right == Type::Uint {
            return Ok(Type::Str);
        }
        Err(CompileError::new(
            CompileErrorKind::InvalidBinOpr("*".to_string(), Type::Str, right),
            span.clone(),
        )
        .into())
    }
    fn mul_list(left: Type, right: Type, span: &Span) -> Result<Type, ChalError> {
        if right == Type::Uint {
            return Ok(Type::List(Box::new(left)));
        }
        Err(CompileError::new(
            CompileErrorKind::InvalidBinOpr("*".to_string(), Type::List(Box::new(left)), right),
            span.clone(),
        )
        .into())
    }
    bin_opr_eval!(eval_stack, mul_str, mul_list, "*", span)
}

fn opr_div(eval_stack: &mut Stack<Type>, span: &Span) -> Result<Type, ChalError> {
    fn div_str(right: Type, span: &Span) -> Result<Type, ChalError> {
        Err(CompileError::new(
            CompileErrorKind::InvalidBinOpr("/".to_string(), Type::Str, right),
            span.clone(),
        )
        .into())
    }
    fn div_list(left: Type, right: Type, span: &Span) -> Result<Type, ChalError> {
        Err(CompileError::new(
            CompileErrorKind::InvalidBinOpr("/".to_string(), Type::List(Box::new(left)), right),
            span.clone(),
        )
        .into())
    }
    bin_opr_eval!(eval_stack, div_str, div_list, "/", span)
}

fn opr_mod(eval_stack: &mut Stack<Type>, span: &Span) -> Result<Type, ChalError> {
    fn mod_str(right: Type, span: &Span) -> Result<Type, ChalError> {
        Err(CompileError::new(
            CompileErrorKind::InvalidBinOpr("mod".to_string(), Type::Str, right),
            span.clone(),
        )
        .into())
    }
    fn mod_list(left: Type, right: Type, span: &Span) -> Result<Type, ChalError> {
        Err(CompileError::new(
            CompileErrorKind::InvalidBinOpr("mod".to_string(), Type::List(Box::new(left)), right),
            span.clone(),
        )
        .into())
    }
    bin_opr_eval!(eval_stack, mod_str, mod_list, "%", span)
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
            NodeExprInner::Resolution(node) => {
                let res = node.as_type(interpreter)?;

                /* only functions can return void type */
                if res == Type::Void {
                    return Err(CompileError::new(
                        CompileErrorKind::VoidFunctionExpr,
                        node.span.clone(),
                    )
                    .into());
                }
                Ok(res)
            }

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

            NodeExprInner::List(node) => {
                /* an empty list is equal to `[Any]` */
                if node.elements.is_empty() {
                    return Ok(Type::List(Box::new(Type::Any)));
                }

                let mut list_ty = Type::Any;

                for el in node.elements.iter() {
                    let ty = el.as_type(interpreter)?;

                    if !Type::implicit_eq(&ty, &list_ty) {
                        return Err(CompileError::new(
                            CompileErrorKind::IncoherentList(list_ty, ty),
                            node.span.clone(),
                        )
                        .into());
                    }

                    if list_ty.root_type() == Type::Any {
                        list_ty = ty;
                    }
                }

                /* since each type should be the same, we can just return the last type */
                Ok(Type::List(Box::new(list_ty)))
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
            if !interpreter.namespaces.contains_key(ns) && !interpreter.builtins.contains_key(ns) {
                return Err(CompileError::new(
                    CompileErrorKind::UnknownNamespace(ns.clone()),
                    self.span.clone(),
                )
                .into());
            }
        }

        if let Some(ann) =
            interpreter.get_function_universal(&self.name, &arg_types, namespace.as_ref())
        {
            return Ok(ann.ret_type);
        }

        let mut func_name = self.name.clone() + "(";
        if let Some(ns) = namespace {
            func_name = ns + "::" + &func_name;
        }

        for ty in &arg_types {
            func_name += &format!("{}, ", ty);
        }

        if !arg_types.is_empty() {
            func_name.pop();
            func_name.pop();
        }
        func_name += ")";

        Err(CompileError::new(
            CompileErrorKind::UnknownFunction(func_name),
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
