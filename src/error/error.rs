use crate::error::scopes::{CompileError, InternalError, LexerError, ParserError};

pub enum ChalError {
    LexerErr(LexerError),
    ParserErr(ParserError),
    CompileErr(CompileError),
    InternalErr(InternalError),
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

impl From<InternalError> for ChalError {
    fn from(err: InternalError) -> Self {
        ChalError::InternalErr(err)
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
            ChalError::InternalErr(err) => write!(f, "{}", err),
        }
    }
}
