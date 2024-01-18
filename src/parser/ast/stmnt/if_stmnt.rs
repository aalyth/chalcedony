use crate::error::{ChalError, InternalError};
use crate::lexer::{Keyword, Special, TokenKind};
use crate::parser::ast::{NodeExpr, NodeStmnt};

use crate::parser::LineReader;

use super::parse_body;

pub struct NodeIfStmnt {
    pub condition: NodeExpr,
    pub body: Vec<NodeStmnt>,
    pub branches: Vec<NodeIfBranch>,
}

pub enum NodeIfBranch {
    Elif(NodeElifStmnt),
    Else(NodeElseStmnt),
}

pub struct NodeElifStmnt {
    condition: NodeExpr,
    body: Vec<NodeStmnt>,
}

pub struct NodeElseStmnt {
    body: Vec<NodeStmnt>,
}

impl NodeIfStmnt {
    pub fn new(mut reader: LineReader) -> Result<Self, ChalError> {
        let mut header = reader.advance_reader()?;
        header.expect_exact(TokenKind::Keyword(Keyword::If))?;

        let cond_raw = header.advance_until(|tk| *tk == TokenKind::Special(Special::Colon))?;
        let condition = NodeExpr::new(cond_raw, reader.spanner())?;

        header.expect_exact(TokenKind::Special(Special::Colon))?;
        header.expect_exact(TokenKind::Newline)?;

        let body = reader.advance_until(|ln| {
            let Some(front) = ln.front_tok() else {
                return false;
            };
            front.kind == TokenKind::Keyword(Keyword::Else)
                || front.kind == TokenKind::Keyword(Keyword::Elif)
        })?;

        let mut branches = Vec::<NodeIfBranch>::new();
        /* NOTE: this block is guaranteed to be with at most 1 else statement
         * (refer to LineReader::advance_chunk()) */
        while !reader.is_empty() {
            let next_branch = reader.advance_until(|ln| {
                let Some(front) = ln.front_tok() else {
                    return false;
                };
                front.kind == TokenKind::Keyword(Keyword::Elif)
                    || front.kind == TokenKind::Keyword(Keyword::Else)
            })?;

            branches.push(NodeIfBranch::new(LineReader::new(
                next_branch,
                reader.spanner(),
            ))?);
        }

        Ok(NodeIfStmnt {
            condition,
            branches,
            body: parse_body(LineReader::new(body, reader.spanner()))?,
        })
    }

    pub fn disassemble(self) -> (NodeExpr, Vec<NodeStmnt>, Vec<NodeIfBranch>) {
        (self.condition, self.body, self.branches)
    }
}

impl NodeIfBranch {
    pub fn new(reader: LineReader) -> Result<Self, ChalError> {
        let Some(front_tok) = reader.peek_tok() else {
            return Err(InternalError::new(
                "NodeIFBranch::new(): generating an if branch from an empty reader",
            )
            .into());
        };

        match front_tok.kind {
            TokenKind::Keyword(Keyword::Elif) => {
                Ok(NodeIfBranch::Elif(NodeElifStmnt::new(reader)?))
            }
            TokenKind::Keyword(Keyword::Else) => {
                Ok(NodeIfBranch::Else(NodeElseStmnt::new(reader)?))
            }
            _ => Err(InternalError::new("NodeIfBranch::new(): advancing a non-if branch").into()),
        }
    }
}

impl NodeElifStmnt {
    pub fn new(mut reader: LineReader) -> Result<Self, ChalError> {
        let mut header = reader.advance_reader()?;
        header.expect_exact(TokenKind::Keyword(Keyword::Elif))?;

        let cond_raw = header.advance_until(|tk| *tk == TokenKind::Special(Special::Colon))?;
        header.expect_exact(TokenKind::Special(Special::Colon))?;
        header.expect_exact(TokenKind::Newline)?;

        let cond = NodeExpr::new(cond_raw, reader.spanner())?;
        Ok(NodeElifStmnt {
            condition: cond,
            body: parse_body(reader)?,
        })
    }

    pub fn disassemble(self) -> (NodeExpr, Vec<NodeStmnt>) {
        (self.condition, self.body)
    }
}

impl NodeElseStmnt {
    pub fn new(mut reader: LineReader) -> Result<Self, ChalError> {
        let mut header = reader.advance_reader()?;
        header.expect_exact(TokenKind::Keyword(Keyword::Else))?;
        header.expect_exact(TokenKind::Special(Special::Colon))?;
        header.expect_exact(TokenKind::Newline)?;

        Ok(NodeElseStmnt {
            body: parse_body(reader)?,
        })
    }

    pub fn disassemble(self) -> Vec<NodeStmnt> {
        self.body
    }
}
