use super::var::get_var_id;
use super::ToBytecode;

use crate::error::{ChalError, RuntimeError};
use crate::interpreter::FuncAnnotation;
use crate::lexer::Type;
use crate::parser::ast::operators::{BinOprType, UnaryOprType};
use crate::parser::ast::{NodeExpr, NodeExprInner, NodeValue, NodeVarCall};
use crate::utils::BytecodeOpr;

use std::collections::BTreeMap;

const U32_BYTES: usize = 4;
const POS_BYTES: usize = 1 + U32_BYTES;

impl ToBytecode for NodeExpr {
    fn to_bytecode(
        self,
        bytecode_len: usize,
        var_symtable: &mut BTreeMap<String, usize>,
        func_symtable: &mut BTreeMap<String, FuncAnnotation>,
    ) -> Result<Vec<BytecodeOpr>, ChalError> {
        let (expr, start, end, _span) = self.disassemble();
        let mut result = Vec::<BytecodeOpr>::new();
        for e in expr {
            result.append(&mut e.to_bytecode(bytecode_len, var_symtable, func_symtable)?)
        }
        Ok(result)
    }
}

impl ToBytecode for NodeExprInner {
    fn to_bytecode(
        self,
        bytecode_len: usize,
        var_symtable: &mut BTreeMap<String, usize>,
        func_symtable: &mut BTreeMap<String, FuncAnnotation>,
    ) -> Result<Vec<BytecodeOpr>, ChalError> {
        match self {
            NodeExprInner::BinOpr(opr_type) => match opr_type {
                BinOprType::Add => Ok(vec![BytecodeOpr::Add]),
                BinOprType::Sub => Ok(vec![BytecodeOpr::Sub]),
                BinOprType::Mul => Ok(vec![BytecodeOpr::Mul]),
                BinOprType::Div => Ok(vec![BytecodeOpr::Div]),
                BinOprType::Mod => Ok(vec![BytecodeOpr::Mod]),

                BinOprType::And => Ok(vec![BytecodeOpr::And]),
                BinOprType::Or => Ok(vec![BytecodeOpr::Or]),

                BinOprType::Lt => Ok(vec![BytecodeOpr::Lt]),
                BinOprType::Gt => Ok(vec![BytecodeOpr::Gt]),

                BinOprType::LtEq => Ok(vec![BytecodeOpr::LtEq]),
                BinOprType::GtEq => Ok(vec![BytecodeOpr::GtEq]),

                BinOprType::EqEq => Ok(vec![BytecodeOpr::Eq]),
                BinOprType::BangEq => Ok(vec![BytecodeOpr::Eq, BytecodeOpr::Not]),
            },

            NodeExprInner::UnaryOpr(opr_type) => match opr_type {
                UnaryOprType::Neg => Ok(vec![BytecodeOpr::Neg]),
                UnaryOprType::Bang => Ok(vec![BytecodeOpr::Not]),
            },

            NodeExprInner::Value(val_node) => match val_node {
                NodeValue::Int(val) => Ok(vec![BytecodeOpr::ConstI(val)]),
                NodeValue::Uint(val) => Ok(vec![BytecodeOpr::ConstU(val)]),
                NodeValue::Float(val) => Ok(vec![BytecodeOpr::ConstF(val)]),
                NodeValue::Str(val) => Ok(vec![BytecodeOpr::ConstS(val.into())]),
                NodeValue::Bool(val) => Ok(vec![BytecodeOpr::ConstB(val)]),
            },

            NodeExprInner::VarCall(NodeVarCall(varname)) => {
                // TODO: add proper checks for missing variable
                let var_id = get_var_id(varname, var_symtable);
                Ok(vec![BytecodeOpr::GetVar(var_id)])
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

                node.to_bytecode(bytecode_len, var_symtable, func_symtable)
            }
        }
    }
}
