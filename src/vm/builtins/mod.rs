//! The module containing the built-in operations inside the `CVM` - mostly the
//! binary and unary operations between [`CvmObjects`].

pub mod bin_opr;
pub mod un_opr;

pub use bin_opr::{add, and, div, eq, gt, gt_eq, lt, lt_eq, modulo, mul, or, sub};
pub use un_opr::{neg, not};

use super::{Cvm, CvmObject};
use crate::error::assertion_fail;

#[inline(always)]
fn get_operands(cvm: &mut Cvm) -> (CvmObject, CvmObject) {
    let right = cvm.stack.pop().expect("expected an object on the stack");
    let left = cvm.stack.pop().expect("expected an object on the stack");
    (left, right)
}

pub fn assert(cvm: &mut Cvm, next_idx: usize) -> usize {
    let (exp, recv) = get_operands(cvm);
    if exp != recv {
        assertion_fail(format!("{:?}", exp), format!("{:?}", recv));
    }
    next_idx
}
