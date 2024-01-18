use super::var::get_var_id;
use super::ToBytecode;

use crate::error::{ChalError, RuntimeError};
use crate::interpreter::FuncAnnotation;
use crate::lexer::Type;
use crate::parser::ast::operators::AssignOprType;
use crate::parser::ast::stmnt::{NodeElifStmnt, NodeElseStmnt, NodeIfBranch};
use crate::parser::ast::{NodeRetStmnt, NodeStmnt, NodeVarCall};
use crate::utils::Bytecode;

use std::collections::{BTreeMap, HashSet};

pub fn stmnt_to_bytecode(
    node: NodeStmnt,
    bytecode_len: usize,
    var_symtable: &mut BTreeMap<String, usize>,
    func_symtable: &mut BTreeMap<String, FuncAnnotation>,
    parent_scope: &mut HashSet<String>,
    local_scope: &mut HashSet<String>,
) -> Result<Vec<Bytecode>, ChalError> {
    match node {
        NodeStmnt::VarDef(node) => {
            let var_name = node.name();
            let mut var_bytecode = node.to_bytecode(bytecode_len, var_symtable, func_symtable)?;

            /* this capacity is for the best case scenario, if we need to delete the already
             * existing variable, there will be another allocation*/
            let mut result = Vec::<Bytecode>::with_capacity(var_bytecode.len() + 1);

            /* 'shadow' the old variable value */
            if local_scope.contains(&var_name) {
                let var_id = get_var_id(var_name, var_symtable);
                result.push(Bytecode::DeleteVar(var_id))
            } else {
                local_scope.insert(var_name);
            }

            result.append(&mut var_bytecode);
            return Ok(result);
        }

        NodeStmnt::FuncCall(node) => {
            let Some(annotation) = func_symtable.get(&node.name) else {
                return Err(RuntimeError::unknown_function(node.name, node.span).into());
            };

            let fn_ty = &annotation.ret_type;
            if *fn_ty != Type::Void {
                return Err(RuntimeError::non_void_func_stmnt(fn_ty.clone(), node.span).into());
            }

            node.to_bytecode(bytecode_len, var_symtable, func_symtable)
        }

        NodeStmnt::RetStmnt(NodeRetStmnt(expr)) => {
            let mut result = Vec::<Bytecode>::new();
            result.append(&mut expr.to_bytecode(bytecode_len, var_symtable, func_symtable)?);

            /* remove all variables in the current scope */
            for var in parent_scope.iter() {
                let var_id = get_var_id(var.clone(), var_symtable);
                result.push(Bytecode::DeleteVar(var_id))
            }

            for var in local_scope.iter() {
                let var_id = get_var_id(var.clone(), var_symtable);
                result.push(Bytecode::DeleteVar(var_id))
            }

            result.push(Bytecode::Return);
            Ok(result)
        }

        NodeStmnt::IfStmnt(node) => {
            let mut cond = node
                .condition
                .to_bytecode(bytecode_len, var_symtable, func_symtable)?;
            let mut body = parse_body(
                node.body,
                bytecode_len,
                var_symtable,
                func_symtable,
                parent_scope,
                local_scope,
            )?;

            let mut branch_bytecodes: Vec<Vec<Bytecode>> = Vec::new();
            let mut err_vec: Vec<ChalError> = Vec::new();
            for branch in node.branches {
                let raw_res = parse_branch(
                    branch,
                    bytecode_len,
                    var_symtable,
                    func_symtable,
                    parent_scope,
                    local_scope,
                );

                match raw_res {
                    Ok(body) => branch_bytecodes.push(body),
                    Err(err) => err_vec.push(err),
                }
            }

            let mut bodies_len: usize = branch_bytecodes.iter().map(|el| el.len()).sum();
            let body_len = body.len() + 1;
            bodies_len += branch_bytecodes.len() - 1; // we add one space for each jump to the end of the
                                                      // condition

            let mut result =
                Vec::<Bytecode>::with_capacity(cond.len() + 1 + body.len() + bodies_len);
            let mut leftover_branch_len: isize = bodies_len as isize;
            result.append(&mut cond);
            result.push(Bytecode::If(body_len));
            result.append(&mut body);
            result.push(Bytecode::Jmp(leftover_branch_len));

            if !err_vec.is_empty() {
                return Err(err_vec.into());
            }

            if branch_bytecodes.len() == 1 {
                result.append(branch_bytecodes.get_mut(0).unwrap());
                return Ok(result);
            }

            for i in 0..branch_bytecodes.len() - 1 {
                let mut branch = branch_bytecodes.get_mut(i).unwrap();
                leftover_branch_len = leftover_branch_len - (branch.len() + 1) as isize;
                result.append(&mut branch);
                result.push(Bytecode::Jmp(leftover_branch_len));
            }

            let mut last = branch_bytecodes.last_mut().unwrap();
            result.append(&mut last);

            Ok(result)
        }

        NodeStmnt::WhileLoop(node) => {
            let (cond, body_raw) = node.disassemble();

            let mut result = Vec::<Bytecode>::new();
            result.append(&mut cond.to_bytecode(bytecode_len, var_symtable, func_symtable)?);
            /* there is no need to use a type assertion for bool value since the OpIf
             * instruction already checks it */

            let mut body = parse_body(
                body_raw,
                bytecode_len,
                var_symtable,
                func_symtable,
                parent_scope,
                local_scope,
            )?;

            /* skipping over the body if the condition is false */
            let body_len = body.len() + 1;
            result.push(Bytecode::If(body_len));
            result.append(&mut body);

            /* how much to go back when we have iterated the body */
            let dist = -(result.len() as isize) - 1;
            result.push(Bytecode::Jmp(dist));

            Ok(result)
        }

        NodeStmnt::Assign(node) => {
            let (var_node, opr, rhs) = node.disassemble();
            let var_id = get_var_id(var_node.name, var_symtable);
            let mut result = Vec::<Bytecode>::new();
            if opr != AssignOprType::Eq {
                result.push(Bytecode::GetVar(var_id));
            }

            result.append(&mut rhs.to_bytecode(bytecode_len, var_symtable, func_symtable)?);

            match opr {
                AssignOprType::AddEq => result.push(Bytecode::Add),
                AssignOprType::SubEq => result.push(Bytecode::Sub),
                AssignOprType::MulEq => result.push(Bytecode::Mul),
                AssignOprType::DivEq => result.push(Bytecode::Div),
                AssignOprType::ModEq => result.push(Bytecode::Mod),
                _ => {}
            }

            result.push(Bytecode::DeleteVar(var_id));
            result.push(Bytecode::CreateVar(var_id));

            Ok(result)
        }
    }
}

fn parse_branch(
    node: NodeIfBranch,
    bytecode_len: usize,
    var_symtable: &mut BTreeMap<String, usize>,
    func_symtable: &mut BTreeMap<String, FuncAnnotation>,
    parent_scope: &mut HashSet<String>,
    local_scope: &mut HashSet<String>,
) -> Result<Vec<Bytecode>, ChalError> {
    match node {
        NodeIfBranch::Else(node_else) => parse_else(
            node_else,
            bytecode_len,
            var_symtable,
            func_symtable,
            parent_scope,
            local_scope,
        ),
        NodeIfBranch::Elif(node_elif) => parse_elif(
            node_elif,
            bytecode_len,
            var_symtable,
            func_symtable,
            parent_scope,
            local_scope,
        ),
    }
}

fn parse_elif(
    node: NodeElifStmnt,
    bytecode_len: usize,
    var_symtable: &mut BTreeMap<String, usize>,
    func_symtable: &mut BTreeMap<String, FuncAnnotation>,
    parent_scope: &mut HashSet<String>,
    local_scope: &mut HashSet<String>,
) -> Result<Vec<Bytecode>, ChalError> {
    let (cond, body_raw) = node.disassemble();

    let mut cond_bytecode = cond.to_bytecode(bytecode_len, var_symtable, func_symtable)?;
    let mut body_bytecode = parse_body(
        body_raw,
        bytecode_len,
        var_symtable,
        func_symtable,
        parent_scope,
        local_scope,
    )?;

    let cond_len = cond_bytecode.len();
    let body_len = body_bytecode.len();

    let mut result = Vec::<Bytecode>::with_capacity(cond_len + body_len + 1);
    result.append(&mut cond_bytecode);
    result.push(Bytecode::If(body_len + 1));
    result.append(&mut body_bytecode);

    Ok(result)
}

fn parse_else(
    node: NodeElseStmnt,
    bytecode_len: usize,
    var_symtable: &mut BTreeMap<String, usize>,
    func_symtable: &mut BTreeMap<String, FuncAnnotation>,
    parent_scope: &mut HashSet<String>,
    local_scope: &mut HashSet<String>,
) -> Result<Vec<Bytecode>, ChalError> {
    let body_raw = node.disassemble();
    parse_body(
        body_raw,
        bytecode_len,
        var_symtable,
        func_symtable,
        parent_scope,
        local_scope,
    )
}

fn parse_body(
    body: Vec<NodeStmnt>,
    bytecode_len: usize,
    var_symtable: &mut BTreeMap<String, usize>,
    func_symtable: &mut BTreeMap<String, FuncAnnotation>,
    parent_scope: &mut HashSet<String>,
    local_scope: &mut HashSet<String>,
) -> Result<Vec<Bytecode>, ChalError> {
    let mut result = Vec::<Bytecode>::new();
    let mut err_vec = Vec::<ChalError>::new();

    let mut current_parent_scope = parent_scope.clone();
    current_parent_scope.extend(local_scope.clone().into_iter());
    let mut current_local_scope = HashSet::<String>::new();

    for stmnt in body {
        let stmnt_res = stmnt_to_bytecode(
            stmnt,
            bytecode_len,
            var_symtable,
            func_symtable,
            &mut current_parent_scope,
            &mut current_local_scope,
        );
        match stmnt_res {
            Ok(mut bytecode) => result.append(&mut bytecode),
            Err(err) => err_vec.push(err),
        }
    }

    if !err_vec.is_empty() {
        return Err(err_vec.into());
    }

    for var in current_local_scope {
        let var_id = get_var_id(var, var_symtable);
        result.push(Bytecode::DeleteVar(var_id));
    }
    Ok(result)
}
