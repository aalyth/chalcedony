mod assignment;
mod if_stmnt;
pub mod return_stmnt;
mod while_loop;

pub use assignment::NodeAssign;
pub use if_stmnt::{NodeElifStmnt, NodeElseStmnt, NodeIfBranch, NodeIfStmnt};
pub use return_stmnt::NodeRetStmnt;
pub use while_loop::NodeWhileLoop;

use super::{NodeFuncCall, NodeVarDef};

use crate::error::{ChalError, InternalError, ParserError};
use crate::lexer::{Delimiter, Keyword, TokenKind};
use crate::parser::{LineReader, TokenReader};

pub enum NodeStmnt {
    VarDef(NodeVarDef),
    FuncCall(NodeFuncCall),
    Assign(NodeAssign),
    IfStmnt(NodeIfStmnt),
    WhileLoop(NodeWhileLoop),
    RetStmnt(NodeRetStmnt),
}

impl TryFrom<LineReader> for Vec<NodeStmnt> {
    type Error = ChalError;

    fn try_from(mut reader: LineReader) -> Result<Self, Self::Error> {
        let mut res = Vec::<NodeStmnt>::new();
        let mut err_vec = Vec::<ChalError>::new();

        while let Some(front) = reader.peek_tok() {
            match front.kind {
                TokenKind::Keyword(Keyword::Let) => {
                    let tok_reader_raw = reader.advance_reader();
                    let Ok(tok_reader) = tok_reader_raw else {
                        err_vec.push(tok_reader_raw.err().unwrap());
                        continue;
                    };

                    let node_raw = NodeVarDef::new(tok_reader);
                    let Ok(node) = node_raw else {
                        err_vec.push(node_raw.err().unwrap());
                        continue;
                    };

                    res.push(NodeStmnt::VarDef(node));
                }

                TokenKind::Keyword(Keyword::Return) => {
                    let tok_reader_raw = reader.advance_reader();
                    let Ok(tok_reader) = tok_reader_raw else {
                        err_vec.push(tok_reader_raw.err().unwrap());
                        continue;
                    };

                    let node_raw = NodeRetStmnt::new(tok_reader);
                    let Ok(node) = node_raw else {
                        err_vec.push(node_raw.err().unwrap());
                        continue;
                    };

                    res.push(NodeStmnt::RetStmnt(node));
                }

                TokenKind::Identifier(_) => {
                    let Some(line) = reader.advance() else {
                        return Err(InternalError::new(
                            "NodeStmnt::parse_body(): could not advance a peeked reader",
                        )
                        .into());
                    };

                    // SAFETY: there is always at least 2 elements in the line (the identifer + newline)
                    if let Some(peek) = line.tokens().get(1) {
                        if peek.kind == TokenKind::Delimiter(Delimiter::OpenPar) {
                            let node_reader = TokenReader::new(line.into(), reader.spanner());
                            let node_raw = NodeFuncCall::new(node_reader);
                            let Ok(node) = node_raw else {
                                err_vec.push(node_raw.err().unwrap());
                                continue;
                            };
                            res.push(NodeStmnt::FuncCall(node));
                            continue;
                        }
                    }

                    let token_reader = TokenReader::new(line.into(), reader.spanner().clone());
                    let node_raw = NodeAssign::new(token_reader);
                    let Ok(node) = node_raw else {
                        err_vec.push(node_raw.err().unwrap());
                        continue;
                    };

                    res.push(NodeStmnt::Assign(node));
                }

                TokenKind::Keyword(Keyword::If) => {
                    let line_reader_raw = reader.advance_chunk();
                    let Ok(line_reader) = line_reader_raw else {
                        err_vec.push(line_reader_raw.err().unwrap());
                        continue;
                    };

                    let node_raw = NodeIfStmnt::new(line_reader);
                    let Ok(node) = node_raw else {
                        err_vec.push(node_raw.err().unwrap());
                        continue;
                    };

                    res.push(NodeStmnt::IfStmnt(node));
                }

                TokenKind::Keyword(Keyword::While) => {
                    let line_reader_raw = reader.advance_chunk();
                    let Ok(line_reader) = line_reader_raw else {
                        err_vec.push(line_reader_raw.err().unwrap());
                        continue;
                    };

                    let node_raw = NodeWhileLoop::new(line_reader);
                    let Ok(node) = node_raw else {
                        err_vec.push(node_raw.err().unwrap());
                        continue;
                    };

                    res.push(NodeStmnt::WhileLoop(node));
                }

                _ => {
                    let front = front.clone();
                    reader.advance();
                    err_vec.push(ParserError::invalid_statement(front.span).into())
                }
            }
        }

        if !err_vec.is_empty() {
            return Err(err_vec.into());
        }
        Ok(res)
    }
}
