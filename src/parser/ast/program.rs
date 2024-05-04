use crate::error::span::{Span, Spanning};
use crate::error::{ChalError, ParserError, ParserErrorKind};
use crate::lexer::{Delimiter, Operator};
use crate::lexer::{Keyword, Line, TokenKind};
use crate::parser::ast::{
    NodeAssign, NodeFuncCall, NodeFuncDef, NodeIfStmnt, NodeVarDef, NodeWhileLoop,
};

use crate::parser::{LineReader, TokenReader};

use std::collections::VecDeque;
use std::rc::Rc;

use super::NodeTryCatch;

/// A node in the program, representing an interpretable global unit, i.e. any
/// statement that could be executed in the global context.
///
/// For syntax refer to each individual node.
#[derive(Debug)]
pub enum NodeProg {
    VarDef(NodeVarDef),
    FuncDef(NodeFuncDef),
    FuncCall(NodeFuncCall),
    Assign(NodeAssign),
    IfStmnt(NodeIfStmnt),
    WhileLoop(NodeWhileLoop),
    TryCatch(NodeTryCatch),
}

/* a wrapper for building a node from a single line statement */
macro_rules! single_line_stmnt {
    ( $enum_type: ident, $node_type: ident, $chunk: ident, $spanner: ident) => {{
        /* SAFETY: the front line is already checked */
        let front_line = $chunk.pop_front().unwrap().into();
        Ok(NodeProg::$enum_type($node_type::new(TokenReader::new(
            front_line,
            Span::from($spanner),
        ))?))
    }};
}

/* a wrapper for building a node from a multiline statement */
macro_rules! multiline_stmnt {
    ( $enum_type: ident, $node_type: ident, $chunk: ident, $spanner: ident) => {{
        Ok(NodeProg::$enum_type($node_type::new(LineReader::new(
            $chunk, $spanner,
        ))?))
    }};
}

impl NodeProg {
    pub fn new(mut chunk: VecDeque<Line>, spanner: Rc<dyn Spanning>) -> Result<Self, ChalError> {
        if chunk.is_empty() {
            panic!("NodeProg::new(): received an empty code chunk");
        }

        let front_line = chunk.front().unwrap();
        if front_line.tokens.is_empty() {
            panic!("NodeProg::new(): empty first line of chunk");
        }

        let front_tok = front_line.front_tok().unwrap();

        match front_tok.kind {
            TokenKind::Keyword(Keyword::Let) => {
                single_line_stmnt!(VarDef, NodeVarDef, chunk, spanner)
            }
            TokenKind::Keyword(Keyword::Fn) => {
                multiline_stmnt!(FuncDef, NodeFuncDef, chunk, spanner)
            }
            TokenKind::Keyword(Keyword::If) => {
                multiline_stmnt!(IfStmnt, NodeIfStmnt, chunk, spanner)
            }
            TokenKind::Keyword(Keyword::While) => {
                multiline_stmnt!(WhileLoop, NodeWhileLoop, chunk, spanner)
            }
            TokenKind::Keyword(Keyword::Try) => {
                multiline_stmnt!(TryCatch, NodeTryCatch, chunk, spanner)
            }

            TokenKind::Identifier(_) => {
                let Some(peek_2nd) = front_line.tokens.get(1) else {
                    /* by deafult a function call is expected upon encountering an identifier  */
                    return Err(ParserError::new(
                        ParserErrorKind::ExpectedToken(TokenKind::Delimiter(Delimiter::OpenPar)),
                        front_tok.span.clone(),
                    )
                    .into());
                };

                match &peek_2nd.kind {
                    /* a function call */
                    TokenKind::Delimiter(Delimiter::OpenPar) => {
                        single_line_stmnt!(FuncCall, NodeFuncCall, chunk, spanner)
                    }
                    /* an assignment */
                    TokenKind::Operator(Operator::Eq)
                    | TokenKind::Operator(Operator::AddEq)
                    | TokenKind::Operator(Operator::SubEq)
                    | TokenKind::Operator(Operator::MulEq)
                    | TokenKind::Operator(Operator::DivEq)
                    | TokenKind::Operator(Operator::ModEq) => {
                        single_line_stmnt!(Assign, NodeAssign, chunk, spanner)
                    }
                    recv_kind => Err(ParserError::new(
                        ParserErrorKind::InvalidToken(
                            TokenKind::Delimiter(Delimiter::OpenPar),
                            recv_kind.clone(),
                        ),
                        peek_2nd.span.clone(),
                    )
                    .into()),
                }
            }

            _ => panic!(
                "NodeProg::new(): invalid chunk front - {:?}",
                front_tok.kind
            ),
        }
    }
}
