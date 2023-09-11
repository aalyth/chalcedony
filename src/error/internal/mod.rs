use crate::error::format::internal;

/* indicates something is wrong with the interpreter itself*/
pub struct InternalError<'a> {
    msg: &'a str,
}

impl<'a> InternalError<'a> {
    pub fn new(msg: &str) -> Self {
        InternalError { msg }
    }
}

impl std::fmt::Display for InternalError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:\n{}", internal(self.msg), self.span.context(self.start, self.end))
    }
}
