use crate::error::span::Span;
use crate::error::ChalError;
use crate::lexer::{Delimiter, Keyword, Special, TokenKind};
use crate::parser::ast::NodeStmnt;
use crate::parser::LineReader;

use crate::common::Type;

#[derive(Debug, PartialEq)]
pub struct Arg {
    pub name: String,
    pub ty: Type,
}

/// The node representing the creation of a function. The span refers to the function's
/// header, i.e. the first line of the definition.
///
/// Syntax:
/// `fn` \<func_name\>(\<arg\>: \<type\>, ...) -> \<type\>:
///     \<statements\>
///
/// Syntax for implicitly infered `void` return type:
/// `fn` \<func_name\>(\<arg\>: \<type\>, ...):
///     \<statements\>
///
#[derive(Debug, PartialEq)]
pub struct NodeFuncDef {
    pub name: String,
    pub args: Vec<Arg>,
    pub ret_type: Type,
    pub body: Vec<NodeStmnt>,
    pub span: Span,
}

impl NodeFuncDef {
    pub fn new(mut reader: LineReader) -> Result<Self, ChalError> {
        // header refers to the first line of the function, for example:
        // fn fib(n: int) -> uint:             | header
        //     if n > 2:                       > body
        //         return fib(n-2) + fib(n-1)  > body
        //     return 1                        > body

        let mut header = reader.advance_reader()?;
        let start = header.current().start;

        header.expect_exact(TokenKind::Keyword(Keyword::Fn))?;
        let name = header.expect_ident()?;
        header.expect_exact(TokenKind::Delimiter(Delimiter::OpenPar))?;

        /* `first_iter` is used to check for proper use of comma separators */
        let mut args = Vec::<Arg>::new();
        let mut first_iter = true;
        while !header.peek_is_exact(TokenKind::Delimiter(Delimiter::ClosePar)) {
            if !first_iter {
                header.expect_exact(TokenKind::Special(Special::Comma))?;
            }

            let name = header.expect_ident()?;
            header.expect_exact(TokenKind::Special(Special::Colon))?;
            let ty = header.expect_type()?;

            args.push(Arg { name, ty });
            first_iter = false;
        }

        header.expect_exact(TokenKind::Delimiter(Delimiter::ClosePar))?;

        let mut ret_type = Type::Void;
        if header.peek_is_exact(TokenKind::Special(Special::RightArrow)) {
            /* pop the right arrow */
            header.advance();
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
