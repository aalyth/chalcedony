use crate::utils::PtrString;
use crate::vm::{Cvm, CvmObject};

use super::get_operands;

macro_rules! apply_bin_operator {
    ( $cvm:ident, $current_idx:ident, $opr:tt, $str_opr_handler:ident ) => {{
        let (left, right) = get_operands($cvm);
        match (left, right) {
            (CvmObject::Int(lval), CvmObject::Int(rval)) => $cvm.push(CvmObject::Int(lval $opr rval)),
            (CvmObject::Int(lval), CvmObject::Uint(rval)) => $cvm.push(CvmObject::Int(lval $opr (rval as i64))),
            (CvmObject::Int(lval), CvmObject::Float(rval)) => $cvm.push(CvmObject::Int(lval $opr (rval as i64))),

            (CvmObject::Uint(lval), CvmObject::Int(rval)) => $cvm.push(CvmObject::Int((lval as i64) $opr rval)),
            (CvmObject::Uint(lval), CvmObject::Uint(rval)) => $cvm.push(CvmObject::Uint(lval $opr rval)),
            (CvmObject::Uint(lval), CvmObject::Float(rval)) => $cvm.push(CvmObject::Int((lval as i64) $opr (rval as i64))),

            (CvmObject::Float(lval), CvmObject::Int(rval)) => $cvm.push(CvmObject::Float(lval $opr (rval as f64))),
            (CvmObject::Float(lval), CvmObject::Uint(rval)) => $cvm.push(CvmObject::Float(lval $opr (rval as f64))),
            (CvmObject::Float(lval), CvmObject::Float(rval)) => $cvm.push(CvmObject::Float(lval $opr rval)),

            (CvmObject::Str(lval), right) => $str_opr_handler($cvm, lval.clone(), right),
            (left, right) => panic!(
                "unchecked invalid binary operation - {:?} and {:?}",
                left.as_type(),
                right.as_type()
            ),
        }
        $current_idx
    }};
}

pub fn add(cvm: &mut Cvm, current_idx: usize) -> usize {
    fn add_str(cvm: &mut Cvm, lval: PtrString, right: CvmObject) {
        match right {
            CvmObject::Int(rval) => cvm.push(CvmObject::Str(lval + rval.to_string().into())),
            CvmObject::Uint(rval) => cvm.push(CvmObject::Str(lval + rval.to_string().into())),
            CvmObject::Float(rval) => cvm.push(CvmObject::Str(lval + rval.to_string().into())),
            CvmObject::Str(rval) => cvm.push(CvmObject::Str(lval + rval)),
            CvmObject::Bool(rval) => cvm.push(CvmObject::Str(lval + rval.to_string().into())),
        }
    }
    apply_bin_operator!(cvm, current_idx, +, add_str)
}

pub fn sub(cvm: &mut Cvm, current_idx: usize) -> usize {
    fn sub_str(_: &mut Cvm, _: PtrString, _: CvmObject) {
        panic!("unchecked invalid string operation - string substitution")
    }
    apply_bin_operator!(cvm, current_idx, -, sub_str)
}

pub fn mul(cvm: &mut Cvm, current_idx: usize) -> usize {
    fn mul_str(cvm: &mut Cvm, lval: PtrString, right: CvmObject) {
        match right {
            CvmObject::Uint(rval) => cvm.stack.push(CvmObject::Str(lval * (rval as usize))),
            _ => panic!("unchecked invalid string operation - string multiplication with non-uint"),
        }
    }
    apply_bin_operator!(cvm, current_idx, *, mul_str)
}

pub fn div(cvm: &mut Cvm, current_idx: usize) -> usize {
    fn div_str(_: &mut Cvm, _: PtrString, _: CvmObject) {
        panic!("unchecked invalid string operation - string division")
    }
    apply_bin_operator!(cvm, current_idx, /, div_str)
}

pub fn modulo(cvm: &mut Cvm, current_idx: usize) -> usize {
    fn mod_str(_: &mut Cvm, _: PtrString, _: CvmObject) {
        panic!("unchecked invalid string operation - string substitution")
    }
    apply_bin_operator!(cvm, current_idx, %, mod_str)
}

macro_rules! apply_logic_operator {
    ( $cvm:ident, $current_idx:ident, $opr:tt ) => {{
        let (left, right) = get_operands($cvm);
        match (left, right) {
            (CvmObject::Int(lval), CvmObject::Int(rval)) => $cvm.push(CvmObject::Bool((lval != 0) $opr (rval != 0))),
            (CvmObject::Int(lval), CvmObject::Uint(rval)) => $cvm.push(CvmObject::Bool((lval != 0) $opr (rval != 0))),
            (CvmObject::Int(lval), CvmObject::Float(rval)) => $cvm.push(CvmObject::Bool((lval != 0) $opr (rval != 0.0))),
            (CvmObject::Int(lval), CvmObject::Bool(rval)) => $cvm.push(CvmObject::Bool((lval != 0) $opr rval)),

            (CvmObject::Uint(lval), CvmObject::Int(rval)) => $cvm.push(CvmObject::Bool((lval != 0) $opr (rval != 0))),
            (CvmObject::Uint(lval), CvmObject::Uint(rval)) => $cvm.push(CvmObject::Bool((lval != 0) $opr (rval != 0))),
            (CvmObject::Uint(lval), CvmObject::Float(rval)) => $cvm.push(CvmObject::Bool((lval != 0) $opr (rval != 0.0))),
            (CvmObject::Uint(lval), CvmObject::Bool(rval)) => $cvm.push(CvmObject::Bool((lval != 0) $opr rval)),

            (CvmObject::Float(lval), CvmObject::Int(rval)) => $cvm.push(CvmObject::Bool((lval != 0.0) $opr (rval != 0))),
            (CvmObject::Float(lval), CvmObject::Uint(rval)) => $cvm.push(CvmObject::Bool((lval != 0.0) $opr (rval != 0))),
            (CvmObject::Float(lval), CvmObject::Float(rval)) => $cvm.push(CvmObject::Bool((lval != 0.0) $opr (rval != 0.0))),
            (CvmObject::Float(lval), CvmObject::Bool(rval)) => $cvm.push(CvmObject::Bool((lval != 0.0) $opr rval)),

            (CvmObject::Bool(lval), CvmObject::Int(rval)) => $cvm.push(CvmObject::Bool(lval $opr (rval != 0))),
            (CvmObject::Bool(lval), CvmObject::Uint(rval)) => $cvm.push(CvmObject::Bool(lval $opr (rval != 0))),
            (CvmObject::Bool(lval), CvmObject::Float(rval)) => $cvm.push(CvmObject::Bool(lval $opr (rval != 0.0))),
            (CvmObject::Bool(lval), CvmObject::Bool(rval)) => $cvm.push(CvmObject::Bool(lval $opr rval)),

            (left, right) => panic!(
                "unchecked invalid logic operation - {:?} and {:?}",
                left.as_type(),
                right.as_type()
            )
        }
        $current_idx
    }};
}

macro_rules! apply_comp_operator {
    ( $cvm:ident, $current_idx:ident, $opr:tt, $bool_opr_handler:ident ) => {{
        let (left, right) = get_operands($cvm);
        match (left, right) {
            (CvmObject::Int(lval), CvmObject::Int(rval)) => $cvm.push(CvmObject::Bool(lval $opr rval)),
            (CvmObject::Int(lval), CvmObject::Uint(rval)) => $cvm.push(CvmObject::Bool(lval $opr (rval as i64))),
            (CvmObject::Int(lval), CvmObject::Float(rval)) => $cvm.push(CvmObject::Bool(lval $opr (rval as i64))),
            (left @ CvmObject::Int(_), CvmObject::Bool(rval)) => $bool_opr_handler($cvm, rval, left),

            (CvmObject::Uint(lval), CvmObject::Int(rval)) => $cvm.push(CvmObject::Bool((lval as i64) $opr rval)),
            (CvmObject::Uint(lval), CvmObject::Uint(rval)) => $cvm.push(CvmObject::Bool(lval $opr rval)),
            (CvmObject::Uint(lval), CvmObject::Float(rval)) => $cvm.push(CvmObject::Bool(lval  $opr (rval as u64))),
            (left @ CvmObject::Uint(_), CvmObject::Bool(rval)) => $bool_opr_handler($cvm, rval, left),

            (CvmObject::Float(lval), CvmObject::Int(rval)) => $cvm.push(CvmObject::Bool((lval as i64) $opr rval)),
            (CvmObject::Float(lval), CvmObject::Uint(rval)) => $cvm.push(CvmObject::Bool((lval as u64) $opr rval)),
            (CvmObject::Float(lval), CvmObject::Float(rval)) => $cvm.push(CvmObject::Bool(lval $opr rval)),
            (left @ CvmObject::Float(_), CvmObject::Bool(rval)) => $bool_opr_handler($cvm, rval, left),

            (CvmObject::Str(lval), CvmObject::Str(rval)) => $cvm.push(CvmObject::Bool(lval $opr rval)),
            (CvmObject::Bool(lval), right) => $bool_opr_handler($cvm, lval, right),

            (left, right) => panic!(
                "unchecked invalid comparison operation - {:?} and {:?}",
                left.as_type(),
                right.as_type()
            )
        }
        $current_idx
    }};
}

pub fn and(cvm: &mut Cvm, current_idx: usize) -> usize {
    apply_logic_operator!(cvm, current_idx, &&)
}

pub fn or(cvm: &mut Cvm, current_idx: usize) -> usize {
    apply_logic_operator!(cvm, current_idx, ||)
}

fn cmp_bool(_: &mut Cvm, _: bool, right: CvmObject) {
    panic!(
        "unchecked invalid comparison operation - bool and {:?}",
        right.as_type()
    )
}

fn eq_bool(cvm: &mut Cvm, lval: bool, right: CvmObject) {
    match right {
        CvmObject::Int(rval) => cvm.push(CvmObject::Bool(lval == (rval == 0))),
        CvmObject::Uint(rval) => cvm.push(CvmObject::Bool(lval == (rval == 0))),
        CvmObject::Float(rval) => cvm.push(CvmObject::Bool(lval == (rval == 0.0))),
        CvmObject::Bool(rval) => cvm.push(CvmObject::Bool(lval == rval)),
        _ => panic!("unchecked invalid equality operation - comparing string == bool"),
    }
}

pub fn lt(cvm: &mut Cvm, current_idx: usize) -> usize {
    apply_comp_operator!(cvm, current_idx, <, cmp_bool)
}

pub fn lt_eq(cvm: &mut Cvm, current_idx: usize) -> usize {
    apply_comp_operator!(cvm, current_idx, <=, cmp_bool)
}

pub fn gt(cvm: &mut Cvm, current_idx: usize) -> usize {
    apply_comp_operator!(cvm, current_idx, >, cmp_bool)
}

pub fn gt_eq(cvm: &mut Cvm, current_idx: usize) -> usize {
    apply_comp_operator!(cvm, current_idx, >=, cmp_bool)
}

pub fn eq(cvm: &mut Cvm, current_idx: usize) -> usize {
    apply_comp_operator!(cvm, current_idx, ==, eq_bool)
}
