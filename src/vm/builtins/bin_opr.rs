use crate::vm::{CVMError, CVMObject, CVM};

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
            return Err(CVMError::ExpectedObject);
        };
        match left {
            CVMObject::Int(lval) => match right {
                CVMObject::Int(rval) => push_operation!($cvm, Int, lval $opr rval),
                CVMObject::Uint(rval) => push_operation!($cvm, Int, lval $opr (rval as i64)),
                CVMObject::Float(rval) => push_operation!($cvm, Float, (lval as f64) $opr rval),
                _ => return Err(CVMError::InvalidOperation),
            },

            CVMObject::Uint(lval) => match right {
                CVMObject::Int(rval) => push_operation!($cvm, Int, (lval as i64) $opr rval),
                CVMObject::Uint(rval) => push_operation!($cvm, Uint, lval $opr rval),
                CVMObject::Float(rval) => push_operation!($cvm, Float, (lval as f64) $opr rval),
                _ => return Err(CVMError::InvalidOperation),
            },

            CVMObject::Float(lval) => match right {
                CVMObject::Int(rval) => push_operation!($cvm, Float, lval $opr (rval as f64)),
                CVMObject::Uint(rval) => push_operation!($cvm, Float, lval $opr (rval as f64)),
                CVMObject::Float(rval) => push_operation!($cvm, Float, lval $opr rval),
                _ => return Err(CVMError::InvalidOperation),
            },

            CVMObject::Str(lval) => $str_opr_handler($cvm, lval, right)?,
            _ => return Err(CVMError::InvalidOperation)
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
        return Err(CVMError::ExpectedObject);
    };
    match left {
        CVMObject::Int(lval) => match right {
            CVMObject::Int(rval) => cvm.stack.push(CVMObject::Float(lval as f64 / rval as f64)),
            CVMObject::Uint(rval) => cvm.stack.push(CVMObject::Float(lval as f64 / rval as f64)),
            CVMObject::Float(rval) => cvm.stack.push(CVMObject::Float(lval as f64 / rval)),
            _ => return Err(CVMError::InvalidOperation),
        },

        CVMObject::Uint(lval) => match right {
            CVMObject::Int(rval) => cvm.stack.push(CVMObject::Float(lval as f64 / rval as f64)),
            CVMObject::Uint(rval) => cvm.stack.push(CVMObject::Float(lval as f64 / rval as f64)),
            CVMObject::Float(rval) => cvm.stack.push(CVMObject::Float(lval as f64 / rval)),
            _ => return Err(CVMError::InvalidOperation),
        },

        CVMObject::Float(lval) => match right {
            CVMObject::Int(rval) => cvm.stack.push(CVMObject::Float(lval / rval as f64)),
            CVMObject::Uint(rval) => cvm.stack.push(CVMObject::Float(lval / rval as f64)),
            CVMObject::Float(rval) => cvm.stack.push(CVMObject::Float(lval / rval)),
            _ => return Err(CVMError::InvalidOperation),
        },

        _ => return Err(CVMError::InvalidOperation),
    }
    Ok(current_idx)
}

pub fn modulo(cvm: &mut CVM, current_idx: usize) -> Result<usize, CVMError> {
    let Some((left, right)) = get_operands(cvm) else {
        return Err(CVMError::ExpectedObject);
    };
    match left {
        CVMObject::Int(lval) => match right {
            CVMObject::Int(rval) => cvm.stack.push(CVMObject::Int(lval % rval)),
            CVMObject::Uint(rval) => cvm.stack.push(CVMObject::Int(lval % rval as i64)),
            CVMObject::Float(rval) => cvm.stack.push(CVMObject::Float(lval as f64 % rval)),
            _ => return Err(CVMError::InvalidOperation),
        },

        CVMObject::Uint(lval) => match right {
            CVMObject::Int(rval) => cvm.stack.push(CVMObject::Int(lval as i64 % rval)),
            CVMObject::Uint(rval) => cvm.stack.push(CVMObject::Uint(lval % rval)),
            CVMObject::Float(rval) => cvm.stack.push(CVMObject::Float(lval as f64 % rval)),
            _ => return Err(CVMError::InvalidOperation),
        },

        CVMObject::Float(lval) => match right {
            CVMObject::Int(rval) => cvm.stack.push(CVMObject::Float(lval % rval as f64)),
            CVMObject::Uint(rval) => cvm.stack.push(CVMObject::Float(lval % rval as f64)),
            CVMObject::Float(rval) => cvm.stack.push(CVMObject::Float(lval % rval)),
            _ => return Err(CVMError::InvalidOperation),
        },

        _ => return Err(CVMError::InvalidOperation),
    }
    Ok(current_idx)
}

fn add_str(cvm: &mut CVM, lval: String, right: CVMObject) -> Result<(), CVMError> {
    match right {
        CVMObject::Int(rval) => cvm.stack.push(CVMObject::Str(lval + &rval.to_string())),
        CVMObject::Uint(rval) => cvm.stack.push(CVMObject::Str(lval + &rval.to_string())),
        CVMObject::Float(rval) => cvm.stack.push(CVMObject::Str(lval + &rval.to_string())),
        CVMObject::Str(rval) => cvm.stack.push(CVMObject::Str(lval + &rval)),
        CVMObject::Bool(rval) => cvm.stack.push(CVMObject::Str(lval + &rval.to_string())),
    }
    Ok(())
}

fn sub_str(_: &mut CVM, _: String, _: CVMObject) -> Result<(), CVMError> {
    Err(CVMError::InvalidOperation)
}

fn mul_str(cvm: &mut CVM, lval: String, right: CVMObject) -> Result<(), CVMError> {
    match right {
        CVMObject::Uint(rval) => cvm.stack.push(CVMObject::Str(lval.repeat(rval as usize))),
        _ => return Err(CVMError::InvalidOperation),
    }
    Ok(())
}

macro_rules! apply_logic_operator {
    ( $cvm:ident, $current_idx:ident, $opr:tt ) => {{
        let Some((left, right)) = get_operands($cvm) else {
            return Err(CVMError::ExpectedObject);
        };
        match left {
            CVMObject::Int(lval) => {
                /* basically convert to bool */
                let lval = lval != 0;
                match right {
                    CVMObject::Int(rval) => push_operation!($cvm, Bool, lval $opr (rval != 0)),
                    CVMObject::Uint(rval) => push_operation!($cvm, Bool, lval $opr (rval != 0)),
                    CVMObject::Float(rval) => push_operation!($cvm, Bool, lval $opr (rval != 0.0)),
                    CVMObject::Bool(rval) => push_operation!($cvm, Bool, lval $opr rval),
                    _ => return Err(CVMError::InvalidOperation),
            }},

            CVMObject::Uint(lval) => {
                let lval = lval != 0;
                match right {
                    CVMObject::Int(rval) => push_operation!($cvm, Bool, lval $opr (rval != 0)),
                    CVMObject::Uint(rval) => push_operation!($cvm, Bool, lval $opr (rval != 0)),
                    CVMObject::Float(rval) => push_operation!($cvm, Bool, lval $opr (rval != 0.0)),
                    CVMObject::Bool(rval) => push_operation!($cvm, Bool, lval $opr rval),
                    _ => return Err(CVMError::InvalidOperation),
            }},

            CVMObject::Float(lval) => {
                let lval = lval != 0.0;
                match right {
                    CVMObject::Int(rval) => push_operation!($cvm, Bool, lval $opr (rval != 0)),
                    CVMObject::Uint(rval) => push_operation!($cvm, Bool, lval $opr (rval != 0)),
                    CVMObject::Float(rval) => push_operation!($cvm, Bool, lval $opr (rval != 0.0)),
                    CVMObject::Bool(rval) => push_operation!($cvm, Bool, lval $opr rval),
                    _ => return Err(CVMError::InvalidOperation),
            }},

            CVMObject::Bool(lval) => match right {
                CVMObject::Int(rval) => push_operation!($cvm, Bool, lval $opr (rval != 0)),
                CVMObject::Uint(rval) => push_operation!($cvm, Bool, lval $opr (rval != 0)),
                CVMObject::Float(rval) => push_operation!($cvm, Bool, lval $opr (rval != 0.0)),
                CVMObject::Bool(rval) => push_operation!($cvm, Bool, lval $opr rval),
                _ => return Err(CVMError::InvalidOperation),
            }

            CVMObject::Str(_) =>  return Err(CVMError::InvalidOperation),
        }
        Ok($current_idx)
    }};
}

macro_rules! apply_comp_operator {
    ( $cvm:ident, $current_idx:ident, $opr:tt, $bool_opr_handler:ident ) => {{
        let Some((left, right)) = get_operands($cvm) else {
            return Err(CVMError::ExpectedObject);
        };
        match left {
            CVMObject::Int(lval) => match right {
                CVMObject::Int(rval) => push_operation!($cvm, Bool, lval $opr rval),
                CVMObject::Uint(rval) => push_operation!($cvm, Bool, lval $opr rval as i64),
                CVMObject::Float(rval) => push_operation!($cvm, Bool, lval $opr rval as i64),
                CVMObject::Bool(rval) => $bool_opr_handler($cvm, rval, left)?,
                _ => return Err(CVMError::InvalidOperation),
            },

            CVMObject::Uint(lval) => match right {
                CVMObject::Int(rval) => push_operation!($cvm, Bool, lval $opr rval as u64),
                CVMObject::Uint(rval) => push_operation!($cvm, Bool, lval $opr rval),
                CVMObject::Float(rval) => push_operation!($cvm, Bool, lval $opr rval as u64),
                CVMObject::Bool(rval) => $bool_opr_handler($cvm, rval, left)?,
                _ => return Err(CVMError::InvalidOperation),
            },

            CVMObject::Float(lval) => match right {
                CVMObject::Int(rval) => push_operation!($cvm, Bool, lval $opr rval as f64),
                CVMObject::Uint(rval) => push_operation!($cvm, Bool, lval $opr rval as f64),
                CVMObject::Float(rval) => push_operation!($cvm, Bool, lval $opr rval),
                CVMObject::Bool(rval) => $bool_opr_handler($cvm, rval, left)?,
                _ => return Err(CVMError::InvalidOperation),
            },

            CVMObject::Bool(lval) => $bool_opr_handler($cvm, lval, right)?,
            CVMObject::Str(_) =>  return Err(CVMError::InvalidOperation),
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

fn cmp_bool(_: &mut CVM, _: bool, _: CVMObject) -> Result<(), CVMError> {
    Err(CVMError::InvalidOperation)
}

fn eq_bool(cvm: &mut CVM, lval: bool, right: CVMObject) -> Result<(), CVMError> {
    match right {
        CVMObject::Int(rval) => cvm.stack.push(CVMObject::Bool(lval == (rval == 0))),
        CVMObject::Uint(rval) => cvm.stack.push(CVMObject::Bool(lval == (rval == 0))),
        CVMObject::Float(rval) => cvm.stack.push(CVMObject::Bool(lval == (rval == 0.0))),
        CVMObject::Bool(rval) => cvm.stack.push(CVMObject::Bool(lval == rval)),
        _ => return Err(CVMError::InvalidOperation),
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
