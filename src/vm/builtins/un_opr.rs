use crate::vm::{CVMObject, CVM};

pub fn neg(cvm: &mut CVM, current_idx: usize) -> usize {
    let operand = cvm.stack.pop().expect("expected an object on the stack");

    match operand {
        CVMObject::Int(val) => cvm.push(CVMObject::Int(-val)),
        CVMObject::Uint(val) => cvm.push(CVMObject::Int(-(val as i64))),
        CVMObject::Float(val) => cvm.push(CVMObject::Float(-val)),
        _ => panic!(
            "unchecked invalid unary negation on {:?}",
            operand.as_type()
        ),
    }

    current_idx
}

pub fn not(cvm: &mut CVM, current_idx: usize) -> usize {
    let operand = cvm.stack.pop().expect("expected an object on the stack");

    match operand {
        CVMObject::Int(val) => cvm.push(CVMObject::Bool(val == 0)),
        CVMObject::Uint(val) => cvm.push(CVMObject::Bool(val == 0)),
        CVMObject::Float(val) => cvm.push(CVMObject::Bool(val == 0.0)),
        CVMObject::Bool(val) => cvm.push(CVMObject::Bool(!val)),
        _ => panic!("unchecked invalid unary not on {:?}", operand.as_type()),
    }

    current_idx
}
