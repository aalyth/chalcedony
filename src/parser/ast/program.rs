use crate::error::span::{Span, Spanning};
use crate::error::ChalError;
use crate::lexer::{Keyword, Line, TokenKind};
use crate::parser::ast::{
    NodeAssign, NodeClass, NodeForLoop, NodeFuncDef, NodeIfStmnt, NodeVarDef, NodeWhileLoop,
};

use crate::parser::{LineReader, TokenReader};

use std::collections::VecDeque;
use std::rc::Rc;

use super::{NodeAttrRes, NodeFuncCallStmnt, NodeTryCatch};

/// A node in the program, representing an interpretable global unit, i.e. any
/// statement that could be executed in the global context.
///
/// For syntax refer to each individual node.
#[derive(Debug)]
pub enum NodeProg {
    VarDef(NodeVarDef),
    FuncDef(NodeFuncDef),
    FuncCall(NodeFuncCallStmnt),
    Assign(NodeAssign),
    IfStmnt(NodeIfStmnt),
    WhileLoop(NodeWhileLoop),
    ForLoop(NodeForLoop),
    TryCatch(NodeTryCatch),
    Import(NodeImport),
    Class(NodeClass),
}

/// The node denoting the import of another script.
///
/// Syntax:
/// `import` \<path\>
///
/// where `<path>` is a string literal
#[derive(Debug)]
pub struct NodeImport {
    pub path: String,
    pub span: Span,
}

impl NodeImport {
    pub fn new(mut reader: TokenReader) -> Result<Self, ChalError> {
        let start = reader.current().start;
        reader.expect_exact(TokenKind::Keyword(Keyword::Import))?;

        let TokenKind::Str(path) = reader.expect(TokenKind::Str(String::new()))?.kind else {
            unreachable!()
        };
        let end = reader.current().end;

        Ok(NodeImport {
            path,
            span: Span::new(start, end, reader.spanner()),
        })
    }
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
            TokenKind::Keyword(Keyword::Let) | TokenKind::Keyword(Keyword::Const) => {
                single_line_stmnt!(VarDef, NodeVarDef, chunk, spanner)
            }
            TokenKind::Keyword(Keyword::Import) => {
                single_line_stmnt!(Import, NodeImport, chunk, spanner)
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
            TokenKind::Keyword(Keyword::For) => {
                multiline_stmnt!(ForLoop, NodeForLoop, chunk, spanner)
            }
            TokenKind::Keyword(Keyword::Class) => {
                multiline_stmnt!(Class, NodeClass, chunk, spanner)
            }

            TokenKind::Identifier(_) => {
                let mut reader =
                    TokenReader::new(front_line.tokens.clone(), front_tok.span.clone());
                let resolution = NodeAttrRes::new(&mut reader)?;

                if reader.peek_is_exact(TokenKind::Newline) {
                    return Ok(NodeProg::FuncCall(NodeFuncCallStmnt::try_from(resolution)?));
                }

                Ok(NodeProg::Assign(NodeAssign::new(resolution, reader)?))
            }

            _ => panic!(
                "NodeProg::new(): invalid chunk front - {:?}",
                front_tok.kind
            ),
        }
    }
}
