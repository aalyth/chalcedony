pub mod compile;
pub mod internal;
pub mod lexer;
pub mod parser;
pub mod runtime;

pub use compile::CompileError;
pub use internal::InternalError;
pub use lexer::LexerError;
pub use parser::ParserError;
pub use runtime::RuntimeError;

use super::format::err;

use super::span::Span;

fn display_err(span: &Span, f: &mut std::fmt::Formatter, msg: &str) -> std::fmt::Result {
    write!(f, "{}:\n{}\n", err(msg), span.context(),)
}
