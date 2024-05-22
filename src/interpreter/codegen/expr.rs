use super::ToBytecode;

use crate::error::{ChalError, CompileError, CompileErrorKind};
use crate::interpreter::Chalcedony;
use crate::parser::ast::{NodeExpr, NodeExprInner, NodeValue};

use crate::common::operators::{BinOprType, UnaryOprType};
use crate::common::{Bytecode, Type};

impl ToBytecode for NodeExpr {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let mut result = Vec::<Bytecode>::new();

        // since the expressions are already parsed into a Reverse Polish
        // Notation, the only thing needed for their compilation is to convert
        // them to their appropriate bytecode instructions
        for inner in self.expr {
            interpreter.inside_stmnt = false;
            result.extend(inner.to_bytecode(interpreter)?);
            interpreter.inside_stmnt = true;
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

            NodeExprInner::Resolution(node) => node.to_bytecode(interpreter),

            NodeExprInner::InlineClass(mut node) => {
                // TODO: try to remove the clone
                let Some(class) = interpreter.namespaces.get(&node.class).cloned() else {
                    return Err(CompileError::new(
                        CompileErrorKind::UnknownClass(node.class),
                        node.span,
                    )
                    .into());
                };

                if node.members.len() < class.members.len() {
                    let mut missing_members = Vec::<String>::new();
                    for member in class.members {
                        if !node.members.contains_key(&member.name) {
                            missing_members.push(member.name);
                        }
                    }

                    return Err(CompileError::new(
                        CompileErrorKind::MissingMembers(missing_members),
                        node.span,
                    )
                    .into());
                }

                if node.members.len() > class.members.len() {
                    for member in class.members {
                        node.members.remove(&member.name);
                    }
                    let undefined_members = node.members.into_keys().collect();
                    return Err(CompileError::new(
                        CompileErrorKind::UndefinedMembers(undefined_members),
                        node.span,
                    )
                    .into());
                }

                let mut result = Vec::<Bytecode>::new();
                let members_count = class.members.len();

                let mut missing_members = Vec::<String>::new();
                for member in class.members {
                    let Some((expr, span)) = node.members.remove(&member.name) else {
                        missing_members.push(member.name);
                        continue;
                    };
                    let expr_ty = expr.as_type(interpreter)?;
                    result.extend(expr.to_bytecode(interpreter)?);
                    Type::verify(member.ty.clone(), expr_ty, &mut result, span)?;
                }

                if !missing_members.is_empty() {
                    return Err(CompileError::new(
                        CompileErrorKind::MissingMembers(missing_members),
                        node.span,
                    )
                    .into());
                }

                // NOTE: the variable is required, since all node members are
                // removed and the length is 0
                result.push(Bytecode::ConstObj(members_count));

                Ok(result)
            }

            NodeExprInner::List(node) => {
                let mut result = Vec::<Bytecode>::new();
                let list_len = node.elements.len();
                for el in node.elements {
                    result.extend(el.to_bytecode(interpreter)?);
                }
                result.push(Bytecode::ConstL(list_len));
                Ok(result)
            }
        }
    }
}
