use std::alloc::{Layout, alloc, dealloc};
use std::ptr;
use std::fmt;

/* an 8-byte ASCII string implementation */
#[derive(Debug, Copy)]
pub struct PtrString (*const u8);

impl PtrString {
    unsafe fn len(&self) -> usize {
        let PtrString(ptr) = self;
        let mut i: usize = 0;
        while *ptr.add(i) != 0  {
            i += 1;
        }
        i
    }
}

// TODO: add multiplication
impl std::ops::Add<PtrString> for PtrString {
    type Output = PtrString;  

    fn add(self, rhs: PtrString) -> Self::Output {
        unsafe {
            let lhs_len = self.len();
            let rhs_len = rhs.len();

            let lhs = self.0 as *mut u8;
            let rhs = rhs.0 as *mut u8;

            let res_layout = Layout::array::<*const u8>(lhs_len + rhs_len + 1)
                .expect("Error: creating a string with size greater than isize::MAX");
            
            let res = alloc(res_layout);
            ptr::copy(lhs, res, lhs_len);
            ptr::copy(rhs, res.add(lhs_len), rhs_len);
            ptr::write(res.add(lhs_len + rhs_len + 1), 0);

            let lhs_layout = Layout::array::<*const u8>(lhs_len + 1)
                .expect("Error: creating a string with size greater than isize::MAX");

            let rhs_layout = Layout::array::<*const u8>(lhs_len + 1)
                .expect("Error: creating a string with size greater than isize::MAX");
            
            dealloc(lhs, lhs_layout);
            dealloc(rhs, rhs_layout);

            PtrString(res)
        }
    }
}

impl fmt::Display for PtrString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let mut res = String::new();
            let ptr = self.0 as *const u8;
            let mut i: usize = 0;
            while *ptr.add(i) != 0 {
                let ch = *ptr.add(i);
                res.push((ch as char).clone());
                i += 1;
            }
            write!(f, "{}", res)
        }
    }
}

impl From<String> for PtrString {
    fn from(val: String) -> PtrString {
        unsafe {
            // SAFETY: the error value is upon isize overflow, which would be extremely rare
            let layout = Layout::array::<*const u8>(val.len() + 1)
                .expect("Error: creating a string with size greater than isize::MAX");
            let res: *mut u8 = alloc(layout) as *mut u8;
            ptr::copy(val.as_ptr(), res, val.len());
            ptr::write(res.add(val.len()), 0);
            PtrString(res)
        }
    }
}

impl Clone for PtrString {
    fn clone(&self) -> Self {
        unsafe {
            let ptr = self.0;
            let len = self.len();

            let res_layout = Layout::array::<*const u8>(len + 1)
                .expect("Error: creating a string with size greater than isize::MAX");
            let res = alloc(res_layout);

            ptr::copy_nonoverlapping(ptr, res, len + 1);
            PtrString(res)
        }
    }
}
