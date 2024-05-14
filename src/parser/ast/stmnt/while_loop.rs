use crate::error::{span::Span, ChalError};
use crate::lexer::{Keyword, Special, TokenKind};
use crate::parser::ast::{NodeExpr, NodeStmnt};
use crate::parser::{LineReader, TokenReader};

/// The structure representing a while loop.
///
/// Syntax:
/// `while` \<condition\>:
///     \<statments\>
#[derive(Debug, PartialEq)]
pub struct NodeWhileLoop {
    pub condition: NodeExpr,
    pub body: Vec<NodeStmnt>,
}

impl NodeWhileLoop {
    pub fn new(mut reader: LineReader) -> Result<Self, ChalError> {
        let mut header = reader.advance_reader();
        header.expect_exact(TokenKind::Keyword(Keyword::While))?;

        let cond_raw = header.advance_until(|tk| {
            *tk == TokenKind::Special(Special::Colon) || *tk == TokenKind::Newline
        })?;
        let cond_reader = TokenReader::new(cond_raw, Span::from(reader.spanner()));
        let cond = NodeExpr::new(cond_reader)?;

        header.expect_exact(TokenKind::Special(Special::Colon))?;
        header.expect_exact(TokenKind::Newline)?;

        Ok(NodeWhileLoop {
            condition: cond,
            body: reader.try_into()?,
        })
    }
}
