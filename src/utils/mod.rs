use std::collections::VecDeque;

#[derive(Debug)]
pub struct Stack<T> {
    values: VecDeque<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            values: VecDeque::<T>::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.values.push_back(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.values.pop_back()
    }

    pub fn peek(&self) -> Option<&T> {
        self.values.back()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl<T> Into<VecDeque<T>> for Stack<T> {
    fn into(self) -> VecDeque<T> {
        self.values
    }
}
