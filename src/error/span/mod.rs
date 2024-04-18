mod pos;
mod spanning;

pub use pos::Position;
pub use spanning::InlineSpanner;

use std::fmt;
use std::rc::Rc;

pub trait Spanning {
    fn context(&self, start: &Position, end: &Position) -> String;
}

#[derive(Clone)]
pub struct Span {
    pub start: Position,
    pub end: Position,
    pub spanner: Rc<dyn Spanning>,
}

impl Span {
    pub fn new(start: Position, end: Position, spanner: Rc<dyn Spanning>) -> Self {
        Span {
            start,
            end,
            spanner,
        }
    }

    pub fn context(&self) -> String {
        self.spanner.context(&self.start, &self.end)
    }
}

impl From<Rc<dyn Spanning>> for Span {
    fn from(value: Rc<dyn Spanning>) -> Self {
        Span {
            start: Position::new(0, 0),
            end: Position::new(0, 0),
            spanner: value.clone(),
        }
    }
}

impl std::cmp::PartialEq for Span {
    fn eq(&self, other: &Span) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<span instance>")
    }
}
