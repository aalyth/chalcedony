use super::ToBytecode;

use crate::error::ChalError;
use crate::interpreter::Chalcedony;
use crate::parser::ast::{NodeExpr, NodeExprInner, NodeValue};

use crate::common::operators::{BinOprType, UnaryOprType};
use crate::common::Bytecode;

impl ToBytecode for NodeExpr {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let mut result = Vec::<Bytecode>::new();
        for inner in self.expr {
            result.extend(inner.to_bytecode(interpreter)?);
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

            NodeExprInner::FuncCall(node) => node.to_bytecode(interpreter),
        }
    }
}
