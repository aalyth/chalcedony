use super::{compile_func_call_inner, ToBytecode};

use crate::common::{Bytecode, Type};
use crate::error::{ChalError, CompileError, CompileErrorKind};
use crate::interpreter::{Chalcedony, ClassNamespace, MemberAnnotation};
use crate::parser::ast::{NodeAttrRes, NodeAttribute, NodeClass, NodeVarCall};

use std::collections::HashSet;

impl ToBytecode for NodeClass {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        if interpreter.namespaces.contains_key(&self.name)
            || interpreter.builtins.contains_key(&self.name)
        {
            return Err(CompileError::new(
                CompileErrorKind::ClassAlreadyExists(self.name),
                self.span,
            )
            .into());
        }

        let mut namespace = ClassNamespace::default();
        let mut lookup = HashSet::<String>::new();
        for (id, member) in self.members.iter().enumerate() {
            if member.ty == Type::Void {
                return Err(
                    CompileError::new(CompileErrorKind::VoidMember, member.span.clone()).into(),
                );
            }

            interpreter.verify_type(&member.ty, &member.span)?;

            if !lookup.insert(member.name.clone()) {
                return Err(CompileError::new(
                    CompileErrorKind::MemberAlreadyExists,
                    member.span.clone(),
                )
                .into());
            }

            namespace.members.push(MemberAnnotation {
                id,
                name: member.name.clone(),
                ty: member.ty.clone(),
            });
        }

        interpreter.namespaces.insert(self.name, namespace);
        for method in self.methods {
            let code = method.to_bytecode(interpreter)?;
            interpreter.vm.execute(code);
        }

        Ok(vec![])
    }
}

fn compile_attribute_access(
    node: NodeVarCall,
    interpreter: &mut Chalcedony,
    parent_type: Option<Type>,
) -> Result<Vec<Bytecode>, ChalError> {
    if let Some(ty) = parent_type {
        let class_name = ty.as_class();
        let Some(class) = interpreter.namespaces.get(&class_name) else {
            return Err(CompileError::new(
                CompileErrorKind::UnknownNamespace(class_name),
                node.span,
            )
            .into());
        };

        let Some(annotation) = class.get_member(&node.name) else {
            return Err(CompileError::new(
                CompileErrorKind::UnknownMember(node.name.clone()),
                node.span,
            )
            .into());
        };

        return Ok(vec![Bytecode::GetAttr(annotation.id)]);
    }

    node.to_bytecode(interpreter)
}

impl ToBytecode for NodeAttrRes {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let mut result = Vec::<Bytecode>::new();
        let mut parent_type: Option<Type> = None;

        for node in self.resolution {
            match node {
                NodeAttribute::VarCall(node) => {
                    let current_type = Some(node.as_type(interpreter, parent_type.clone())?);
                    result.extend(compile_attribute_access(
                        node,
                        interpreter,
                        parent_type.clone(),
                    )?);
                    parent_type = current_type;
                }
                NodeAttribute::FuncCall(node) => {
                    let current_type = Some(node.as_type(interpreter, parent_type.clone())?);
                    result.extend(compile_func_call_inner(
                        node,
                        interpreter,
                        parent_type.clone(),
                    )?);
                    parent_type = current_type;
                }
            }
        }

        Ok(result)
    }
}
