use crate::utils::PtrString;
use crate::vm::{CVMObject, CVM};

#[inline(always)]
fn get_operands(cvm: &mut CVM) -> (CVMObject, CVMObject) {
    let right = cvm.stack.pop().expect("expected an object on the stack");
    let left = cvm.stack.pop().expect("expected an object on the stack");
    (left, right)
}

macro_rules! apply_bin_operator {
    ( $cvm:ident, $current_idx:ident, $opr:tt, $str_opr_handler:ident ) => {{
        let (left, right) = get_operands($cvm);
        match (left, right) {
            (CVMObject::Int(lval), CVMObject::Int(rval)) => $cvm.push(CVMObject::Int(lval $opr rval)),
            (CVMObject::Int(lval), CVMObject::Uint(rval)) => $cvm.push(CVMObject::Int(lval $opr (rval as i64))),
            (CVMObject::Int(lval), CVMObject::Float(rval)) => $cvm.push(CVMObject::Float((lval as f64) $opr rval)),

            (CVMObject::Uint(lval), CVMObject::Int(rval)) => $cvm.push(CVMObject::Int((lval as i64) $opr rval)),
            (CVMObject::Uint(lval), CVMObject::Uint(rval)) => $cvm.push(CVMObject::Uint(lval $opr rval)),
            (CVMObject::Uint(lval), CVMObject::Float(rval)) => $cvm.push(CVMObject::Float((lval as f64) $opr rval)),

            (CVMObject::Float(lval), CVMObject::Int(rval)) => $cvm.push(CVMObject::Float(lval $opr (rval as f64))),
            (CVMObject::Float(lval), CVMObject::Uint(rval)) => $cvm.push(CVMObject::Float(lval $opr (rval as f64))),
            (CVMObject::Float(lval), CVMObject::Float(rval)) => $cvm.push(CVMObject::Float(lval $opr rval)),

            (CVMObject::Str(lval), right @ _) => $str_opr_handler($cvm, lval.clone(), right),
            (left @ _, right @ _) => panic!(
                "unchecked invalid binary operation - {:?} and {:?}",
                left.as_type(),
                right.as_type()
            ),
        }
        $current_idx
    }};
}

pub fn add(cvm: &mut CVM, current_idx: usize) -> usize {
    fn add_str(cvm: &mut CVM, lval: PtrString, right: CVMObject) {
        match right {
            CVMObject::Int(rval) => cvm.push(CVMObject::Str(lval + rval.to_string().into())),
            CVMObject::Uint(rval) => cvm.push(CVMObject::Str(lval + rval.to_string().into())),
            CVMObject::Float(rval) => cvm.push(CVMObject::Str(lval + rval.to_string().into())),
            CVMObject::Str(rval) => cvm.push(CVMObject::Str(lval + rval)),
            CVMObject::Bool(rval) => cvm.push(CVMObject::Str(lval + rval.to_string().into())),
        }
    }
    apply_bin_operator!(cvm, current_idx, +, add_str)
}

pub fn sub(cvm: &mut CVM, current_idx: usize) -> usize {
    fn sub_str(_: &mut CVM, _: PtrString, _: CVMObject) {
        panic!("unchecked invalid string operation - string substitution")
    }
    apply_bin_operator!(cvm, current_idx, -, sub_str)
}

pub fn mul(cvm: &mut CVM, current_idx: usize) -> usize {
    fn mul_str(cvm: &mut CVM, lval: PtrString, right: CVMObject) {
        match right {
            CVMObject::Uint(rval) => cvm.stack.push(CVMObject::Str(lval * (rval as usize))),
            _ => panic!("unchecked invalid string operation - string multiplication with non-uint"),
        }
    }
    apply_bin_operator!(cvm, current_idx, *, mul_str)
}

pub fn div(cvm: &mut CVM, current_idx: usize) -> usize {
    let (left, right) = get_operands(cvm);
    match (left, right) {
        (CVMObject::Int(lval), CVMObject::Int(rval)) => {
            cvm.push(CVMObject::Float((lval as f64) / (rval as f64)))
        }
        (CVMObject::Int(lval), CVMObject::Uint(rval)) => {
            cvm.push(CVMObject::Float((lval as f64) / (rval as f64)))
        }
        (CVMObject::Int(lval), CVMObject::Float(rval)) => {
            cvm.push(CVMObject::Float((lval as f64) / rval))
        }

        (CVMObject::Uint(lval), CVMObject::Int(rval)) => {
            cvm.push(CVMObject::Float((lval as f64) / (rval as f64)))
        }
        (CVMObject::Uint(lval), CVMObject::Uint(rval)) => {
            cvm.push(CVMObject::Float((lval as f64) / (rval as f64)))
        }
        (CVMObject::Uint(lval), CVMObject::Float(rval)) => {
            cvm.push(CVMObject::Float((lval as f64) / rval))
        }

        (CVMObject::Float(lval), CVMObject::Int(rval)) => {
            cvm.push(CVMObject::Float(lval / (rval as f64)))
        }
        (CVMObject::Float(lval), CVMObject::Uint(rval)) => {
            cvm.push(CVMObject::Float(lval / (rval as f64)))
        }
        (CVMObject::Float(lval), CVMObject::Float(rval)) => cvm.push(CVMObject::Float(lval / rval)),

        (left @ _, right @ _) => panic!(
            "unchecked invalid division - {:?} and {:?}",
            left.as_type(),
            right.as_type()
        ),
    }
    current_idx
}

pub fn modulo(cvm: &mut CVM, current_idx: usize) -> usize {
    fn mod_str(_: &mut CVM, _: PtrString, _: CVMObject) {
        panic!("unchecked invalid string operation - string substitution")
    }
    apply_bin_operator!(cvm, current_idx, %, mod_str)
    /*
    let (left, right) = get_operands(cvm);
    match (left, right) {
        (CVMObject::Int(lval), CVMObject::Int(rval)) => cvm.push(CVMObject::Int(lval % rval)),
        (CVMObject::Int(lval), CVMObject::Uint(rval)) => {
            cvm.push(CVMObject::Int(lval % (rval as i64)))
        }
        (CVMObject::Int(lval), CVMObject::Float(rval)) => {
            cvm.push(CVMObject::Float((lval as f64) % rval))
        }

        (CVMObject::Uint(lval), CVMObject::Int(rval)) => {
            cvm.push(CVMObject::Int((lval as i64) % rval))
        }
        (CVMObject::Uint(lval), CVMObject::Uint(rval)) => cvm.push(CVMObject::Uint(lval % rval)),
        (CVMObject::Uint(lval), CVMObject::Float(rval)) => {
            cvm.push(CVMObject::Float((lval as f64) % rval))
        }

        (CVMObject::Float(lval), CVMObject::Int(rval)) => {
            cvm.push(CVMObject::Float(lval % (rval as f64)))
        }
        (CVMObject::Float(lval), CVMObject::Uint(rval)) => {
            cvm.push(CVMObject::Float(lval % (rval as f64)))
        }
        (CVMObject::Float(lval), CVMObject::Float(rval)) => cvm.push(CVMObject::Float(lval % rval)),

        (left @ _, right @ _) => panic!(
            "unchecked invalid modulo operation - {:?} and {:?}",
            left.as_type(),
            right.as_type()
        ),
    }
    current_idx
    */
}

macro_rules! apply_logic_operator {
    ( $cvm:ident, $current_idx:ident, $opr:tt ) => {{
        let (left, right) = get_operands($cvm);
        match (left, right) {
            (CVMObject::Int(lval), CVMObject::Int(rval)) => $cvm.push(CVMObject::Bool((lval != 0) $opr (rval != 0))),
            (CVMObject::Int(lval), CVMObject::Uint(rval)) => $cvm.push(CVMObject::Bool((lval != 0) $opr (rval != 0))),
            (CVMObject::Int(lval), CVMObject::Float(rval)) => $cvm.push(CVMObject::Bool((lval != 0) $opr (rval != 0.0))),
            (CVMObject::Int(lval), CVMObject::Bool(rval)) => $cvm.push(CVMObject::Bool((lval != 0) $opr rval)),

            (CVMObject::Uint(lval), CVMObject::Int(rval)) => $cvm.push(CVMObject::Bool((lval != 0) $opr (rval != 0))),
            (CVMObject::Uint(lval), CVMObject::Uint(rval)) => $cvm.push(CVMObject::Bool((lval != 0) $opr (rval != 0))),
            (CVMObject::Uint(lval), CVMObject::Float(rval)) => $cvm.push(CVMObject::Bool((lval != 0) $opr (rval != 0.0))),
            (CVMObject::Uint(lval), CVMObject::Bool(rval)) => $cvm.push(CVMObject::Bool((lval != 0) $opr rval)),

            (CVMObject::Float(lval), CVMObject::Int(rval)) => $cvm.push(CVMObject::Bool((lval != 0.0) $opr (rval != 0))),
            (CVMObject::Float(lval), CVMObject::Uint(rval)) => $cvm.push(CVMObject::Bool((lval != 0.0) $opr (rval != 0))),
            (CVMObject::Float(lval), CVMObject::Float(rval)) => $cvm.push(CVMObject::Bool((lval != 0.0) $opr (rval != 0.0))),
            (CVMObject::Float(lval), CVMObject::Bool(rval)) => $cvm.push(CVMObject::Bool((lval != 0.0) $opr rval)),

            (CVMObject::Bool(lval), CVMObject::Int(rval)) => $cvm.push(CVMObject::Bool(lval $opr (rval != 0))),
            (CVMObject::Bool(lval), CVMObject::Uint(rval)) => $cvm.push(CVMObject::Bool(lval $opr (rval != 0))),
            (CVMObject::Bool(lval), CVMObject::Float(rval)) => $cvm.push(CVMObject::Bool(lval $opr (rval != 0.0))),
            (CVMObject::Bool(lval), CVMObject::Bool(rval)) => $cvm.push(CVMObject::Bool(lval $opr rval)),

            (left @ _, right @ _) => panic!(
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
            (CVMObject::Int(lval), CVMObject::Int(rval)) => $cvm.push(CVMObject::Bool(lval $opr rval)),
            (CVMObject::Int(lval), CVMObject::Uint(rval)) => $cvm.push(CVMObject::Bool((lval as u64) $opr rval)),
            (CVMObject::Int(lval), CVMObject::Float(rval)) => $cvm.push(CVMObject::Bool((lval as f64) $opr rval)),
            (left @ CVMObject::Int(_), CVMObject::Bool(rval)) => $bool_opr_handler($cvm, rval, left),

            (CVMObject::Uint(lval), CVMObject::Int(rval)) => $cvm.push(CVMObject::Bool(lval $opr (rval as u64))),
            (CVMObject::Uint(lval), CVMObject::Uint(rval)) => $cvm.push(CVMObject::Bool(lval $opr rval)),
            (CVMObject::Uint(lval), CVMObject::Float(rval)) => $cvm.push(CVMObject::Bool((lval as f64) $opr rval)),
            (left @ CVMObject::Uint(_), CVMObject::Bool(rval)) => $bool_opr_handler($cvm, rval, left),

            (CVMObject::Float(lval), CVMObject::Int(rval)) => $cvm.push(CVMObject::Bool(lval $opr (rval as f64))),
            (CVMObject::Float(lval), CVMObject::Uint(rval)) => $cvm.push(CVMObject::Bool(lval $opr (rval as f64))),
            (CVMObject::Float(lval), CVMObject::Float(rval)) => $cvm.push(CVMObject::Bool(lval $opr rval)),
            (left @ CVMObject::Float(_), CVMObject::Bool(rval)) => $bool_opr_handler($cvm, rval, left),

            (CVMObject::Str(lval), CVMObject::Str(rval)) => $cvm.push(CVMObject::Bool(lval $opr rval)),
            (CVMObject::Bool(lval), right @ _) => $bool_opr_handler($cvm, lval, right),

            (left @ _, right @ _) => panic!(
                "unchecked invalid comparison operation - {:?} and {:?}",
                left.as_type(),
                right.as_type()
            )
        }
        $current_idx
    }};
}

pub fn and(cvm: &mut CVM, current_idx: usize) -> usize {
    apply_logic_operator!(cvm, current_idx, &&)
}

pub fn or(cvm: &mut CVM, current_idx: usize) -> usize {
    apply_logic_operator!(cvm, current_idx, ||)
}

fn cmp_bool(_: &mut CVM, _: bool, right: CVMObject) {
    panic!(
        "unchecked invalid comparison operation - bool and {:?}",
        right.as_type()
    )
}

fn eq_bool(cvm: &mut CVM, lval: bool, right: CVMObject) {
    match right {
        CVMObject::Int(rval) => cvm.push(CVMObject::Bool(lval == (rval == 0))),
        CVMObject::Uint(rval) => cvm.push(CVMObject::Bool(lval == (rval == 0))),
        CVMObject::Float(rval) => cvm.push(CVMObject::Bool(lval == (rval == 0.0))),
        CVMObject::Bool(rval) => cvm.push(CVMObject::Bool(lval == rval)),
        _ => panic!("unchecked invalid equality operation - comparing string == bool"),
    }
}

pub fn lt(cvm: &mut CVM, current_idx: usize) -> usize {
    apply_comp_operator!(cvm, current_idx, <, cmp_bool)
}

pub fn lt_eq(cvm: &mut CVM, current_idx: usize) -> usize {
    apply_comp_operator!(cvm, current_idx, <=, cmp_bool)
}

pub fn gt(cvm: &mut CVM, current_idx: usize) -> usize {
    apply_comp_operator!(cvm, current_idx, >, cmp_bool)
}

pub fn gt_eq(cvm: &mut CVM, current_idx: usize) -> usize {
    apply_comp_operator!(cvm, current_idx, >=, cmp_bool)
}

pub fn eq(cvm: &mut CVM, current_idx: usize) -> usize {
    apply_comp_operator!(cvm, current_idx, ==, eq_bool)
}
