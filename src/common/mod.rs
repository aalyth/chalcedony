//! The module containing common functionalities, used in various parts of the
//! `Chalcedony` interpreter.

mod bytecode;
pub mod operators;
mod types;

pub use bytecode::Bytecode;
pub use types::Type;
