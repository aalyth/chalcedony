use crate::error::{span::Span, ChalError};
use crate::lexer::{Keyword, TokenKind};
use crate::parser::ast::NodeExpr;
use crate::parser::TokenReader;

pub struct NodeRetStmnt {
    pub value: NodeExpr,
    pub span: Span,
}

impl NodeRetStmnt {
    pub fn new(mut reader: TokenReader) -> Result<Self, ChalError> {
        let span = reader.current();
        reader.expect_exact(TokenKind::Keyword(Keyword::Return))?;

        let value_raw = reader.advance_until(|tk| *tk == TokenKind::Newline)?;
        let value_reader = TokenReader::new(value_raw, reader.spanner());
        let value = NodeExpr::new(value_reader)?;

        Ok(NodeRetStmnt { value, span })
    }
}
