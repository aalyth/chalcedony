use super::ToBytecode;

use crate::error::{ChalError, CompileError};
use crate::interpreter::Chalcedony;
use crate::parser::ast::{
    NodeAssign, NodeElifStmnt, NodeElseStmnt, NodeIfBranch, NodeIfStmnt, NodeRetStmnt, NodeStmnt,
    NodeWhileLoop,
};

use crate::common::{operators::AssignOprType, Bytecode, Type};

impl ToBytecode for NodeStmnt {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        match self {
            NodeStmnt::VarDef(node) => {
                // check whether we're redefining a function's argument
                if let Some(func) = interpreter.current_func.clone() {
                    if let Some(_) = func.borrow().args.get(&node.name) {
                        return Err(CompileError::redefining_function_arg(node.span).into());
                    }
                }

                if node.ty != Type::Any {
                    let value_type = node.value.as_type(&interpreter)?;
                    if node.ty != value_type {
                        return Err(CompileError::invalid_type(
                            node.ty,
                            value_type,
                            node.value.span.clone(),
                        )
                        .into());
                    }
                }

                let var_id = interpreter.get_local_id(&node);

                let mut result = Vec::<Bytecode>::new();
                result.append(&mut node.value.to_bytecode(interpreter)?);
                result.push(Bytecode::SetLocal(var_id));

                Ok(result)
            }

            NodeStmnt::FuncCall(node) => node.to_bytecode(interpreter),

            NodeStmnt::RetStmnt(NodeRetStmnt(expr)) => {
                let func = interpreter
                    .current_func
                    .clone()
                    .expect("calling return outside of function");

                let return_type = expr.as_type(&interpreter)?;
                if func.borrow().ret_type != return_type {
                    return Err(CompileError::invalid_type(
                        func.borrow().ret_type,
                        return_type,
                        expr.span.clone(),
                    )
                    .into());
                }

                if return_type == Type::Void {
                    return Ok(vec![Bytecode::ReturnVoid]);
                }

                let mut result = Vec::<Bytecode>::new();
                result.append(&mut expr.to_bytecode(interpreter)?);
                // TODO: check whether the return is a single function call optimize for TAIL_CALL
                result.push(Bytecode::Return);
                Ok(result)
            }

            NodeStmnt::IfStmnt(node) => node.to_bytecode(interpreter),

            NodeStmnt::WhileLoop(node) => node.to_bytecode(interpreter),

            NodeStmnt::Assign(node) => node.to_bytecode(interpreter),
        }
    }
}

impl ToBytecode for NodeIfBranch {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        match self {
            NodeIfBranch::Elif(node) => node.to_bytecode(interpreter),
            NodeIfBranch::Else(node) => node.to_bytecode(interpreter),
        }
    }
}

impl ToBytecode for NodeElifStmnt {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let mut cond_bytecode = self.condition.to_bytecode(interpreter)?;
        let mut body_bytecode = self.body.to_bytecode(interpreter)?;

        let cond_len = cond_bytecode.len();
        let body_len = body_bytecode.len();

        let mut result = Vec::<Bytecode>::with_capacity(cond_len + body_len + 1);
        result.append(&mut cond_bytecode);
        result.push(Bytecode::If(body_len + 1));
        result.append(&mut body_bytecode);

        Ok(result)
    }
}

impl ToBytecode for NodeElseStmnt {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        self.body.to_bytecode(interpreter)
    }
}

impl ToBytecode for Vec<NodeStmnt> {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let mut result = Vec::<Bytecode>::new();
        let mut err_vec = Vec::<ChalError>::new();

        for stmnt in self {
            match stmnt.to_bytecode(interpreter) {
                Ok(mut bytecode) => result.append(&mut bytecode),
                Err(err) => err_vec.push(err),
            }
        }

        if !err_vec.is_empty() {
            return Err(err_vec.into());
        }

        Ok(result)
    }
}

impl ToBytecode for NodeIfStmnt {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let mut cond = self.condition.to_bytecode(interpreter)?;
        let mut body = self.body.to_bytecode(interpreter)?;

        let mut branch_bytecodes: Vec<Vec<Bytecode>> = Vec::new();
        let mut err_vec: Vec<ChalError> = Vec::new();
        for branch in self.branches {
            match branch.to_bytecode(interpreter) {
                Ok(body) => branch_bytecodes.push(body),
                Err(err) => err_vec.push(err),
            }
        }

        if !err_vec.is_empty() {
            return Err(err_vec.into());
        }

        let mut bodies_len: usize = branch_bytecodes.iter().map(|el| el.len()).sum();
        let body_len = body.len() + 1;
        bodies_len += branch_bytecodes.len() - 1; // we add one space for each jump to the end of the
                                                  // condition

        let mut result = Vec::<Bytecode>::with_capacity(cond.len() + 1 + body.len() + bodies_len);
        let mut leftover_branch_len: isize = bodies_len as isize;
        result.append(&mut cond);
        result.push(Bytecode::If(body_len));
        result.append(&mut body);
        result.push(Bytecode::Jmp(leftover_branch_len));

        if branch_bytecodes.len() == 0 {
            return Ok(result);
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
}

impl ToBytecode for NodeWhileLoop {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let mut result = Vec::<Bytecode>::new();
        result.append(&mut self.condition.to_bytecode(interpreter)?);
        /* there is no need to use a type assertion for bool value since the OpIf
         * instruction already checks it */

        let mut body = self.body.to_bytecode(interpreter)?;

        /* skipping over the body if the condition is false */
        let body_len = body.len() + 1;
        result.push(Bytecode::If(body_len));
        result.append(&mut body);

        /* how much to go back when we have iterated the body */
        let dist = -(result.len() as isize) - 1;
        result.push(Bytecode::Jmp(dist));

        Ok(result)
    }
}

impl ToBytecode for NodeAssign {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let var_id: usize;
        let mut scope_is_global = true;

        if let Some(func) = interpreter.current_func.clone() {
            let func = func.borrow();
            if let Some(var) = func.args.get(&self.lhs.name) {
                var_id = var.id;
            } else if let Some(var) = func.locals.get(&self.lhs.name) {
                var_id = var.id;
            } else {
                return Err(CompileError::stateful_function(self.lhs.span).into());
            }
        } else {
            if let Some(var) = interpreter.globals.get(&self.lhs.name) {
                var_id = var.id;
                scope_is_global = true;
            } else {
                return Err(CompileError::unknown_variable(self.lhs.name, self.lhs.span).into());
            }
        }

        let mut result = Vec::<Bytecode>::new();
        if self.opr != AssignOprType::Eq {
            if scope_is_global {
                result.push(Bytecode::GetGlobal(var_id));
            } else {
                result.push(Bytecode::GetLocal(var_id));
            }
        }

        result.append(&mut self.rhs.to_bytecode(interpreter)?);

        match self.opr {
            AssignOprType::AddEq => result.push(Bytecode::Add),
            AssignOprType::SubEq => result.push(Bytecode::Sub),
            AssignOprType::MulEq => result.push(Bytecode::Mul),
            AssignOprType::DivEq => result.push(Bytecode::Div),
            AssignOprType::ModEq => result.push(Bytecode::Mod),
            _ => {}
        }

        if scope_is_global {
            result.push(Bytecode::SetGlobal(var_id));
        } else {
            result.push(Bytecode::SetLocal(var_id));
        }

        Ok(result)
    }
}
