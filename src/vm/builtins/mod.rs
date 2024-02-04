pub mod bin_opr;
pub mod un_opr;

pub use bin_opr::{add, and, div, eq, gt, gt_eq, lt, lt_eq, modulo, mul, or, sub};
pub use un_opr::{neg, not};

use super::{CVMObject, CVM};
use crate::error::assertion_fail;

#[inline(always)]
fn get_operands(cvm: &mut CVM) -> (CVMObject, CVMObject) {
    let right = cvm.stack.pop().expect("expected an object on the stack");
    let left = cvm.stack.pop().expect("expected an object on the stack");
    (left, right)
}

pub fn assert(cvm: &mut CVM, next_idx: usize) -> usize {
    let (exp, recv) = get_operands(cvm);
    if exp != recv {
        assertion_fail(format!("{:?}", exp), format!("{:?}", recv));
    }
    next_idx
}
