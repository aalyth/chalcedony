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

use super::{NodeAttrRes, NodeFuncCallStmnt, NodeVarDef};

use crate::error::{span::Span, ChalError, ParserError, ParserErrorKind};
use crate::lexer::{Keyword, TokenKind};
use crate::parser::{LineReader, TokenReader};

/// The node representing a single statement in the program. A statement is a
/// code unit which does not result in a value.
///
/// For syntax refer to individual nodes.
#[derive(Debug, PartialEq)]
pub enum NodeStmnt {
    VarDef(NodeVarDef),
    FuncCall(NodeFuncCallStmnt),
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
        match $node_type::new($reader.advance_reader()) {
            Ok(node) => $result.push(NodeStmnt::$stmnt_type(node)),
            Err(err) => $errors.push(err),
        }
    }};
}

macro_rules! multiline_statement {
    ($reader:ident, $result:ident, $errors:ident, $node_type:ident, $stmnt_type:ident) => {{
        let line_reader = match $reader.advance_chunk() {
            Ok(reader) => reader,
            Err(err) => {
                $errors.push(err);
                continue;
            }
        };

        match $node_type::new(line_reader) {
            Ok(node) => $result.push(NodeStmnt::$stmnt_type(node)),
            Err(err) => $errors.push(err),
        }
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

                TokenKind::Keyword(Keyword::Try) => {
                    multiline_statement!(reader, result, errors, NodeTryCatch, TryCatch);
                }

                /* Function calls and assignments */
                TokenKind::Identifier(_) => {
                    let mut token_reader = reader.advance_reader();
                    let resolution = NodeAttrRes::new(&mut token_reader)?;

                    /* there is a single resolution on the line, so try to parse it as func call */
                    if token_reader.peek_is_exact(TokenKind::Newline) {
                        match NodeFuncCallStmnt::try_from(resolution) {
                            Ok(node) => result.push(NodeStmnt::FuncCall(node)),
                            Err(err) => errors.push(err),
                        }
                        continue;
                    }

                    match NodeAssign::new(resolution, token_reader) {
                        Ok(node) => result.push(NodeStmnt::Assign(node)),
                        Err(err) => errors.push(err),
                    }
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
