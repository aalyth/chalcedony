use crate::error::ChalError;
use crate::lexer::{Keyword, Special, TokenKind};
use crate::parser::ast::{NodeExpr, NodeStmnt, NodeVarCall};
use crate::parser::{LineReader, TokenReader};

#[derive(Debug)]
pub struct NodeWhileLoop {
    pub condition: NodeExpr,
    pub body: Vec<NodeStmnt>,
}

impl NodeWhileLoop {
    pub fn new(mut reader: LineReader) -> Result<Self, ChalError> {
        /* while loop structure:
         * while a <= 42:    | header
         *     print(a)      > body
         *     a += 1        > body
         */

        let mut header = reader.advance_reader()?;
        header.expect_exact(TokenKind::Keyword(Keyword::While))?;

        let cond_raw = header.advance_until(|tk| {
            *tk == TokenKind::Special(Special::Colon) || *tk == TokenKind::Newline
        })?;
        let cond_reader = TokenReader::new(cond_raw, header.current());
        let cond = NodeExpr::new(cond_reader)?;

        header.expect_exact(TokenKind::Special(Special::Colon))?;
        header.expect_exact(TokenKind::Newline)?;

        Ok(NodeWhileLoop {
            condition: cond,
            body: reader.try_into()?,
        })
    }
}

#[derive(Debug)]
pub struct NodeForLoop {
    pub iter: NodeVarCall,
    pub iterable: NodeExpr,
    pub body: Vec<NodeStmnt>,
}

impl NodeForLoop {
    pub fn new(mut reader: LineReader) -> Result<Self, ChalError> {
        // for loop structure:
        // for i in [1, 2, 3]:    | header
        //     print(i)           > body
        //     let c = i * 3      > body
        //

        let mut header = reader.advance_reader()?;
        header.expect_exact(TokenKind::Keyword(Keyword::For))?;

        let iter_raw = header.expect(TokenKind::Identifier("".to_string()))?;
        let iter = NodeVarCall::new(iter_raw)?;

        header.expect_exact(TokenKind::Keyword(Keyword::In))?;

        let iterable_raw = header.advance_until(|tk| {
            *tk == TokenKind::Special(Special::Colon) || *tk == TokenKind::Newline
        })?;
        let iterable_reader = TokenReader::new(iterable_raw, header.current());
        let iterable = NodeExpr::new(iterable_reader)?;

        header.expect_exact(TokenKind::Special(Special::Colon))?;
        header.expect_exact(TokenKind::Newline)?;

        Ok(NodeForLoop {
            iter,
            iterable,
            body: reader.try_into()?,
        })
    }
}
