use crate::vm::{CVMError, CVMErrorKind, CVMObject, CVM};

pub fn neg(cvm: &mut CVM, current_idx: usize) -> Result<usize, CVMError> {
    let Some(operand) = cvm.stack.pop() else {
        return Err(cvm.error(CVMErrorKind::ExpectedObject));
    };

    match operand {
        CVMObject::Int(val) => cvm.stack.push(CVMObject::Int(-val)),
        CVMObject::Uint(val) => cvm.stack.push(CVMObject::Int(-(val as i64))),
        CVMObject::Float(val) => cvm.stack.push(CVMObject::Float(-val)),
        _ => return Err(cvm.error(CVMErrorKind::InvalidUnOperation(operand.as_type()))),
    }

    Ok(current_idx)
}

pub fn not(cvm: &mut CVM, current_idx: usize) -> Result<usize, CVMError> {
    let Some(operand) = cvm.stack.pop() else {
        return Err(cvm.error(CVMErrorKind::ExpectedObject));
    };

    match operand {
        CVMObject::Int(val) => cvm.stack.push(CVMObject::Bool(val == 0)),
        CVMObject::Uint(val) => cvm.stack.push(CVMObject::Bool(val == 0)),
        CVMObject::Float(val) => cvm.stack.push(CVMObject::Bool(val == 0.0)),
        CVMObject::Bool(val) => cvm.stack.push(CVMObject::Bool(!val)),
        _ => return Err(cvm.error(CVMErrorKind::InvalidUnOperation(operand.as_type()))),
    }

    Ok(current_idx)
}
