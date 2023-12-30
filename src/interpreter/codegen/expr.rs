use super::var::get_var_id;
use super::ToBytecode;

use crate::error::{ChalError, Position, RuntimeError};
use crate::interpreter::FuncAnnotation;
use crate::lexer::Type;
use crate::parser::ast::operators::{BinOprType, UnaryOprType};
use crate::parser::ast::{NodeExpr, NodeExprInner, NodeValue, NodeVarCall};
use crate::utils::Bytecode;

use std::collections::BTreeMap;

const U32_BYTES: usize = 4;
const POS_BYTES: usize = 1 + U32_BYTES;

macro_rules! push_pos_bytecode {
    ($result:ident, $code:ident, $val:expr) => {{
        $result.push(Bytecode::$code as u8);
        $result.extend_from_slice(&($val as u64).to_ne_bytes());
    }};
}

fn set_positions(start: Position, end: Position) -> Vec<u8> {
    let mut result = Vec::<u8>::with_capacity(4 * POS_BYTES);
    push_pos_bytecode!(result, OpStartLn, start.ln);
    push_pos_bytecode!(result, OpStartCol, start.col);
    push_pos_bytecode!(result, OpEndLn, end.ln);
    push_pos_bytecode!(result, OpEndCol, end.col);
    return result;
}

impl ToBytecode for NodeExpr {
    fn to_bytecode(
        self,
        bytecode_len: usize,
        var_symtable: &mut BTreeMap<String, u64>,
        func_symtable: &mut BTreeMap<String, FuncAnnotation>,
    ) -> Result<Vec<u8>, ChalError> {
        let (expr, start, end, _span) = self.disassemble();
        let mut result = Vec::<u8>::new();
        result.append(&mut set_positions(start, end));
        for e in expr {
            result.append(&mut e.to_bytecode(bytecode_len, var_symtable, func_symtable)?)
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
        bytecode_len: usize,
        var_symtable: &mut BTreeMap<String, u64>,
        func_symtable: &mut BTreeMap<String, FuncAnnotation>,
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
                NodeValue::Bool(val) => {
                    val_to_bytecode!(2, std::slice::from_ref(&u8::from(val)), OpConstB)
                }
            },

            NodeExprInner::VarCall(NodeVarCall(varname)) => {
                // TODO: add proper checks for missing variable
                let mut result = Vec::<u8>::with_capacity(varname.len() + 1);
                result.push(Bytecode::OpGetVar as u8);
                let var_id = get_var_id(varname, var_symtable);
                result.extend_from_slice(&var_id.to_ne_bytes());
                // result.extend_from_slice(varname.as_bytes());
                // result.push(0);
                Ok(result)
            }

            NodeExprInner::FuncCall(node) => {
                let Some(annotation) = func_symtable.get(&node.name()) else {
                    let (fn_name, _, start, end, span) = node.disassemble();
                    return Err(RuntimeError::unknown_function(fn_name, start, end, span).into());
                };

                let fn_ty = &annotation.ret_type;
                if *fn_ty == Type::Void {
                    let (_, _, start, end, span) = node.disassemble();
                    return Err(RuntimeError::void_func_expr(start, end, span).into());
                }

                let mut res = Vec::<u8>::new();
                res.append(&mut node.to_bytecode(bytecode_len, var_symtable, func_symtable)?);
                Ok(res)
            }
        }
    }
}
