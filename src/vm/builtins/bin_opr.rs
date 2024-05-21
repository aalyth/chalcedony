use crate::utils::PtrString;
use crate::vm::{Cvm, CvmList, CvmObject};

use super::get_operands;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

macro_rules! apply_bin_operator {
    ( $cvm:ident, $current_idx:ident, $opr:tt,
      $str_opr_handler:ident, $list_opr_handler:ident)
    => {{
        let (left, right) = get_operands($cvm);
        match (left, right) {
            (CvmObject::Int(lval), CvmObject::Int(rval))
                => $cvm.push(CvmObject::Int(lval $opr rval)),
            (CvmObject::Int(lval), CvmObject::Uint(rval))
                => $cvm.push(CvmObject::Int(lval $opr (rval as i64))),
            (CvmObject::Int(lval), CvmObject::Float(rval))
                => $cvm.push(CvmObject::Int(lval $opr (rval as i64))),

            (CvmObject::Uint(lval), CvmObject::Int(rval))
                => $cvm.push(CvmObject::Int((lval as i64) $opr rval)),
            (CvmObject::Uint(lval), CvmObject::Uint(rval))
                => $cvm.push(CvmObject::Uint(lval $opr rval)),
            (CvmObject::Uint(lval), CvmObject::Float(rval))
                => $cvm.push(CvmObject::Int((lval as i64) $opr (rval as i64))),

            (CvmObject::Float(lval), CvmObject::Int(rval))
                => $cvm.push(CvmObject::Float(lval $opr (rval as f64))),
            (CvmObject::Float(lval), CvmObject::Uint(rval))
                => $cvm.push(CvmObject::Float(lval $opr (rval as f64))),
            (CvmObject::Float(lval), CvmObject::Float(rval))
                => $cvm.push(CvmObject::Float(lval $opr rval)),

            (CvmObject::Str(lval), right)
                => $str_opr_handler($cvm, lval.clone(), right),

            (CvmObject::List(lval), right)
                => $list_opr_handler($cvm, lval.clone(), right),

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
    fn add_str(cvm: &mut Cvm, lval: PtrString, rval: CvmObject) {
        cvm.push(CvmObject::Str(format!("{}{}", lval, rval).into()))
    }
    fn add_list(cvm: &mut Cvm, list: CvmList, rval: CvmObject) {
        match rval {
            CvmObject::List(rhs_list) => {
                for el in rhs_list.borrow().clone().into_iter() {
                    list.borrow_mut().push_back(el.deep_copy());
                }
                cvm.push(CvmObject::List(list));
            }
            rval => {
                list.borrow_mut().push_back(rval);
                cvm.push(CvmObject::List(list))
            }
        }
    }
    apply_bin_operator!(cvm, current_idx, +, add_str, add_list)
}

pub fn sub(cvm: &mut Cvm, current_idx: usize) -> usize {
    fn sub_str(_: &mut Cvm, _: PtrString, _: CvmObject) {
        panic!("unchecked invalid string operation - subtraction")
    }
    fn sub_list(_: &mut Cvm, _: CvmList, _: CvmObject) {
        panic!("unchecked invalid list operation - subtraction")
    }
    apply_bin_operator!(cvm, current_idx, -, sub_str, sub_list)
}

pub fn mul(cvm: &mut Cvm, current_idx: usize) -> usize {
    fn mul_str(cvm: &mut Cvm, lval: PtrString, right: CvmObject) {
        match right {
            CvmObject::Uint(rval) => cvm.stack.push(CvmObject::Str(lval * (rval as usize))),
            _ => panic!("unchecked invalid string operation - multiplication with non-uint"),
        }
    }
    fn mul_list(cvm: &mut Cvm, list: CvmList, right: CvmObject) {
        match right {
            CvmObject::Uint(rval) => {
                if rval == 0 {
                    cvm.push(CvmObject::List(Rc::new(RefCell::new(VecDeque::new()))));
                    return;
                }
                let CvmObject::List(iter) = CvmObject::List(list.clone()).deep_copy() else {
                    unreachable!();
                };
                for _ in 0..(rval - 1) {
                    for el in iter.borrow().clone().into_iter() {
                        list.borrow_mut().push_back(el.deep_copy());
                    }
                }
                cvm.stack.push(CvmObject::List(list));
            }
            _ => panic!("unchecked invalid list operation - list multiplication with non-uint"),
        }
    }
    apply_bin_operator!(cvm, current_idx, *, mul_str, mul_list)
}

pub fn div(cvm: &mut Cvm, current_idx: usize) -> usize {
    fn div_str(_: &mut Cvm, _: PtrString, _: CvmObject) {
        panic!("unchecked invalid string operation - division")
    }
    fn div_list(_: &mut Cvm, _: CvmList, _: CvmObject) {
        panic!("unchecked invalid list operation - division")
    }
    apply_bin_operator!(cvm, current_idx, /, div_str, div_list)
}

pub fn modulo(cvm: &mut Cvm, current_idx: usize) -> usize {
    fn mod_str(_: &mut Cvm, _: PtrString, _: CvmObject) {
        panic!("unchecked invalid string operation - modulo ")
    }
    fn mod_list(_: &mut Cvm, _: CvmList, _: CvmObject) {
        panic!("unchecked invalid list operation - modulo ")
    }
    apply_bin_operator!(cvm, current_idx, %, mod_str, mod_list)
}

macro_rules! apply_logic_operator {
    ( $cvm:ident, $current_idx:ident, $opr:tt ) => {{
        let (left, right) = get_operands($cvm);
        match (left, right) {
            (CvmObject::Int(lval), CvmObject::Int(rval))
                => $cvm.push(CvmObject::Bool((lval != 0) $opr (rval != 0))),
            (CvmObject::Int(lval), CvmObject::Uint(rval))
                => $cvm.push(CvmObject::Bool((lval != 0) $opr (rval != 0))),
            (CvmObject::Int(lval), CvmObject::Float(rval))
                => $cvm.push(CvmObject::Bool((lval != 0) $opr (rval != 0.0))),
            (CvmObject::Int(lval), CvmObject::Bool(rval))
                => $cvm.push(CvmObject::Bool((lval != 0) $opr rval)),

            (CvmObject::Uint(lval), CvmObject::Int(rval))
                => $cvm.push(CvmObject::Bool((lval != 0) $opr (rval != 0))),
            (CvmObject::Uint(lval), CvmObject::Uint(rval))
                => $cvm.push(CvmObject::Bool((lval != 0) $opr (rval != 0))),
            (CvmObject::Uint(lval), CvmObject::Float(rval))
                => $cvm.push(CvmObject::Bool((lval != 0) $opr (rval != 0.0))),
            (CvmObject::Uint(lval), CvmObject::Bool(rval))
                => $cvm.push(CvmObject::Bool((lval != 0) $opr rval)),

            (CvmObject::Float(lval), CvmObject::Int(rval))
                => $cvm.push(CvmObject::Bool((lval != 0.0) $opr (rval != 0))),
            (CvmObject::Float(lval), CvmObject::Uint(rval))
                => $cvm.push(CvmObject::Bool((lval != 0.0) $opr (rval != 0))),
            (CvmObject::Float(lval), CvmObject::Float(rval))
                => $cvm.push(CvmObject::Bool((lval != 0.0) $opr (rval != 0.0))),
            (CvmObject::Float(lval), CvmObject::Bool(rval))
                => $cvm.push(CvmObject::Bool((lval != 0.0) $opr rval)),

            (CvmObject::Bool(lval), CvmObject::Int(rval))
                => $cvm.push(CvmObject::Bool(lval $opr (rval != 0))),
            (CvmObject::Bool(lval), CvmObject::Uint(rval))
                => $cvm.push(CvmObject::Bool(lval $opr (rval != 0))),
            (CvmObject::Bool(lval), CvmObject::Float(rval))
                => $cvm.push(CvmObject::Bool(lval $opr (rval != 0.0))),
            (CvmObject::Bool(lval), CvmObject::Bool(rval))
                => $cvm.push(CvmObject::Bool(lval $opr rval)),

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
            (CvmObject::Int(lval), CvmObject::Int(rval))
                => $cvm.push(CvmObject::Bool(lval $opr rval)),
            (CvmObject::Int(lval), CvmObject::Uint(rval))
                => $cvm.push(CvmObject::Bool(lval $opr (rval as i64))),
            (CvmObject::Int(lval), CvmObject::Float(rval))
                => $cvm.push(CvmObject::Bool((lval as f64) $opr rval)),

            (left @ CvmObject::Int(_), CvmObject::Bool(rval))
                => $bool_opr_handler($cvm, rval, left),

            (CvmObject::Uint(lval), CvmObject::Int(rval))
                => $cvm.push(CvmObject::Bool((lval as i64) $opr rval)),
            (CvmObject::Uint(lval), CvmObject::Uint(rval))
                => $cvm.push(CvmObject::Bool(lval $opr rval)),
            (CvmObject::Uint(lval), CvmObject::Float(rval))
                => $cvm.push(CvmObject::Bool((lval as f64) $opr rval)),

            (left @ CvmObject::Uint(_), CvmObject::Bool(rval))
                => $bool_opr_handler($cvm, rval, left),

            (CvmObject::Float(lval), CvmObject::Int(rval))
                => $cvm.push(CvmObject::Bool(lval $opr (rval as f64))),
            (CvmObject::Float(lval), CvmObject::Uint(rval))
                => $cvm.push(CvmObject::Bool(lval $opr (rval as f64))),
            (CvmObject::Float(lval), CvmObject::Float(rval))
                => $cvm.push(CvmObject::Bool(lval $opr rval)),

            (left @ CvmObject::Float(_), CvmObject::Bool(rval))
                => $bool_opr_handler($cvm, rval, left),

            (CvmObject::Str(lval), CvmObject::Str(rval))
                => $cvm.push(CvmObject::Bool(lval $opr rval)),
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
