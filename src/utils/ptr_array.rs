use std::alloc::{alloc, dealloc, Layout};
use std::ops;
use std::ptr;

#[derive(Debug)]
pub struct PtrArray<T: Clone>(*const T);

// TODO: make this not leak memory
impl<T: Clone> PtrArray<T> {
    pub fn new(len: usize) -> Self {
        let layout = Layout::array::<*const u8>(std::mem::size_of::<T>() * len + 8)
            .expect("Error: creating a string with size greater than isize::MAX");
        let data = unsafe { alloc(layout) } as *mut u64;
        unsafe { ptr::write(data, len as u64) };
        PtrArray((unsafe { data.add(1) }) as *const T)
    }

    pub fn get(&self, idx: usize) -> T {
        unsafe { ptr::read(self.0.add(idx)).clone() }
    }

    pub fn set(&self, idx: usize, val: T) {
        unsafe { ptr::write(self.0.add(idx) as *mut T, val) }
    }
}

impl<T: Clone> ops::Drop for PtrArray<T> {
    fn drop(&mut self) {
        unsafe {
            let data_raw = self.0 as *const u64;
            let data = data_raw.offset(-1);
            let len = ptr::read(data) as usize;
            let layout = Layout::array::<*const u8>(std::mem::size_of::<T>() * len + 8)
                .expect("Error: creating a string with size greater than isize::MAX");
            dealloc(data as *mut u8, layout)
        }
    }
}
