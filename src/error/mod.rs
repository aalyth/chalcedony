mod color;
mod scopes;
pub mod span;

pub mod error;

pub use error::ChalError;

pub use color::{color, err, internal, warn, Colors};
pub use scopes::{CompileError, InternalError, LexerError, ParserError};

pub fn assertion_fail(exp: String, recv: String) {
    let fail_msg = color(Colors::Blue, "Assertion fail");
    eprintln!("{} - expected: {}, received: {}", fail_msg, exp, recv);
    std::process::exit(1);
}
