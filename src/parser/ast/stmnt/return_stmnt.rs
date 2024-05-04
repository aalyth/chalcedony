use crate::error::{span::Span, ChalError};
use crate::lexer::{Keyword, TokenKind};
use crate::parser::ast::NodeExpr;
use crate::parser::TokenReader;

/// The node representing the returning of value
///
/// Syntax:
/// `return` \<expr\>
///
/// N.B.: \<expr\> can be empty for `void` functions
#[derive(Debug, PartialEq)]
pub struct NodeRetStmnt {
    pub value: NodeExpr,
    pub span: Span,
}

impl NodeRetStmnt {
    pub fn new(mut reader: TokenReader) -> Result<Self, ChalError> {
        let span = reader.current();
        reader.expect_exact(TokenKind::Keyword(Keyword::Return))?;

        let value: NodeExpr;
        let value_raw = reader.advance_until(|tk| *tk == TokenKind::Newline)?;
        if !value_raw.is_empty() {
            let value_reader = TokenReader::new(value_raw, reader.current());
            value = NodeExpr::new(value_reader)?;
        } else {
            value = NodeExpr::empty(reader.current());
        }

        Ok(NodeRetStmnt { value, span })
    }
}
