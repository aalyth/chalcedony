use crate::error::span::Span;
use crate::error::{ChalError, InternalError, LexerError};
use crate::lexer::{Delimiter, Keyword, Special, TokenKind};
use crate::parser::ast::NodeStmnt;
use crate::parser::{LineReader, TokenReader};

use crate::common::Type;

pub struct NodeFuncDef {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub ret_type: Type,
    pub body: Vec<NodeStmnt>,

    /* the span refers to the function's header */
    pub span: Span,
}

impl NodeFuncDef {
    pub fn new(mut reader: LineReader) -> Result<Self, ChalError> {
        /* function composition:
         * fn main() -> void:        | header
         *     let a = 5             > body
         *     print("Hello world")  > body
         */

        /* NOTE: this looks strange, but it's used to check wheater the indentations inside the
         * function body are correct */
        let mut reader = reader.advance_chunk()?;

        let Some(header_src) = reader.advance() else {
            return Err(InternalError::new(
                "NodeFuncDef::new(): creating a function definiton from empty source",
            )
            .into());
        };

        if header_src.tokens().is_empty() {
            return Err(InternalError::new(
                "NodeFuncDef::new(): creating a function definiton with empty source tokens",
            )
            .into());
        }

        /* NOTE: remove this if nested function definitions become allowed */
        if header_src.indent() != 0 {
            let front_tok = header_src.tokens().front().unwrap();
            return Err(LexerError::invalid_indentation(front_tok.span.clone()).into());
        }

        let mut header = TokenReader::new(header_src.into(), reader.spanner());
        let start = header.current().start;

        header.expect_exact(TokenKind::Keyword(Keyword::Fn))?;

        let name = header.expect_ident()?;

        header.expect_exact(TokenKind::Delimiter(Delimiter::OpenPar))?;

        let mut args = Vec::<(String, Type)>::new();
        let mut first_iter = true;
        while !header.peek_is_exact(TokenKind::Delimiter(Delimiter::ClosePar)) {
            if !first_iter {
                header.expect_exact(TokenKind::Special(Special::Comma))?;
            }

            let argname = header.expect_ident()?;
            header.expect_exact(TokenKind::Special(Special::Colon))?;
            let argkind = header.expect_type()?;

            args.push((argname, argkind));

            first_iter = false;
        }

        header.expect_exact(TokenKind::Delimiter(Delimiter::ClosePar))?;

        let mut ret_type = Type::Void;
        if header.peek_is_exact(TokenKind::Special(Special::RightArrow)) {
            /* this should never fail */
            header.expect_exact(TokenKind::Special(Special::RightArrow))?;
            ret_type = header.expect_type()?;
        }

        header.expect_exact(TokenKind::Special(Special::Colon))?;
        let end = header.current().end;
        header.expect_exact(TokenKind::Newline)?;

        let span = Span::new(start, end, reader.spanner());
        Ok(NodeFuncDef {
            name,
            args,
            ret_type,
            body: reader.try_into()?,
            span,
        })
    }
}
