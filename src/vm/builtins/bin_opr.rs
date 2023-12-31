use crate::lexer::Type;
use crate::utils::PtrString;
use crate::vm::{CVMError, CVMErrorKind, CVMObject, CVM};

fn get_operands(cvm: &mut CVM) -> Option<(CVMObject, CVMObject)> {
    let right = cvm.stack.pop()?;
    let left = cvm.stack.pop()?;
    Some((left, right))
}

macro_rules! push_operation {
    ( $cvm:ident, $obj_type:ident, $val:expr ) => {
        $cvm.stack.push(CVMObject::$obj_type($val))
    };
}

macro_rules! apply_bin_operator {
    ( $cvm:ident, $current_idx:ident, $opr:tt, $str_opr_handler:ident ) => {{
        let Some((left, right)) = get_operands($cvm) else {
            return Err($cvm.error(CVMErrorKind::ExpectedObject));
        };
        match (&left, &right) {
            (CVMObject::Int(lval), CVMObject::Int(rval)) => push_operation!($cvm, Int, *lval $opr *rval),
            (CVMObject::Int(lval), CVMObject::Uint(rval)) => push_operation!($cvm, Int, *lval $opr (*rval as i64)),
            (CVMObject::Int(lval), CVMObject::Float(rval)) => push_operation!($cvm, Float, (*lval as f64) $opr *rval),

            (CVMObject::Uint(lval), CVMObject::Int(rval)) => push_operation!($cvm, Int, (*lval as i64) $opr *rval),
            (CVMObject::Uint(lval), CVMObject::Uint(rval)) => push_operation!($cvm, Uint, *lval $opr *rval),
            (CVMObject::Uint(lval), CVMObject::Float(rval)) => push_operation!($cvm, Float, (*lval as f64) $opr *rval),

            (CVMObject::Float(lval), CVMObject::Int(rval)) => push_operation!($cvm, Float, *lval $opr (*rval as f64)),
            (CVMObject::Float(lval), CVMObject::Uint(rval)) => push_operation!($cvm, Float, *lval $opr (*rval as f64)),
            (CVMObject::Float(lval), CVMObject::Float(rval)) => push_operation!($cvm, Float, *lval $opr *rval),

            // (CVMObject::Str(lval), _) => $str_opr_handler($cvm, lval.to_string(), right)?,
            _ => return Err($cvm.error(CVMErrorKind::InvalidBinOperation(left.as_type(), right.as_type())))
        }
        Ok($current_idx)
    }};
}

pub fn add(cvm: &mut CVM, current_idx: usize) -> Result<usize, CVMError> {
    apply_bin_operator!(cvm, current_idx, +, add_str)
}

pub fn sub(cvm: &mut CVM, current_idx: usize) -> Result<usize, CVMError> {
    apply_bin_operator!(cvm, current_idx, -, sub_str)
}

pub fn mul(cvm: &mut CVM, current_idx: usize) -> Result<usize, CVMError> {
    apply_bin_operator!(cvm, current_idx, *, mul_str)
}

pub fn div(cvm: &mut CVM, current_idx: usize) -> Result<usize, CVMError> {
    let Some((left, right)) = get_operands(cvm) else {
        return Err(cvm.error(CVMErrorKind::ExpectedObject));
    };
    match (&left, &right) {
        (CVMObject::Int(lval), CVMObject::Int(rval)) => push_operation!(cvm, Float, (*lval as f64) / (*rval as f64)),
        (CVMObject::Int(lval), CVMObject::Uint(rval)) => push_operation!(cvm, Float, (*lval as f64) / (*rval as f64)),
        (CVMObject::Int(lval), CVMObject::Float(rval)) => push_operation!(cvm, Float, (*lval as f64) / *rval),

        (CVMObject::Uint(lval), CVMObject::Int(rval)) => push_operation!(cvm, Float, (*lval as f64) / (*rval as f64)),
        (CVMObject::Uint(lval), CVMObject::Uint(rval)) => push_operation!(cvm, Float, (*lval as f64) / (*rval as f64)),
        (CVMObject::Uint(lval), CVMObject::Float(rval)) => push_operation!(cvm, Float, (*lval as f64) / *rval),

        (CVMObject::Float(lval), CVMObject::Int(rval)) => push_operation!(cvm, Float, *lval / (*rval as f64)),
        (CVMObject::Float(lval), CVMObject::Uint(rval)) => push_operation!(cvm, Float, *lval / (*rval as f64)),
        (CVMObject::Float(lval), CVMObject::Float(rval)) => push_operation!(cvm, Float, *lval / *rval),

        _ => return Err(cvm.error(CVMErrorKind::InvalidBinOperation(left.as_type(), right.as_type())))
    }
    Ok(current_idx)
}

pub fn modulo(cvm: &mut CVM, current_idx: usize) -> Result<usize, CVMError> {
    let Some((left, right)) = get_operands(cvm) else {
        return Err(cvm.error(CVMErrorKind::ExpectedObject));
    };
    match (&left, &right) {
        (CVMObject::Int(lval), CVMObject::Int(rval)) => push_operation!(cvm, Int, *lval % *rval),
        (CVMObject::Int(lval), CVMObject::Uint(rval)) => push_operation!(cvm, Int, *lval % (*rval as i64)),
        (CVMObject::Int(lval), CVMObject::Float(rval)) => push_operation!(cvm, Float, (*lval as f64) % *rval),

        (CVMObject::Uint(lval), CVMObject::Int(rval)) => push_operation!(cvm, Int, (*lval as i64) % *rval),
        (CVMObject::Uint(lval), CVMObject::Uint(rval)) => push_operation!(cvm, Uint, *lval % *rval),
        (CVMObject::Uint(lval), CVMObject::Float(rval)) => push_operation!(cvm, Float, (*lval as f64) % *rval),

        (CVMObject::Float(lval), CVMObject::Int(rval)) => push_operation!(cvm, Float, *lval % (*rval as f64)),
        (CVMObject::Float(lval), CVMObject::Uint(rval)) => push_operation!(cvm, Float, *lval % (*rval as f64)),
        (CVMObject::Float(lval), CVMObject::Float(rval)) => push_operation!(cvm, Float, *lval % *rval),

        _ => return Err(cvm.error(CVMErrorKind::InvalidBinOperation(left.as_type(), right.as_type())))
    }
    Ok(current_idx)
}

fn add_str(cvm: &mut CVM, lval: PtrString, right: CVMObject) -> Result<(), CVMError> {
    /*
    match right {
        CVMObject::Int(rval) => cvm.stack.push(CVMObject::Str(lval + rval.to_string())),
        CVMObject::Uint(rval) => cvm.stack.push(CVMObject::Str(lval + rval.to_string())),
        CVMObject::Float(rval) => cvm.stack.push(CVMObject::Str(lval + rval.to_string())),
        CVMObject::Str(rval) => cvm.stack.push(CVMObject::Str(lval + rval)),
        CVMObject::Bool(rval) => cvm.stack.push(CVMObject::Str(lval + rval)),
    }
    */
    Ok(())
}

fn sub_str(cvm: &mut CVM, _: String, right: CVMObject) -> Result<(), CVMError> {
    Err(cvm.error(CVMErrorKind::InvalidBinOperation(
        Type::Str,
        right.as_type(),
    )))
}

fn mul_str(cvm: &mut CVM, lval: String, right: CVMObject) -> Result<(), CVMError> {
    /*
    match right {
        CVMObject::Uint(rval) => cvm.stack.push(CVMObject::Str(lval.repeat(rval as usize))),
        _ => {
            return Err(cvm.error(CVMErrorKind::InvalidBinOperation(
                Type::Str,
                right.as_type(),
            )))
        }
    }
    */
    Ok(())
}

macro_rules! apply_logic_operator {
    ( $cvm:ident, $current_idx:ident, $opr:tt ) => {{
        let Some((left, right)) = get_operands($cvm) else {
            return Err($cvm.error(CVMErrorKind::ExpectedObject));
        };
        match (&left, &right) {
            (CVMObject::Int(lval), CVMObject::Int(rval)) => push_operation!($cvm, Bool, (*lval != 0) $opr (*rval != 0)),
            (CVMObject::Int(lval), CVMObject::Uint(rval)) => push_operation!($cvm, Bool, (*lval != 0) $opr (*rval != 0)),
            (CVMObject::Int(lval), CVMObject::Float(rval)) => push_operation!($cvm, Bool, (*lval != 0) $opr (*rval != 0.0)),
            (CVMObject::Int(lval), CVMObject::Bool(rval)) => push_operation!($cvm, Bool, (*lval != 0) $opr *rval),

            (CVMObject::Uint(lval), CVMObject::Int(rval)) => push_operation!($cvm, Bool, (*lval != 0) $opr (*rval != 0)),
            (CVMObject::Uint(lval), CVMObject::Uint(rval)) => push_operation!($cvm, Bool, (*lval != 0) $opr (*rval != 0)),
            (CVMObject::Uint(lval), CVMObject::Float(rval)) => push_operation!($cvm, Bool, (*lval != 0) $opr (*rval != 0.0)),
            (CVMObject::Uint(lval), CVMObject::Bool(rval)) => push_operation!($cvm, Bool, (*lval != 0) $opr *rval),

            (CVMObject::Float(lval), CVMObject::Int(rval)) => push_operation!($cvm, Bool, (*lval != 0.0) $opr (*rval != 0)),
            (CVMObject::Float(lval), CVMObject::Uint(rval)) => push_operation!($cvm, Bool, (*lval != 0.0) $opr (*rval != 0)),
            (CVMObject::Float(lval), CVMObject::Float(rval)) => push_operation!($cvm, Bool, (*lval != 0.0) $opr (*rval != 0.0)),
            (CVMObject::Float(lval), CVMObject::Bool(rval)) => push_operation!($cvm, Bool, (*lval != 0.0) $opr *rval),

            (CVMObject::Bool(lval), CVMObject::Int(rval)) => push_operation!($cvm, Bool, *lval $opr (*rval != 0)),
            (CVMObject::Bool(lval), CVMObject::Uint(rval)) => push_operation!($cvm, Bool, *lval $opr (*rval != 0)),
            (CVMObject::Bool(lval), CVMObject::Float(rval)) => push_operation!($cvm, Bool, *lval $opr (*rval != 0.0)),
            (CVMObject::Bool(lval), CVMObject::Bool(rval)) => push_operation!($cvm, Bool, *lval $opr *rval),

            _ =>  return Err($cvm.error(CVMErrorKind::InvalidBinOperation(left.as_type(), right.as_type()))),
        }
        Ok($current_idx)
    }};
}

macro_rules! apply_comp_operator {
    ( $cvm:ident, $current_idx:ident, $opr:tt, $bool_opr_handler:ident ) => {{
        let Some((left, right)) = get_operands($cvm) else {
            return Err($cvm.error(CVMErrorKind::ExpectedObject));
        };
        match (&left, &right) {
            (CVMObject::Int(lval), CVMObject::Int(rval)) => push_operation!($cvm, Bool, *lval $opr *rval),
            (CVMObject::Int(lval), CVMObject::Uint(rval)) => push_operation!($cvm, Bool, (*lval as u64) $opr *rval),
            (CVMObject::Int(lval), CVMObject::Float(rval)) => push_operation!($cvm, Bool, (*lval as f64) $opr *rval),
            (CVMObject::Int(_), CVMObject::Bool(rval)) => $bool_opr_handler($cvm, *rval, left)?,

            (CVMObject::Uint(lval), CVMObject::Int(rval)) => push_operation!($cvm, Bool, *lval $opr (*rval as u64)),
            (CVMObject::Uint(lval), CVMObject::Uint(rval)) => push_operation!($cvm, Bool, *lval $opr *rval),
            (CVMObject::Uint(lval), CVMObject::Float(rval)) => push_operation!($cvm, Bool, (*lval as f64) $opr *rval),
            (CVMObject::Uint(_), CVMObject::Bool(rval)) => $bool_opr_handler($cvm, *rval, left)?,

            (CVMObject::Float(lval), CVMObject::Int(rval)) => push_operation!($cvm, Bool, *lval $opr (*rval as f64)),
            (CVMObject::Float(lval), CVMObject::Uint(rval)) => push_operation!($cvm, Bool, *lval $opr (*rval as f64)),
            (CVMObject::Float(lval), CVMObject::Float(rval)) => push_operation!($cvm, Bool, *lval $opr *rval),
            (CVMObject::Float(_), CVMObject::Bool(rval)) => $bool_opr_handler($cvm, *rval, left)?,

            // (CVMObject::Str(lval), CVMObject::Str(rval)) => push_operation!($cvm, Bool, lval $opr rval),
            (CVMObject::Bool(lval), _) => $bool_opr_handler($cvm, *lval, right)?,

            _ => return Err($cvm.error(CVMErrorKind::InvalidBinOperation(left.as_type(), right.as_type()))),
        }
        Ok($current_idx)
    }};
}

pub fn and(cvm: &mut CVM, current_idx: usize) -> Result<usize, CVMError> {
    apply_logic_operator!(cvm, current_idx, &&)
}

pub fn or(cvm: &mut CVM, current_idx: usize) -> Result<usize, CVMError> {
    apply_logic_operator!(cvm, current_idx, ||)
}

fn cmp_bool(cvm: &mut CVM, _: bool, right: CVMObject) -> Result<(), CVMError> {
    Err(cvm.error(CVMErrorKind::InvalidBinOperation(
        Type::Bool,
        right.as_type(),
    )))
}

fn eq_bool(cvm: &mut CVM, lval: bool, right: CVMObject) -> Result<(), CVMError> {
    match right {
        CVMObject::Int(rval) => cvm.stack.push(CVMObject::Bool(lval == (rval == 0))),
        CVMObject::Uint(rval) => cvm.stack.push(CVMObject::Bool(lval == (rval == 0))),
        CVMObject::Float(rval) => cvm.stack.push(CVMObject::Bool(lval == (rval == 0.0))),
        CVMObject::Bool(rval) => cvm.stack.push(CVMObject::Bool(lval == rval)),
        _ => {
            return Err(cvm.error(CVMErrorKind::InvalidBinOperation(
                Type::Bool,
                right.as_type(),
            )))
        }
    }
    Ok(())
}

pub fn lt(cvm: &mut CVM, current_idx: usize) -> Result<usize, CVMError> {
    apply_comp_operator!(cvm, current_idx, <, cmp_bool)
}

pub fn lt_eq(cvm: &mut CVM, current_idx: usize) -> Result<usize, CVMError> {
    apply_comp_operator!(cvm, current_idx, <=, cmp_bool)
}

pub fn gt(cvm: &mut CVM, current_idx: usize) -> Result<usize, CVMError> {
    apply_comp_operator!(cvm, current_idx, >, cmp_bool)
}

pub fn gt_eq(cvm: &mut CVM, current_idx: usize) -> Result<usize, CVMError> {
    apply_comp_operator!(cvm, current_idx, >=, cmp_bool)
}

pub fn eq(cvm: &mut CVM, current_idx: usize) -> Result<usize, CVMError> {
    apply_comp_operator!(cvm, current_idx, ==, eq_bool)
}
