use crate::vm::{Cvm, CvmObject};

use super::get_operands;

pub fn list_insert(cvm: &mut Cvm, list_idx: isize, current_idx: usize) -> usize {
    println!("STACK: {:#?}\n", cvm.stack);
    let (list_obj, el) = get_operands(cvm);
    let CvmObject::List(list) = list_obj else {
        panic!("inserting into a non-list object");
    };

    if list_idx < 0 || list_idx > list.borrow().len() as isize {
        list.borrow_mut().push_back(el);
    } else {
        list.borrow_mut().insert(list_idx as usize, el);
    }

    cvm.stack.push(CvmObject::List(list));

    current_idx
}
