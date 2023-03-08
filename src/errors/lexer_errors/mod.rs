//mod invalid_identifier;

#[derive(PartialEq, Debug, Clone)]
pub enum LexerError { 
    InvalidIdentifier,
    UnclosedString,
    UnclosedComment, // this is for multiline comments
    UnclosedDelimiter(String),
    UnexpectedClosingDelimiter(String),
    MissmatchingDelimiter(String, String),
}

