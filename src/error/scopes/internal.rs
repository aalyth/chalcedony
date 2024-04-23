use crate::error::internal;

/// A structure, representing an internal error with the interpreter.
///
/// TODO: Currently this structure is outdated and should be replaced by panics
/// inside the source code.
pub struct InternalError {
    msg: String,
}

impl InternalError {
    pub fn new(msg: &str) -> Self {
        InternalError {
            msg: msg.to_string(),
        }
    }
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", internal(&self.msg))
    }
}
