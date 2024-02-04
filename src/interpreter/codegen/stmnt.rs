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
            NodeStmnt::VarDef(mut node) => {
                if interpreter.locals.borrow().contains_key(&node.name) {
                    return Err(CompileError::redefining_variable(node.span.clone()).into());
                }

                /* check whether the variable exists as a function's argument */
                if let Some(func) = interpreter.current_func.clone() {
                    if let Some(_) = func.borrow().arg_lookup.get(&node.name) {
                        return Err(CompileError::redefining_function_arg(node.span).into());
                    }
                }

                let value_type = node.value.as_type(&interpreter)?;
                if node.ty != Type::Any {
                    if node.ty != value_type {
                        return Err(CompileError::invalid_type(
                            node.ty,
                            value_type,
                            node.value.span.clone(),
                        )
                        .into());
                    }
                } else {
                    node.ty = value_type;
                }

                let mut result = Vec::<Bytecode>::new();
                result.append(&mut node.value.clone().to_bytecode(interpreter)?);

                /* this implicitly adds the variable to the locals symtable */
                let var_id = interpreter.get_local_id(&node);
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

        let mut branches: Vec<Vec<Bytecode>> = Vec::new();
        let mut err_vec: Vec<ChalError> = Vec::new();
        for branch in self.branches {
            match branch.to_bytecode(interpreter) {
                Ok(bytecode) => branches.push(bytecode),
                Err(err) => err_vec.push(err),
            }
        }

        if !err_vec.is_empty() {
            return Err(err_vec.into());
        }

        let mut result = Vec::<Bytecode>::new();
        result.append(&mut cond);

        /* a single if statement */
        if branches.len() == 0 {
            result.push(Bytecode::If(body.len()));
            result.append(&mut body);

        /* if statement with a single branch */
        } else if branches.len() == 0 {
            result.push(Bytecode::If(body.len() + 1));
            result.append(&mut body);

            let branch = branches.first_mut().unwrap();
            result.push(Bytecode::Jmp(branch.len() as isize));
            result.append(branch);

        /* if statement with more branches */
        } else {
            let mut branches_len: usize = branches.iter().map(|el| el.len()).sum();
            branches_len += branches.len() - 1;

            let mut leftover_branch_len: isize = branches_len as isize;

            result.push(Bytecode::If(body.len() + 1));
            result.append(&mut body);
            result.push(Bytecode::Jmp(leftover_branch_len));

            let branches_count = branches.len() - 1;
            for (idx, mut branch) in branches.into_iter().enumerate() {
                leftover_branch_len = leftover_branch_len - (branch.len() + 1) as isize;
                result.append(&mut branch);
                if idx < branches_count {
                    result.push(Bytecode::Jmp(leftover_branch_len));
                }
            }
        }

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

enum VarScope {
    Arg,
    Local,
    Global,
}

impl ToBytecode for NodeAssign {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let var_id: usize;
        let mut scope = VarScope::Global;

        if let Some(var) = interpreter.locals.borrow().get(&self.lhs.name) {
            var_id = var.id;
            scope = VarScope::Local;

        /* check whether the interpreter is compiling inside a function scope */
        } else if let Some(func) = interpreter.current_func.clone() {
            let func = func.borrow();

            /* check whether the variable is an argument */
            if let Some(var) = func.arg_lookup.get(&self.lhs.name) {
                var_id = var.id;
                scope = VarScope::Arg;

            /* check whether the variable is a local variable */
            } else {
                return Err(CompileError::mutating_external_state(self.lhs.span).into());
            }

        /* the interpreter is in the global scope*/
        } else {
            if let Some(var) = interpreter.globals.get(&self.lhs.name) {
                var_id = var.id;
            } else {
                return Err(CompileError::unknown_variable(self.lhs.name, self.lhs.span).into());
            }
        }

        let mut result = Vec::<Bytecode>::new();
        if self.opr != AssignOprType::Eq {
            match scope {
                VarScope::Arg => result.push(Bytecode::GetArg(var_id)),
                VarScope::Local => result.push(Bytecode::GetLocal(var_id)),
                VarScope::Global => result.push(Bytecode::GetGlobal(var_id)),
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

        match scope {
            VarScope::Arg => result.push(Bytecode::SetArg(var_id)),
            VarScope::Local => result.push(Bytecode::SetLocal(var_id)),
            VarScope::Global => result.push(Bytecode::SetGlobal(var_id)),
        }

        Ok(result)
    }
}
