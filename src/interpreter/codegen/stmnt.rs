use super::ToBytecode;

use crate::error::{ChalError, CompileError};
use crate::interpreter::{Chalcedony, VarAnnotation, WhileScope};
use crate::parser::ast::{
    NodeAssign, NodeBreakStmnt, NodeContStmnt, NodeElifStmnt, NodeElseStmnt, NodeExprInner,
    NodeForLoop, NodeIfBranch, NodeIfStmnt, NodeRetStmnt, NodeStmnt, NodeWhileLoop,
};

use crate::common::operators::{AssignOprType, BinOprType};
use crate::common::{Bytecode, Type};

fn increment_while_scope(interpreter: &mut Chalcedony, val: usize) {
    if let Some(while_scope) = interpreter.current_while.as_mut() {
        while_scope.current_length += val;
    }
}

fn set_while_scope(interpreter: &mut Chalcedony, val: usize) {
    if let Some(while_scope) = interpreter.current_while.as_mut() {
        while_scope.current_length = val;
    }
}

impl ToBytecode for NodeStmnt {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let result: Vec<Bytecode> = match self {
            /* the code here cannot be infered by `NodeVarDef::to_bytecode()` */
            NodeStmnt::VarDef(mut node) => {
                if interpreter.locals.borrow().contains_key(&node.name) {
                    return Err(CompileError::redefining_variable(node.span.clone()).into());
                }

                /* check whether the variable exists as a function's argument */
                if let Some(func) = interpreter.current_func.clone() {
                    if func.arg_lookup.get(&node.name).is_some() {
                        return Err(CompileError::redefining_function_arg(node.span).into());
                    }
                }

                let mut result = node.value.clone().to_bytecode(interpreter)?;

                let value_type = node.value.as_type(interpreter)?;
                if node.ty != Type::Any {
                    Type::verify(
                        node.ty.clone(),
                        value_type,
                        &mut result,
                        node.value.span.clone(),
                    )?;
                } else {
                    node.ty = value_type;
                }

                /* this implicitly adds the variable to the locals symtable */
                let var_id = interpreter.get_local_id(&node);
                result.push(Bytecode::SetLocal(var_id));

                result
            }

            NodeStmnt::RetStmnt(node) => node.to_bytecode(interpreter)?,
            NodeStmnt::FuncCall(node) => node.to_bytecode(interpreter)?,
            NodeStmnt::IfStmnt(node) => node.to_bytecode(interpreter)?,
            NodeStmnt::WhileLoop(node) => node.to_bytecode(interpreter)?,
            NodeStmnt::Assign(node) => node.to_bytecode(interpreter)?,
            NodeStmnt::ContStmnt(node) => node.to_bytecode(interpreter)?,
            NodeStmnt::BreakStmnt(node) => node.to_bytecode(interpreter)?,
            NodeStmnt::ForLoop(node) => node.to_bytecode(interpreter)?,
        };

        increment_while_scope(interpreter, result.len());

        Ok(result)
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
        let prev_while_scope_len = interpreter
            .current_while
            .as_ref()
            .unwrap_or(&WhileScope::default())
            .current_length;

        let mut result = self.condition.clone().to_bytecode(interpreter)?;

        let cond_ty = self.condition.as_type(interpreter)?;
        Type::verify(
            Type::Bool,
            cond_ty,
            &mut result,
            self.condition.span.clone(),
        )?;

        increment_while_scope(interpreter, result.len() + 1);

        let body = self.body.to_bytecode(interpreter)?;

        /* the extra length is for potential jumps over other branches */
        result.push(Bytecode::If(body.len() + 1));
        result.extend(body);

        set_while_scope(interpreter, prev_while_scope_len);

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
        let mut errors = Vec::<ChalError>::new();

        for stmnt in self {
            match stmnt.to_bytecode(interpreter) {
                Ok(bytecode) => result.extend(bytecode),
                Err(err) => errors.push(err),
            }
        }

        if !errors.is_empty() {
            return Err(errors.into());
        }

        Ok(result)
    }
}

impl ToBytecode for NodeIfStmnt {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let prev_while_scope_len = interpreter
            .current_while
            .as_ref()
            .unwrap_or(&WhileScope::default())
            .current_length;

        let mut result = self.condition.clone().to_bytecode(interpreter)?;
        let cond_ty = self.condition.as_type(interpreter)?;
        Type::verify(
            Type::Bool,
            cond_ty,
            &mut result,
            self.condition.span.clone(),
        )?;

        increment_while_scope(interpreter, result.len() + 1);

        let body = self.body.to_bytecode(interpreter)?;

        increment_while_scope(interpreter, body.len() + 1);

        let mut branches: Vec<Vec<Bytecode>> = Vec::new();
        let mut errors: Vec<ChalError> = Vec::new();
        for branch in self.branches {
            match branch.to_bytecode(interpreter) {
                Ok(bytecode) => {
                    increment_while_scope(interpreter, bytecode.len() + 1);
                    branches.push(bytecode);
                }
                Err(err) => errors.push(err),
            }
        }

        set_while_scope(interpreter, prev_while_scope_len + 1);

        if !errors.is_empty() {
            return Err(errors.into());
        }

        let mut branches_len: usize = branches.iter().map(|el| el.len()).sum();
        branches_len += branches.len();

        let mut leftover_branch_len: isize = branches_len as isize;

        fn push_jump(code: &mut Vec<Bytecode>, jmp_dist: isize) {
            if jmp_dist > 0 {
                code.push(Bytecode::Jmp(jmp_dist));
            } else {
                /* this doesn't have any performance impact, but helps when debugging */
                code.push(Bytecode::Nop);
            }
        }

        result.push(Bytecode::If(body.len() + 1));
        result.extend(body);
        push_jump(&mut result, leftover_branch_len);

        for branch in branches.into_iter() {
            leftover_branch_len -= (branch.len() + 1) as isize;
            result.extend(branch);
            push_jump(&mut result, leftover_branch_len);
        }

        Ok(result)
    }
}

impl ToBytecode for NodeWhileLoop {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let prev_while_scope = interpreter.current_while.clone();
        interpreter.current_while = Some(WhileScope::default());

        /* for some reason nested while loops need an extra len */
        if prev_while_scope.is_some() {
            increment_while_scope(interpreter, 1);
        }

        let mut result = self.condition.clone().to_bytecode(interpreter)?;

        let cond_ty = self.condition.as_type(interpreter)?;
        Type::verify(
            Type::Bool,
            cond_ty,
            &mut result,
            self.condition.span.clone(),
        )?;

        increment_while_scope(interpreter, result.len() + 1);

        let body = self.body.to_bytecode(interpreter)?;
        let body_len = body.len() + 1; // taking into account the jump backwards

        result.push(Bytecode::If(body_len));
        result.extend(body);

        let scope = interpreter
            .current_while
            .as_ref()
            .expect("the while scope disappeared");

        /* finish off any break statements */
        for pos in &scope.unfinished_breaks {
            /* the distance to terminate the while */
            let distance: isize = (result.len() - pos) as isize;
            *result.get_mut(*pos).unwrap() = Bytecode::Jmp(distance);
        }

        /* how much to go back when we have iterated the body */
        let dist = -(result.len() as isize) - 1;
        result.push(Bytecode::Jmp(dist));

        interpreter.current_while = prev_while_scope;
        Ok(result)
    }
}

enum VarScope {
    Arg,
    Local,
    Global,
}

impl ToBytecode for NodeAssign {
    fn to_bytecode(mut self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        /* the annotation of the mutated variable */
        let annotation: VarAnnotation;
        let mut scope = VarScope::Global;

        if let Some(var) = interpreter.locals.borrow().get(&self.lhs.name) {
            annotation = var.clone();
            scope = VarScope::Local;

        /* check whether the interpreter is compiling inside a function scope */
        } else if let Some(func) = interpreter.current_func.clone() {
            /* check whether the variable is an argument */
            if let Some(arg) = func.arg_lookup.get(&self.lhs.name) {
                annotation = VarAnnotation::new(arg.id, arg.ty.clone());
                scope = VarScope::Arg;

            /* check whether the variable is a local variable */
            } else {
                return Err(CompileError::mutating_external_state(self.lhs.span).into());
            }

        /* the interpreter is in the global scope*/
        } else if let Some(var) = interpreter.globals.get(&self.lhs.name) {
            annotation = var.clone();
        } else {
            return Err(CompileError::unknown_variable(self.lhs.name, self.lhs.span).into());
        }

        let mut result = Vec::<Bytecode>::new();
        if self.opr != AssignOprType::Eq {
            match scope {
                VarScope::Arg => result.push(Bytecode::GetArg(annotation.id)),
                VarScope::Local => result.push(Bytecode::GetLocal(annotation.id)),
                VarScope::Global => result.push(Bytecode::GetGlobal(annotation.id)),
            }

            self.rhs
                .expr
                .push_front(NodeExprInner::VarCall(self.lhs.clone()));

            macro_rules! push_bin_opr {
                ($expr:expr, $type:ident) => {{
                    $expr.push_back(NodeExprInner::BinOpr(BinOprType::$type))
                }};
            }

            match self.opr {
                AssignOprType::AddEq => push_bin_opr!(self.rhs.expr, Add),
                AssignOprType::SubEq => push_bin_opr!(self.rhs.expr, Sub),
                AssignOprType::MulEq => push_bin_opr!(self.rhs.expr, Mul),
                AssignOprType::DivEq => push_bin_opr!(self.rhs.expr, Div),
                AssignOprType::ModEq => push_bin_opr!(self.rhs.expr, Mod),
                _ => unreachable!(),
            }
        }

        result.extend(self.rhs.clone().to_bytecode(interpreter)?);

        let rhs_ty = self.rhs.as_type(interpreter)?;
        Type::verify(annotation.ty, rhs_ty, &mut result, self.rhs.span)?;

        match scope {
            VarScope::Arg => result.push(Bytecode::SetArg(annotation.id)),
            VarScope::Local => result.push(Bytecode::SetLocal(annotation.id)),
            VarScope::Global => result.push(Bytecode::SetGlobal(annotation.id)),
        }

        Ok(result)
    }
}

impl ToBytecode for NodeRetStmnt {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let Some(func) = interpreter.current_func.clone() else {
            return Err(CompileError::return_outside_func(self.span.clone()).into());
        };

        let recv_type = self.value.as_type(interpreter)?;
        let exp_type = func.ret_type.clone();

        if exp_type == Type::Void && recv_type == Type::Void {
            return Ok(vec![Bytecode::ReturnVoid]);
        }

        let mut result = self.value.clone().to_bytecode(interpreter)?;

        Type::verify(exp_type, recv_type, &mut result, self.value.span)?;

        result.push(Bytecode::Return);
        Ok(result)
    }
}

impl ToBytecode for NodeBreakStmnt {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let Some(scope) = interpreter.current_while.as_mut() else {
            return Err(CompileError::control_flow_outside_while(self.span.clone()).into());
        };
        scope.unfinished_breaks.push(scope.current_length);
        Ok(vec![Bytecode::Nop])
    }
}

impl ToBytecode for NodeContStmnt {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        let Some(scope) = interpreter.current_while.as_mut() else {
            return Err(CompileError::control_flow_outside_while(self.span.clone()).into());
        };
        Ok(vec![Bytecode::Jmp(-(scope.current_length as isize))])
    }
}

impl ToBytecode for NodeForLoop {
    fn to_bytecode(self, interpreter: &mut Chalcedony) -> Result<Vec<Bytecode>, ChalError> {
        if interpreter.locals.borrow().contains_key(&self.iter.name) {
            return Err(CompileError::redefining_variable(self.iter.span.clone()).into());
        }

        /* check whether the variable exists as a function's argument */
        if let Some(func) = interpreter.current_func.clone() {
            if func.arg_lookup.get(&self.iter.name).is_some() {
                return Err(CompileError::redefining_function_arg(self.iter.span).into());
            }
        }

        Ok(vec![])
    }
}
