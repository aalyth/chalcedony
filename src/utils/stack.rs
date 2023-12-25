#[derive(Debug)]
pub struct Stack<T> {
    values: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            values: Vec::<T>::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.values.push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.values.pop()
    }

    pub fn top(&mut self) -> Option<&mut T> {
        self.values.last_mut()
    }

    pub fn peek(&self) -> Option<&T> {
        self.values.last()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl<T> Into<Vec<T>> for Stack<T> {
    fn into(self) -> Vec<T> {
        self.values
    }
}
