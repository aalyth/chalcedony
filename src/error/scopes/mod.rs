mod compile;
mod internal;
mod lexer;
mod parser;

pub use compile::CompileError;
pub use internal::InternalError;
pub use lexer::LexerError;
pub use parser::ParserError;

use super::err;

use super::span::Span;

fn display_err(span: &Span, f: &mut std::fmt::Formatter, msg: &str) -> std::fmt::Result {
    write!(f, "{}:\n{}\n", err(msg), span.context(),)
}
