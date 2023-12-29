use crate::error::ChalError;
use crate::lexer::{Keyword, TokenKind};
use crate::parser::ast::NodeExpr;
use crate::parser::TokenReader;

pub struct NodeRetStmnt(pub NodeExpr);

impl NodeRetStmnt {
    pub fn new(mut reader: TokenReader) -> Result<Self, ChalError> {
        reader.expect_exact(TokenKind::Keyword(Keyword::Return))?;

        let value_raw = reader.advance_until(|tk| *tk == TokenKind::Newline)?;
        let value = NodeExpr::new(value_raw, reader.span())?;

        Ok(NodeRetStmnt(value))
    }
}
