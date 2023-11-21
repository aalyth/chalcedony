use std::collections::VecDeque;

pub trait Reader<T> {
    fn advance(&mut self) -> Option<T>;

    fn advance_while(&mut self, cond: fn(&T) -> bool) -> VecDeque<T>;

    fn peek(&self) -> Option<&T>;

    fn is_empty(&self) -> bool;
}
