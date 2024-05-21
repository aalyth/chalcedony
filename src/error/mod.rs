//! The module containing all the functionality, related to errors inside the
//! `Chalcedony` interpreter.
//!
//! It includes all of the different types of errors that could be encoutnered
//! (for more information look at each individual `ChalError` element), the
//! generation of code snippets (via the `span` module) and some utilities such
//! as terminal coloring.

mod color;
mod scopes;
pub mod span;

#[allow(unused_imports)]
pub use color::{color, err, warn, Colors};
pub use scopes::{
    CompileError, CompileErrorKind, LexerError, LexerErrorKind, ParserError, ParserErrorKind,
};

pub fn unhandled_exception(exc: String) {
    let fail_msg = color(Colors::Blue, "Unhandled exception");
    eprintln!("{}: {}", fail_msg, exc);
    terminate_program();
}

fn terminate_program() {
    #[cfg(not(feature = "panicking-asserts"))]
    std::process::exit(1);

    // this is a workaround since tests don't check for `std::process::exit()`
    #[cfg(feature = "panicking-asserts")]
    panic!("");
}

pub enum ChalError {
    LexerErr(LexerError),
    ParserErr(ParserError),
    CompileErr(CompileError),
    ErrorChunk(Vec<ChalError>),
}

impl From<LexerError> for ChalError {
    fn from(err: LexerError) -> Self {
        ChalError::LexerErr(err)
    }
}

impl From<Vec<ChalError>> for ChalError {
    fn from(chunk: Vec<ChalError>) -> Self {
        ChalError::ErrorChunk(chunk)
    }
}

impl From<ParserError> for ChalError {
    fn from(err: ParserError) -> Self {
        ChalError::ParserErr(err)
    }
}

impl From<CompileError> for ChalError {
    fn from(err: CompileError) -> Self {
        ChalError::CompileErr(err)
    }
}

impl std::fmt::Display for ChalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            ChalError::ErrorChunk(chunk) => {
                let mut res = String::new();
                for err in chunk {
                    res.push_str(&format!("{}", err));
                }
                write!(f, "{}", res)
            }

            ChalError::LexerErr(err) => write!(f, "{}", err),
            ChalError::ParserErr(err) => write!(f, "{}", err),
            ChalError::CompileErr(err) => write!(f, "{}", err),
        }
    }
}

impl std::fmt::Debug for ChalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
