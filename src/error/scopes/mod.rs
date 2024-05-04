//! The module containing all error types which could be encountered during the
//! first 3 stages of interpreting: lexing, parsing, and compiling to bytecode.

mod compile;
mod lexer;
mod parser;

pub use compile::{CompileError, CompileErrorKind};
pub use lexer::{LexerError, LexerErrorKind};
pub use parser::{ParserError, ParserErrorKind};

use super::err;

use super::span::Span;

fn display_err(span: &Span, f: &mut std::fmt::Formatter, msg: &str) -> std::fmt::Result {
    write!(f, "{}:\n{}\n", err(msg), span.context(),)
}
