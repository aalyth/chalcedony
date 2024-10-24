use super::{Cvm, CvmObject};

use crate::utils::PtrString;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn list_create(cvm: &mut Cvm, len: usize, next_idx: usize) -> usize {
    let mut list = VecDeque::<CvmObject>::with_capacity(len);
    for _ in 0..len {
        list.push_front(cvm.stack.pop().expect("expected a value on the stack"));
    }
    cvm.stack.push(CvmObject::List(Rc::new(RefCell::new(list))));
    next_idx
}

/* returns the index of the next instruction */
pub fn list_insert(cvm: &mut Cvm, next_idx: usize) -> usize {
    let CvmObject::Int(idx) = cvm.stack.pop().unwrap() else {
        panic!("inserting in a non-int index");
    };
    let val = cvm.stack.pop().unwrap();
    let CvmObject::List(list) = cvm.stack.pop().unwrap() else {
        panic!("inserting in a non-list");
    };

    let mut list = list.borrow_mut();
    match get_idx(idx, list.len()) {
        Ok(idx) => {
            list.insert(idx, val);
            next_idx
        }
        Err(exc) => {
            cvm.stack.push(CvmObject::Exception(exc));
            cvm.handle_exception()
        }
    }
}

/* returns the index of the next instruction */
pub fn list_remove(cvm: &mut Cvm, next_idx: usize) -> usize {
    let CvmObject::Int(idx) = cvm.stack.pop().unwrap() else {
        panic!("removing with a non-int index");
    };
    let CvmObject::List(list) = cvm.stack.pop().unwrap() else {
        panic!("removing from a non-list");
    };

    let mut list = list.borrow_mut();
    match get_idx_strict(idx, list.len()) {
        Ok(idx) => {
            cvm.stack.push(list.remove(idx).unwrap());
            next_idx
        }
        Err(exc) => {
            cvm.stack.push(CvmObject::Exception(exc));
            cvm.handle_exception()
        }
    }
}

/* returns the index of the next instruction */
pub fn list_get(cvm: &mut Cvm, next_idx: usize) -> usize {
    let CvmObject::Int(idx) = cvm.stack.pop().unwrap() else {
        panic!("indexing with a non-int value");
    };
    let CvmObject::List(list) = cvm.stack.pop().unwrap() else {
        panic!("getting from a non-list");
    };

    let list = list.borrow();
    match get_idx(idx, list.len()) {
        Ok(idx) => {
            cvm.stack.push(list.get(idx).unwrap().clone());
            next_idx
        }
        Err(exc) => {
            cvm.stack.push(CvmObject::Exception(exc));
            cvm.handle_exception()
        }
    }
}

/* returns the index of the next instruction */
pub fn list_set(cvm: &mut Cvm, next_idx: usize) -> usize {
    let CvmObject::Int(idx) = cvm.stack.pop().unwrap() else {
        panic!("indexing with a non-int value");
    };
    let val = cvm.stack.pop().unwrap();
    let CvmObject::List(list) = cvm.stack.pop().unwrap() else {
        panic!("setting a non-list");
    };

    let mut list = list.borrow_mut();
    match get_idx_strict(idx, list.len()) {
        Ok(idx) => {
            *list.get_mut(idx).unwrap() = val;
            next_idx
        }
        Err(exc) => {
            cvm.stack.push(CvmObject::Exception(exc));
            cvm.handle_exception()
        }
    }
}

fn get_idx(idx: i64, list_len: usize) -> Result<usize, PtrString> {
    if idx < (-(list_len as i64) - 1) || idx > list_len as i64 {
        let msg = format!("invalid index ({}) for list with len {}", idx, list_len);
        return Err(msg.into());
    }

    if idx < 0 {
        /* this makes idx always positive */
        return Ok((list_len as i64 + idx + 1) as usize);
    }
    Ok(idx as usize)
}

/* used for the `set!` instruction, which requires a more strict indexing */
fn get_idx_strict(idx: i64, list_len: usize) -> Result<usize, PtrString> {
    if idx < (-(list_len as i64) - 1) || idx >= list_len as i64 {
        let msg = format!(
            "invalid strict index ({}) for list with len {}",
            idx, list_len
        );
        return Err(msg.into());
    }

    if idx < 0 {
        /* this makes idx always positive */
        return Ok((list_len as i64 + idx + 1) as usize);
    }
    Ok(idx as usize)
}
