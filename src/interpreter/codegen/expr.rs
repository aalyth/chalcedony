use super::ToBytecode;

use crate::error::ChalError;
use crate::interpreter::FuncAnnotation;
use crate::parser::ast::operators::{BinOprType, UnaryOprType};
use crate::parser::ast::{NodeExpr, NodeExprInner, NodeValue, NodeVarCall};
use crate::utils::Bytecode;

use std::collections::HashMap;

impl ToBytecode for NodeExpr {
    fn to_bytecode(
        self,
        func_symtable: &mut HashMap<String, FuncAnnotation>,
    ) -> Result<Vec<u8>, ChalError> {
        let NodeExpr(expr) = self;
        let mut result = Vec::<u8>::new();
        for e in expr {
            result.append(&mut e.to_bytecode(func_symtable)?)
        }
        Ok(result)
    }
}

macro_rules! val_to_bytecode {
    ($capacity:expr, $value:expr, $bytecode:ident) => {{
        let mut result = Vec::<u8>::with_capacity($capacity);
        result.push(Bytecode::$bytecode as u8);
        result.extend_from_slice($value);
        return Ok(result);
    }};
}

impl ToBytecode for NodeExprInner {
    fn to_bytecode(
        self,
        func_symtable: &mut HashMap<String, FuncAnnotation>,
    ) -> Result<Vec<u8>, ChalError> {
        match self {
            NodeExprInner::BinOpr(opr_type) => match opr_type {
                BinOprType::Add => Ok(vec![Bytecode::OpAdd as u8]),
                BinOprType::Sub => Ok(vec![Bytecode::OpSub as u8]),
                BinOprType::Mul => Ok(vec![Bytecode::OpMul as u8]),
                BinOprType::Div => Ok(vec![Bytecode::OpDiv as u8]),
                BinOprType::Mod => Ok(vec![Bytecode::OpMod as u8]),

                BinOprType::And => Ok(vec![Bytecode::OpAnd as u8]),
                BinOprType::Or => Ok(vec![Bytecode::OpOr as u8]),

                BinOprType::Lt => Ok(vec![Bytecode::OpLt as u8]),
                BinOprType::Gt => Ok(vec![Bytecode::OpGt as u8]),

                BinOprType::LtEq => Ok(vec![Bytecode::OpLtEq as u8]),
                BinOprType::GtEq => Ok(vec![Bytecode::OpGtEq as u8]),

                BinOprType::EqEq => Ok(vec![Bytecode::OpEq as u8]),
                BinOprType::BangEq => Ok(vec![Bytecode::OpEq as u8, Bytecode::OpNot as u8]),
            },
            NodeExprInner::UnaryOpr(opr_type) => match opr_type {
                UnaryOprType::Neg => Ok(vec![Bytecode::OpNeg as u8]),
                UnaryOprType::Bang => Ok(vec![Bytecode::OpNot as u8]),
            },
            NodeExprInner::Value(val_node) => match val_node {
                NodeValue::Int(val) => val_to_bytecode!(5, &val.to_ne_bytes(), OpConstI),
                NodeValue::Uint(val) => val_to_bytecode!(5, &val.to_ne_bytes(), OpConstU),
                NodeValue::Float(val) => val_to_bytecode!(5, &val.to_ne_bytes(), OpConstF),
                NodeValue::Str(val) => {
                    let mut str_vec = val[1..val.len() - 1].as_bytes().to_vec();
                    str_vec.push(0);
                    val_to_bytecode!(val.len() + 1, str_vec.as_slice(), OpConstS)
                }
            },
            NodeExprInner::VarCall(NodeVarCall(varname)) => {
                let mut result = Vec::<u8>::with_capacity(varname.len() + 1);
                result.push(Bytecode::OpGetVar as u8);
                result.extend_from_slice(varname.as_bytes());
                result.push(0);
                Ok(result)
            }
            NodeExprInner::FuncCall(node) => {
                let mut res = Vec::<u8>::new();
                res.append(&mut node.to_bytecode(func_symtable)?);
                Ok(res)
            }
        }
    }
}
