mod assignment;
mod exceptions;
mod if_stmnt;
mod return_stmnt;
mod while_loop;

pub use assignment::NodeAssign;
pub use exceptions::{NodeThrow, NodeTryCatch};
pub use if_stmnt::{NodeElifStmnt, NodeElseStmnt, NodeIfBranch, NodeIfStmnt};
pub use return_stmnt::NodeRetStmnt;
pub use while_loop::NodeWhileLoop;

use super::{NodeFuncCall, NodeVarDef};

use crate::error::{span::Span, ChalError, ParserError, ParserErrorKind};
use crate::lexer::{Delimiter, Keyword, TokenKind};
use crate::parser::{LineReader, TokenReader};

/// The node representing a single statement in the program. A statement is a
/// code unit which does not result in a value.
///
/// For syntax refer to individual nodes.
#[derive(Debug, PartialEq)]
pub enum NodeStmnt {
    VarDef(NodeVarDef),
    FuncCall(NodeFuncCall),
    Assign(NodeAssign),
    RetStmnt(NodeRetStmnt),

    IfStmnt(NodeIfStmnt),
    WhileLoop(NodeWhileLoop),
    ContStmnt(NodeContStmnt),
    BreakStmnt(NodeBreakStmnt),

    TryCatch(NodeTryCatch),
    Throw(NodeThrow),
}

/// Boils down to the `TokenKind::Keyword(Keyword::Continue)`. Can only be used
/// in the context of a loop.
#[derive(Debug, PartialEq)]
pub struct NodeContStmnt {
    pub span: Span,
}

/// Boils down to the `TokenKind::Keyword(Keyword::Break)`. Can only be used in
/// the context of a loop.
#[derive(Debug, PartialEq)]
pub struct NodeBreakStmnt {
    pub span: Span,
}

macro_rules! single_line_statement {
    ($reader:ident, $result:ident, $errors:ident, $node_type:ident, $stmnt_type:ident) => {{
        let tok_reader_raw = $reader.advance_reader();
        let Ok(tok_reader) = tok_reader_raw else {
            $errors.push(tok_reader_raw.err().unwrap());
            continue;
        };

        let node_raw = $node_type::new(tok_reader);
        let Ok(node) = node_raw else {
            $errors.push(node_raw.err().unwrap());
            continue;
        };

        $result.push(NodeStmnt::$stmnt_type(node));
    }};
}

macro_rules! multiline_statement {
    ($reader:ident, $result:ident, $errors:ident, $node_type:ident, $stmnt_type:ident) => {{
        let line_reader_raw = $reader.advance_chunk();
        let Ok(line_reader) = line_reader_raw else {
            $errors.push(line_reader_raw.err().unwrap());
            continue;
        };

        let node_raw = $node_type::new(line_reader);
        let Ok(node) = node_raw else {
            $errors.push(node_raw.err().unwrap());
            continue;
        };

        $result.push(NodeStmnt::$stmnt_type(node));
    }};
}

impl TryFrom<LineReader> for Vec<NodeStmnt> {
    type Error = ChalError;

    fn try_from(mut reader: LineReader) -> Result<Self, Self::Error> {
        let mut result = Vec::<NodeStmnt>::new();
        let mut errors = Vec::<ChalError>::new();

        while let Some(front) = reader.peek_tok() {
            match front.kind {
                /* Single line statements */
                TokenKind::Keyword(Keyword::Let) => {
                    single_line_statement!(reader, result, errors, NodeVarDef, VarDef);
                }
                TokenKind::Keyword(Keyword::Return) => {
                    single_line_statement!(reader, result, errors, NodeRetStmnt, RetStmnt);
                }
                TokenKind::Keyword(Keyword::Continue) => {
                    single_line_statement!(reader, result, errors, NodeContStmnt, ContStmnt);
                }
                TokenKind::Keyword(Keyword::Break) => {
                    single_line_statement!(reader, result, errors, NodeBreakStmnt, BreakStmnt);
                }

                TokenKind::Keyword(Keyword::Throw) => {
                    single_line_statement!(reader, result, errors, NodeThrow, Throw);
                }

                /* Multiline statements  */
                TokenKind::Keyword(Keyword::If) => {
                    multiline_statement!(reader, result, errors, NodeIfStmnt, IfStmnt);
                }

                TokenKind::Keyword(Keyword::While) => {
                    multiline_statement!(reader, result, errors, NodeWhileLoop, WhileLoop);
                }

                TokenKind::Keyword(Keyword::Catch) => {
                    multiline_statement!(reader, result, errors, NodeTryCatch, TryCatch);
                }

                /* Function calls and assignments */
                TokenKind::Identifier(_) => {
                    let Some(line) = reader.advance() else {
                        panic!("NodeStmnt::parse_body(): could not advance a peeked reader")
                    };

                    // check whether the identifier should be treated as a function
                    // call or a variable assignment. SAFETY: there are always at
                    // least 2 elements in the line (the identifer + newline)
                    if let Some(peek) = line.tokens.get(1) {
                        if peek.kind == TokenKind::Delimiter(Delimiter::OpenPar) {
                            let node_reader =
                                TokenReader::new(line.into(), Span::from(reader.spanner()));
                            let node_raw = NodeFuncCall::new(node_reader);
                            let Ok(node) = node_raw else {
                                errors.push(node_raw.err().unwrap());
                                continue;
                            };
                            result.push(NodeStmnt::FuncCall(node));
                            continue;
                        }
                    }

                    let token_reader = TokenReader::new(line.into(), Span::from(reader.spanner()));
                    let node_raw = NodeAssign::new(token_reader);
                    let Ok(node) = node_raw else {
                        errors.push(node_raw.err().unwrap());
                        continue;
                    };

                    result.push(NodeStmnt::Assign(node));
                }

                _ => {
                    let front = front.clone();
                    reader.advance();
                    errors.push(
                        ParserError::new(ParserErrorKind::InvalidStatement, front.span).into(),
                    )
                }
            }
        }

        if !errors.is_empty() {
            return Err(errors.into());
        }
        Ok(result)
    }
}

impl NodeContStmnt {
    fn new(mut reader: TokenReader) -> Result<Self, ChalError> {
        reader.expect_exact(TokenKind::Keyword(Keyword::Continue))?;
        let span = reader.current();
        reader.expect_exact(TokenKind::Newline)?;

        Ok(NodeContStmnt { span })
    }
}

impl NodeBreakStmnt {
    fn new(mut reader: TokenReader) -> Result<Self, ChalError> {
        reader.expect_exact(TokenKind::Keyword(Keyword::Break))?;
        let span = reader.current();
        reader.expect_exact(TokenKind::Newline)?;

        Ok(NodeBreakStmnt { span })
    }
}
