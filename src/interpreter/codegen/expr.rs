use super::ToBytecode;

use crate::error::{ChalError, RuntimeError};
use crate::interpreter::Chalcedony;
use crate::lexer::Type;
use crate::parser::ast::operators::{BinOprType, UnaryOprType};
use crate::parser::ast::{NodeExpr, NodeExprInner, NodeValue};
use crate::utils::Bytecode;

impl ToBytecode for NodeExpr {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let mut result = Vec::<Bytecode>::new();
        for e in self.expr {
            result.append(&mut e.to_bytecode(interpreter)?)
        }
        Ok(result)
    }
}

impl ToBytecode for NodeExprInner {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        match self {
            NodeExprInner::BinOpr(opr_type) => match opr_type {
                BinOprType::Add => Ok(vec![Bytecode::Add]),
                BinOprType::Sub => Ok(vec![Bytecode::Sub]),
                BinOprType::Mul => Ok(vec![Bytecode::Mul]),
                BinOprType::Div => Ok(vec![Bytecode::Div]),
                BinOprType::Mod => Ok(vec![Bytecode::Mod]),

                BinOprType::And => Ok(vec![Bytecode::And]),
                BinOprType::Or => Ok(vec![Bytecode::Or]),

                BinOprType::Lt => Ok(vec![Bytecode::Lt]),
                BinOprType::Gt => Ok(vec![Bytecode::Gt]),

                BinOprType::LtEq => Ok(vec![Bytecode::LtEq]),
                BinOprType::GtEq => Ok(vec![Bytecode::GtEq]),

                BinOprType::EqEq => Ok(vec![Bytecode::Eq]),
                BinOprType::BangEq => Ok(vec![Bytecode::Eq, Bytecode::Not]),
            },

            NodeExprInner::UnaryOpr(opr_type) => match opr_type {
                UnaryOprType::Neg => Ok(vec![Bytecode::Neg]),
                UnaryOprType::Bang => Ok(vec![Bytecode::Not]),
            },

            NodeExprInner::Value(val_node) => match val_node {
                NodeValue::Int(val) => Ok(vec![Bytecode::ConstI(val)]),
                NodeValue::Uint(val) => Ok(vec![Bytecode::ConstU(val)]),
                NodeValue::Float(val) => Ok(vec![Bytecode::ConstF(val)]),
                NodeValue::Str(val) => Ok(vec![Bytecode::ConstS(val.into())]),
                NodeValue::Bool(val) => Ok(vec![Bytecode::ConstB(val)]),
            },

            NodeExprInner::VarCall(node) => node.to_bytecode(interpreter),

            NodeExprInner::FuncCall(node) => {
                let Some(annotation) = interpreter.func_symtable.get(&node.name).cloned() else {
                    return Err(RuntimeError::unknown_function(node.name, node.span).into());
                };

                let fn_ty = &annotation.borrow().ret_type;
                if *fn_ty == Type::Void {
                    return Err(RuntimeError::void_func_expr(node.span).into());
                }

                node.to_bytecode(interpreter)
            }
        }
    }
}
