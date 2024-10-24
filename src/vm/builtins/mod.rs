//! The module containing the built-in operations inside the `CVM` - mostly the
//! binary and unary operations between [`CvmObjects`].

pub mod bin_opr;
pub mod list;
pub mod un_opr;

use super::{Cvm, CvmObject};

pub fn print(cvm: &mut Cvm, next_idx: usize) -> usize {
    let obj = cvm.stack.pop().unwrap();
    println!("{}", obj);
    next_idx
}

pub fn assert(cvm: &mut Cvm, next_idx: usize) -> usize {
    let CvmObject::Bool(successful) = cvm.stack.pop().unwrap() else {
        panic!("assert typing failed")
    };

    if !successful {
        cvm.stack
            .push(CvmObject::Exception("assertion failed".to_string().into()));
        return cvm.handle_exception();
    }

    next_idx
}

fn get_operands(cvm: &mut Cvm) -> (CvmObject, CvmObject) {
    let right = cvm.stack.pop().expect("expected an object on the stack");
    let left = cvm.stack.pop().expect("expected an object on the stack");
    (left, right)
}
