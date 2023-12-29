use crate::error::{ChalError, InternalError};
use crate::lexer::{Keyword, Special, TokenKind};
use crate::parser::ast::{NodeExpr, NodeStmnt};

use crate::parser::LineReader;

use super::parse_body;

pub struct NodeIfStmnt {
    condition: NodeExpr,
    body: Vec<NodeStmnt>,
    branches: Vec<NodeIfBranch>,
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
        let condition = NodeExpr::new(cond_raw, reader.span())?;

        header.expect_exact(TokenKind::Special(Special::Colon))?;
        header.expect_exact(TokenKind::Newline)?;

        let body = reader.advance_until(|ln| {
            let Some(front) = ln.front_tok() else {
                return false;
            };
            *front.kind() == TokenKind::Keyword(Keyword::Else)
                || *front.kind() == TokenKind::Keyword(Keyword::Elif)
        })?;

        let mut branches = Vec::<NodeIfBranch>::new();
        // TODO: trhow error when there is a branch after an else
        while !reader.is_empty() {
            let next_branch = reader.advance_until(|ln| {
                let Some(front) = ln.front_tok() else {
                    return false;
                };
                *front.kind() == TokenKind::Keyword(Keyword::Elif)
                    || *front.kind() == TokenKind::Keyword(Keyword::Else)
            })?;
            branches.push(NodeIfBranch::new(LineReader::new(
                next_branch,
                reader.span(),
            ))?);
        }

        Ok(NodeIfStmnt {
            condition,
            branches,
            body: parse_body(LineReader::new(body, reader.span()))?,
        })
    }

    pub fn disassemble(self) -> (NodeExpr, Vec<NodeStmnt>, Vec<NodeIfBranch>) {
        (self.condition, self.body, self.branches)
    }
}

impl NodeIfBranch {
    pub fn new(reader: LineReader) -> Result<Self, ChalError> {
        let Some(front_tok) = reader.peek_tok() else {
            return Err(InternalError::new("NodeIFBranch::new(): generating ").into());
        };

        match front_tok.kind() {
            TokenKind::Keyword(Keyword::Elif) => {
                Ok(NodeIfBranch::Elif(NodeElifStmnt::new(reader)?))
            }
            TokenKind::Keyword(Keyword::Else) => {
                Ok(NodeIfBranch::Else(NodeElseStmnt::new(reader)?))
            }
            // TODO: check weather this should be an internal or parser error
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

        let cond = NodeExpr::new(cond_raw, reader.span())?;
        Ok(NodeElifStmnt {
            condition: cond,
            body: parse_body(reader)?,
        })
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
}
