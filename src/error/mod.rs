mod color;
mod scopes;
pub mod span;

pub mod error;

pub use error::ChalError;

pub use color::{color, err, internal, warn, Colors};
pub use scopes::{CompileError, InternalError, LexerError, ParserError};
