use crate::error::span::Span;
use crate::error::{ChalError, ParserError, ParserErrorKind};
use crate::lexer::{Token, TokenKind};

/// The node representing a variable's call. Essentialy boils down to a single
/// `TokenKind::Identifier()` with the corresponding variable's name inside.
///
/// Syntax:
/// \<var_name\>
#[derive(Clone, Debug, PartialEq)]
pub struct NodeVarCall {
    pub name: String,
    pub span: Span,
}

impl NodeVarCall {
    pub fn new(token: Token) -> Result<Self, ChalError> {
        let kind = token.kind;
        let TokenKind::Identifier(name) = kind else {
            return Err(ParserError::new(
                ParserErrorKind::InvalidToken(TokenKind::Identifier(String::new()), kind.clone()),
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
