use crate::error::{ChalError};
use crate::lexer::{Keyword, Special, TokenKind};
use crate::parser::ast::{parse_body, NodeExpr, NodeStmnt};
use crate::parser::{LineReader};

use std::collections::VecDeque;

#[derive(Debug)]
pub struct NodeWhileLoop {
    condition: NodeExpr,
    body: VecDeque<NodeStmnt>,
}

impl NodeWhileLoop {
    pub fn new(mut reader: LineReader) -> Result<Self, ChalError> {
        // Err(ChalError::from(InternalError::new("NodeWhileLoop not implemented")))

        // while loop structure:
        // while a <= 42:    | header
        //     print(a)      > body
        //     a += 1        > body

        let mut header = reader.advance_reader()?;
        header.expect_exact(TokenKind::Keyword(Keyword::While))?;

        let cond_raw = header.advance_until(|tk| *tk == TokenKind::Special(Special::Colon))?;
        let cond = NodeExpr::new(cond_raw, reader.span())?;

        header.expect_exact(TokenKind::Special(Special::Colon))?;
        header.expect_exact(TokenKind::Newline)?;

        Ok(NodeWhileLoop {
            condition: cond,
            body: parse_body(reader)?,
        })
    }
}
