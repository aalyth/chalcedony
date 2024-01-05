use crate::error::span::Spanning;
use crate::error::{ChalError, ParserError};
use crate::lexer::{Token, TokenKind};

use std::rc::Rc;

pub struct NodeVarCall(pub String);

impl NodeVarCall {
    pub fn new(token: Token, spanner: Rc<dyn Spanning>) -> Result<Self, ChalError> {
        let kind = token.kind;
        let TokenKind::Identifier(name) = kind else {
            return Err(ParserError::invalid_token(
                TokenKind::Identifier(String::new()),
                kind.clone(),
                token.span,
            )
            .into());
        };
        Ok(NodeVarCall(name.clone()))
    }
}
