use super::{Cvm, CvmObject};

use crate::utils::PtrString;

/* returns the index of the next instruction */
#[inline(always)]
pub fn list_insert(cvm: &mut Cvm, next_idx: usize) -> usize {
    let CvmObject::Int(idx) = cvm.pop() else {
        panic!("inserting in a non-int index");
    };
    let val = cvm.pop();
    let CvmObject::List(list) = cvm.pop() else {
        panic!("inserting in a non-list");
    };

    let mut list = list.borrow_mut();
    match get_idx(idx, list.len()) {
        Ok(idx) => {
            list.insert(idx, val);
            next_idx
        }
        Err(exc) => {
            cvm.push(CvmObject::Exception(exc));
            cvm.handle_exception()
        }
    }
}

/* returns the index of the next instruction */
#[inline(always)]
pub fn list_remove(cvm: &mut Cvm, next_idx: usize) -> usize {
    let CvmObject::Int(idx) = cvm.pop() else {
        panic!("inserting in a non-int index");
    };
    let CvmObject::List(list) = cvm.pop() else {
        panic!("inserting in a non-list");
    };

    let mut list = list.borrow_mut();
    match get_idx(idx, list.len()) {
        Ok(idx) => {
            cvm.push(list.remove(idx).unwrap());
            next_idx
        }
        Err(exc) => {
            cvm.push(CvmObject::Exception(exc));
            cvm.handle_exception()
        }
    }
}

/* returns the index of the next instruction */
#[inline(always)]
pub fn list_get(cvm: &mut Cvm, next_idx: usize) -> usize {
    let CvmObject::Int(idx) = cvm.pop() else {
        panic!("indexing with a non-int value");
    };
    let CvmObject::List(list) = cvm.pop() else {
        panic!("getting from a non-list");
    };

    let list = list.borrow();
    match get_idx(idx, list.len()) {
        Ok(idx) => {
            cvm.push(list.get(idx).unwrap().clone());
            next_idx
        }
        Err(exc) => {
            cvm.push(CvmObject::Exception(exc));
            cvm.handle_exception()
        }
    }
}

fn get_idx(idx: i64, list_len: usize) -> Result<usize, PtrString> {
    if idx.unsigned_abs() as usize > list_len {
        let msg = format!("invalid index ({}) for list with len {}", idx, list_len);
        return Err(msg.into());
    }

    if idx < 0 {
        /* this makes idx always positive */
        return Ok((list_len as i64 + idx) as usize);
    }
    Ok(idx as usize)
}
