use std::collections::VecDeque;

/// A wrapper around the default `Vec<T>` type, used for better code readability.
#[derive(Debug, Default)]
pub struct Stack<T> {
    values: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            values: Vec::<T>::new(),
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Stack {
            values: Vec::<T>::with_capacity(cap),
        }
    }

    pub fn push(&mut self, val: T) {
        self.values.push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.values.pop()
    }

    pub fn get(&mut self, idx: usize) -> Option<&T> {
        self.values.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.values.get_mut(idx)
    }

    pub fn top(&mut self) -> Option<&mut T> {
        self.values.last_mut()
    }

    // Keeps the first `len` elements on the stack, removing the others.
    pub fn truncate(&mut self, len: usize) {
        self.values.truncate(len)
    }

    pub fn peek(&self) -> Option<&T> {
        self.values.last()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl<T> From<Stack<T>> for VecDeque<T> {
    fn from(value: Stack<T>) -> Self {
        value.values.into()
    }
}
