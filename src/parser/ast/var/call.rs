use crate::error::{ChalError, ParserError, Span};
use crate::lexer::{Token, TokenKind};

use std::rc::Rc;

#[derive(Debug)]
pub struct NodeVarCall(pub String);

impl NodeVarCall {
    pub fn new(token: Token, span: Rc<Span>) -> Result<Self, ChalError> {
        let kind = token.kind();
        let TokenKind::Identifier(name) = kind else {
            return Err(ChalError::from(ParserError::invalid_token(
                TokenKind::Identifier(String::new()),
                kind.clone(),
                token.start(),
                token.end(),
                span.clone(),
            )));
        };
        Ok(NodeVarCall(name.clone()))
    }
}
