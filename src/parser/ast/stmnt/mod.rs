mod assignment;
mod if_stmnt;
mod return_stmnt;
mod while_loop;

use assignment::NodeAssign;
use if_stmnt::NodeIfStmnt;
use return_stmnt::NodeRetStmnt;
use while_loop::NodeWhileLoop;

use super::{NodeFuncCall, NodeVarDef};

use crate::error::ChalError;
use crate::lexer::{Delimiter, Keyword, TokenKind};
use crate::parser::{LineReader, TokenReader};
use std::collections::VecDeque;

#[derive(Debug)]
pub enum NodeStmnt {
    VarDef(NodeVarDef),
    FuncCall(NodeFuncCall),
    Assign(NodeAssign),
    IfStmnt(NodeIfStmnt),
    WhileLoop(NodeWhileLoop),
    RetStmnt(NodeRetStmnt),
}

pub fn parse_body(mut reader: LineReader) -> Result<VecDeque<NodeStmnt>, ChalError> {
    let mut res = VecDeque::<NodeStmnt>::new();
    let mut err_vec = VecDeque::<ChalError>::new();

    while let Some(front) = reader.peek_tok() {
        match *front.kind() {
            TokenKind::Keyword(Keyword::Let) => {
                let tok_reader_raw = reader.advance_reader();
                let Ok(tok_reader) = tok_reader_raw else {
                    err_vec.push_back(tok_reader_raw.err().unwrap());
                    continue;
                };

                let node_raw = NodeVarDef::new(tok_reader);
                let Ok(node) = node_raw else {
                    err_vec.push_back(node_raw.err().unwrap());
                    continue;
                };

                res.push_back(NodeStmnt::VarDef(node));
            }

            TokenKind::Keyword(Keyword::Return) => {
                let tok_reader_raw = reader.advance_reader();
                let Ok(tok_reader) = tok_reader_raw else {
                    err_vec.push_back(tok_reader_raw.err().unwrap());
                    continue;
                };

                let node_raw = NodeRetStmnt::new(tok_reader);
                let Ok(node) = node_raw else {
                    err_vec.push_back(node_raw.err().unwrap());
                    continue;
                };

                res.push_back(NodeStmnt::RetStmnt(node));
            }

            TokenKind::Identifier(_) => {
                // TODO: throw error?
                let Some(line) = reader.advance() else {
                    continue;
                };

                if let Some(peek) = line.tokens().get(1) {
                    if *peek.kind() == TokenKind::Delimiter(Delimiter::OpenPar) {
                        let node_raw = NodeFuncCall::new(line.into(), reader.span().clone());
                        let Ok(node) = node_raw else {
                            err_vec.push_back(node_raw.err().unwrap());
                            continue;
                        };
                        res.push_back(NodeStmnt::FuncCall(node));
                        continue;
                    }
                }

                let token_reader = TokenReader::new(line.into(), reader.span().clone());
                let node_raw = NodeAssign::new(token_reader);
                let Ok(node) = node_raw else {
                    err_vec.push_back(node_raw.err().unwrap());
                    continue;
                };

                res.push_back(NodeStmnt::Assign(node));
            }

            TokenKind::Keyword(Keyword::If) => {
                let line_reader_raw = reader.advance_chunk();
                let Ok(line_reader) = line_reader_raw else {
                    err_vec.push_back(line_reader_raw.err().unwrap());
                    continue;
                };

                let node_raw = NodeIfStmnt::new(line_reader);
                let Ok(node) = node_raw else {
                    err_vec.push_back(node_raw.err().unwrap());
                    continue;
                };

                res.push_back(NodeStmnt::IfStmnt(node));
            }

            TokenKind::Keyword(Keyword::While) => {
                let line_reader_raw = reader.advance_chunk();
                let Ok(line_reader) = line_reader_raw else {
                    err_vec.push_back(line_reader_raw.err().unwrap());
                    continue;
                };

                let node_raw = NodeWhileLoop::new(line_reader);
                let Ok(node) = node_raw else {
                    err_vec.push_back(node_raw.err().unwrap());
                    continue;
                };

                res.push_back(NodeStmnt::WhileLoop(node));
            }

            // TODO: proper errors
            _ => return Err(ChalError::from(err_vec)),
        }
    }

    if !err_vec.is_empty() {
        return Err(ChalError::from(err_vec));
    }
    Ok(res)
}
