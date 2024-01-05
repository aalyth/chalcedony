pub mod format;
pub mod scopes;
pub mod span;

pub mod error;

pub use error::ChalError;

pub use scopes::{CompileError, InternalError, LexerError, ParserError, RuntimeError};
