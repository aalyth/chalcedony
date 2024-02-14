use crate::error::{ChalError, InternalError};
use crate::lexer::{Keyword, Special, TokenKind};
use crate::parser::ast::{NodeExpr, NodeStmnt};

use crate::parser::{LineReader, TokenReader};

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
    pub condition: NodeExpr,
    pub body: Vec<NodeStmnt>,
}

pub struct NodeElseStmnt {
    pub body: Vec<NodeStmnt>,
}

impl NodeIfStmnt {
    pub fn new(mut reader: LineReader) -> Result<Self, ChalError> {
        /* if statement structure:
         * if n % 3 == 0:    | header
         *     print(n)      > body
         *     count += 1    > body
         */
        let mut header = reader.advance_reader()?;
        header.expect_exact(TokenKind::Keyword(Keyword::If))?;

        let cond_raw = header.advance_until(|tk| {
            *tk == TokenKind::Special(Special::Colon) || *tk == TokenKind::Newline
        })?;
        let cond_reader = TokenReader::new(cond_raw, reader.spanner());
        let condition = NodeExpr::new(cond_reader)?;

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
            body: LineReader::new(body, reader.spanner()).try_into()?,
        })
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

        let cond_raw = header.advance_until(|tk| {
            *tk == TokenKind::Special(Special::Colon) || *tk == TokenKind::Newline
        })?;
        header.expect_exact(TokenKind::Special(Special::Colon))?;
        header.expect_exact(TokenKind::Newline)?;

        let cond_reader = TokenReader::new(cond_raw, reader.spanner());
        let cond = NodeExpr::new(cond_reader)?;
        Ok(NodeElifStmnt {
            condition: cond,
            body: reader.try_into()?,
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
            body: reader.try_into()?,
        })
    }
}
