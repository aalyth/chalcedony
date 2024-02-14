use crate::vm::{Cvm, CvmObject};

pub fn neg(cvm: &mut Cvm, current_idx: usize) -> usize {
    let operand = cvm.stack.pop().expect("expected an object on the stack");

    match operand {
        CvmObject::Int(val) => cvm.push(CvmObject::Int(-val)),
        CvmObject::Uint(val) => cvm.push(CvmObject::Int(-(val as i64))),
        CvmObject::Float(val) => cvm.push(CvmObject::Float(-val)),
        _ => panic!(
            "unchecked invalid unary negation on {:?}",
            operand.as_type()
        ),
    }

    current_idx
}

pub fn not(cvm: &mut Cvm, current_idx: usize) -> usize {
    let operand = cvm.stack.pop().expect("expected an object on the stack");

    match operand {
        CvmObject::Int(val) => cvm.push(CvmObject::Bool(val == 0)),
        CvmObject::Uint(val) => cvm.push(CvmObject::Bool(val == 0)),
        CvmObject::Float(val) => cvm.push(CvmObject::Bool(val == 0.0)),
        CvmObject::Bool(val) => cvm.push(CvmObject::Bool(!val)),
        _ => panic!("unchecked invalid unary not on {:?}", operand.as_type()),
    }

    current_idx
}
