use crate::error::{ChalError, ParserError, Span};
use crate::lexer::{Token, TokenKind};

use std::rc::Rc;

pub struct NodeVarCall(pub String);

impl NodeVarCall {
    pub fn new(token: Token, span: Rc<Span>) -> Result<Self, ChalError> {
        let kind = token.kind();
        let TokenKind::Identifier(name) = kind else {
            return Err(ParserError::invalid_token(
                TokenKind::Identifier(String::new()),
                kind.clone(),
                token.start(),
                token.end(),
                span.clone(),
            )
            .into());
        };
        Ok(NodeVarCall(name.clone()))
    }
}
