use crate::error::{ChalError, InternalError, LexerError, Span};
use crate::lexer::{Delimiter, Keyword, Line, Special, TokenKind, Type};
use crate::parser::ast::{parse_body, NodeStmnt};
use crate::parser::{LineReader, TokenReader};

use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
pub struct NodeFuncDef {
    name: String,
    args: Vec<(String, Type)>,
    ret_type: Type,
    body: Vec<NodeStmnt>,
}

impl NodeFuncDef {
    pub fn new(chunk: VecDeque<Line>, span: Rc<Span>) -> Result<Self, ChalError> {
        /* function composition:
         * fn main() -> void:        | header
         *     let a := 5            > body
         *     print("Hello world")  > body
         */
        let mut reader = LineReader::new(chunk, span.clone());
        let Some(header_src) = reader.advance() else {
            return Err(ChalError::from(InternalError::new(
                "NodeFuncDef::new(): creating a function definiton from empty source",
            )));
        };

        if header_src.tokens().is_empty() {
            return Err(ChalError::from(InternalError::new(
                "NodeFuncDef::new(): creating a function definiton with empty source tokens",
            )));
        }

        /* NOTE: remove this if nested function definitions become allowed */
        if header_src.indent() != 0 {
            let start = header_src.tokens().front().unwrap().start();
            let end = header_src.tokens().front().unwrap().end();
            return Err(ChalError::from(LexerError::invalid_indentation(
                start,
                end,
                span.clone(),
            )));
        }

        let mut header = TokenReader::new(header_src.into(), span.clone());

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

        /* TODO: uncomment
        header.expect_exact(TokenKind::Special(Special::Colon))?;
        header.expect_exact(TokenKind::Newline)?;
        */

        Ok(NodeFuncDef {
            name,
            args,
            ret_type,
            body: parse_body(reader)?,
        })
    }

    pub fn disassemble(self) -> (String, Vec<(String, Type)>, Type, Vec<NodeStmnt>) {
        (self.name, self.args, self.ret_type, self.body)
    }
}
