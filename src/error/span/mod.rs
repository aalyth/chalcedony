mod pos;
mod spanning;

pub use pos::Position;
pub use spanning::InlineSpanner;

use std::rc::Rc;

/// The trait, used to define objects which can build code snippets from a given
/// start and end position in the source code.
pub trait Spanning {
    fn context(&self, start: &Position, end: &Position) -> String;
}

/// The structure, denoting a snippet of source code. Used in numerous structures
/// from the Lexer's tokens to the Abstract Syntax Tree nodes. The purpose of this
/// abstractions is to provide an easy way to display adequate error messages
/// with code snippets upon any encountered error.
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
