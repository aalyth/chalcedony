use crate::error::{ChalError, InternalError, LexerError, Position, Span};
use crate::lexer::{Delimiter, Keyword, Line, Special, TokenKind, Type};
use crate::parser::ast::{parse_body, NodeStmnt};
use crate::parser::{LineReader, TokenReader};

use std::collections::VecDeque;
use std::rc::Rc;

pub struct NodeFuncDef {
    name: String,
    args: Vec<(String, Type)>,
    ret_type: Type,
    body: Vec<NodeStmnt>,

    /* the positions refer to the function's header */
    start: Position,
    end: Position,
    span: Rc<Span>,
}

impl NodeFuncDef {
    pub fn new(chunk: VecDeque<Line>, span: Rc<Span>) -> Result<Self, ChalError> {
        /* function composition:
         * fn main() -> void:        | header
         *     let a = 5             > body
         *     print("Hello world")  > body
         */

        /* NOTE: this looks strange, but it's used to check wheater the indentations inside the
         * function body are correct */
        let mut reader = LineReader::new(chunk, span.clone());
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
            let start = header_src.tokens().front().unwrap().start();
            let end = header_src.tokens().front().unwrap().end();
            return Err(LexerError::invalid_indentation(start, end, span.clone()).into());
        }

        let mut header = TokenReader::new(header_src.into(), span.clone());
        let start = header.start();

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
        let end = header.end();
        header.expect_exact(TokenKind::Newline)?;

        Ok(NodeFuncDef {
            name,
            args,
            ret_type,
            body: parse_body(reader)?,
            start,
            end,
            span: header.span(),
        })
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn disassemble(
        self,
    ) -> (
        String,
        Vec<(String, Type)>,
        Type,
        Vec<NodeStmnt>,
        Position,
        Position,
        Rc<Span>,
    ) {
        (
            self.name,
            self.args,
            self.ret_type,
            self.body,
            self.start,
            self.end,
            self.span,
        )
    }
}
