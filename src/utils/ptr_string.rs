use std::alloc::{alloc, dealloc, Layout};
use std::cmp;
use std::fmt;
use std::ops;
use std::ptr;

/// An 8-byte string implementation. The purpose of this structure is to
/// optimize the memory size of every `CvmObject`, reducing it by 16 bytes
/// compared to using the Rust's built-in `String` type. Optimizing the memory
/// footprint of the CVM objects leads to improved speed of the user's programs.
pub struct PtrString(*const char);

impl PtrString {
    unsafe fn len(&self) -> usize {
        let ptr = self.0;
        let mut i: usize = 0;
        while *ptr.add(i) != '\0' {
            i += 1;
        }
        i
    }
}

impl ops::Add<PtrString> for PtrString {
    type Output = PtrString;

    fn add(self, rhs: PtrString) -> Self::Output {
        unsafe {
            let lhs_len = self.len();
            let rhs_len = rhs.len();

            let lhs = self.0 as *mut char;
            let rhs = rhs.0 as *mut char;

            let res_layout = Layout::array::<*const char>(lhs_len + rhs_len + 1)
                .expect("Error: creating a string with size greater than isize::MAX");

            let res = alloc(res_layout) as *mut char;
            ptr::copy(lhs, res, lhs_len);
            ptr::copy(rhs, res.add(lhs_len), rhs_len);
            ptr::write(res.add(lhs_len + rhs_len), '\0');

            PtrString(res as *const char)
        }
    }
}

impl ops::Mul<usize> for PtrString {
    type Output = PtrString;

    fn mul(self, mult: usize) -> Self::Output {
        unsafe {
            let len = self.len();
            let lhs = self.0 as *mut char;

            let res_layout = Layout::array::<*const char>(len * mult + 1)
                .expect("Error: creating a string with size greater than isize::MAX");

            let res = alloc(res_layout) as *mut char;

            for i in 0..mult {
                ptr::copy(lhs, res.add(i * len), len);
            }

            ptr::write(res.add(len * mult), '\0');

            PtrString(res)
        }
    }
}

impl cmp::PartialEq for PtrString {
    fn eq(&self, other: &PtrString) -> bool {
        self.partial_cmp(other).is_none()
    }
}

impl cmp::PartialOrd for PtrString {
    fn partial_cmp(&self, other: &PtrString) -> Option<cmp::Ordering> {
        unsafe {
            let lhs_len = self.len();
            let rhs_len = other.len();
            let len = cmp::min(lhs_len, rhs_len);

            for i in 0..len {
                let lval = *self.0.add(i);
                let rval = *other.0.add(i);
                if lval < rval {
                    return Some(cmp::Ordering::Less);
                }
                if lval > rval {
                    return Some(cmp::Ordering::Greater);
                }
            }

            if lhs_len < rhs_len {
                return Some(cmp::Ordering::Less);
            }

            if lhs_len > rhs_len {
                return Some(cmp::Ordering::Greater);
            }

            None
        }
    }
}

impl fmt::Display for PtrString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let mut res = String::new();
            let ptr = self.0;
            let mut i: usize = 0;
            while *ptr.add(i) != '\0' {
                res.push(*ptr.add(i));
                i += 1;
            }
            write!(f, "{}", res)
        }
    }
}

impl fmt::Debug for PtrString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<String> for PtrString {
    fn from(val: String) -> PtrString {
        unsafe {
            // SAFETY: the error value is upon isize overflow, which would be an
            // extremely rare case
            let layout = Layout::array::<*const char>(val.len() + 1)
                .expect("Error: creating a string with size greater than isize::MAX");

            let res = alloc(layout) as *mut char;

            let val_len = val.len();
            for (idx, ch) in val.chars().enumerate() {
                ptr::write(res.add(idx), ch);
            }
            ptr::write(res.add(val_len), '\0');

            PtrString(res)
        }
    }
}

impl Clone for PtrString {
    fn clone(&self) -> Self {
        unsafe {
            let ptr = self.0;
            let len = self.len();

            let res_layout = Layout::array::<*const char>(len + 1)
                .expect("Error: creating a string with size greater than isize::MAX");
            let res = alloc(res_layout) as *mut char;

            ptr::copy_nonoverlapping(ptr, res, len + 1);
            PtrString(res)
        }
    }
}

impl ops::Drop for PtrString {
    fn drop(&mut self) {
        unsafe {
            let len = self.len();
            let ptr = self.0 as *mut u8;
            let layout = Layout::array::<*const char>(len + 1)
                .expect("Error: creating a string with size greater than isize::MAX");
            dealloc(ptr, layout)
        }
    }
}
