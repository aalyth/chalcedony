use crate::error::span::Span;
use crate::error::{ChalError, ParserError};
use crate::lexer::{Token, TokenKind};

#[derive(Clone, Debug)]
pub struct NodeVarCall {
    pub name: String,
    pub span: Span,
}

impl NodeVarCall {
    pub fn new(token: Token) -> Result<Self, ChalError> {
        let kind = token.kind;
        let TokenKind::Identifier(name) = kind else {
            return Err(ParserError::invalid_token(
                TokenKind::Identifier(String::new()),
                kind.clone(),
                token.span,
            )
            .into());
        };
        Ok(NodeVarCall {
            name: name.clone(),
            span: token.span,
        })
    }
}
