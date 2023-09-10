use crate::error::LexerError;
use crate::error::ParserError;

pub enum ChalError<'a> {
    LexerErr (LexerError<'a>),
    ParserErr (ParserError<'a>),
}

impl<'a> From<LexerError<'a>> for ChalError<'_> {
    fn from(err: LexerError) -> Self {
        ChalError::LexerErr(err)
    }
}

impl<'a> From<ParserError<'a>> for ChalError<'_> {
    fn from(err: ParserError) -> Self {
        ChalError::ParserErr(err)
    }
}

impl std::fmt::Display for ChalError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            ChalError::LexerErr(err)  => write!(f, "{}", err),
            ChalError::ParserErr(err) => write!(f, "{}", err),
        }
    }
}
