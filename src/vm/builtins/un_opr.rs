use crate::vm::{CVMError, CVMObject, CVM};

pub fn neg(cvm: &mut CVM, current_idx: usize) -> Result<usize, CVMError> {
    let Some(operand) = cvm.stack.top() else {
        return Err(CVMError::ExpectedObject);
    };

    match operand {
        CVMObject::Int(val) => *operand = CVMObject::Int(-*val),
        CVMObject::Uint(val) => *operand = CVMObject::Int(-(*val as i64)),
        CVMObject::Float(val) => *operand = CVMObject::Float(-*val),
        _ => return Err(CVMError::InvalidOperation),
    }

    Ok(current_idx)
}

pub fn not(cvm: &mut CVM, current_idx: usize) -> Result<usize, CVMError> {
    let Some(operand) = cvm.stack.pop() else {
        return Err(CVMError::ExpectedObject);
    };

    match operand {
        CVMObject::Int(val) => cvm.stack.push(CVMObject::Bool(val == 0)),
        CVMObject::Uint(val) => cvm.stack.push(CVMObject::Bool(val == 0)),
        CVMObject::Float(val) => cvm.stack.push(CVMObject::Bool(val == 0.0)),
        CVMObject::Str(_) => return Err(CVMError::InvalidOperation),
        CVMObject::Bool(val) => cvm.stack.push(CVMObject::Bool(!val)),
    }

    Ok(current_idx)
}
