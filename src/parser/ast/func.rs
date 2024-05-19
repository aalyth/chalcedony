use crate::error::{span::Span, ChalError, ParserError, ParserErrorKind};
use crate::lexer::{Delimiter, Keyword, Special, Token, TokenKind};
use crate::parser::ast::{NodeAttrRes, NodeAttribute, NodeExpr, NodeStmnt};
use crate::parser::{LineReader, TokenReader};

use crate::common::Type;

use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct Arg {
    pub name: String,
    pub ty: Type,
}

/// The node representing the creation of a function. The span refers to the function's
/// header, i.e. the first line of the definition.
///
/// Syntax:
/// `fn` \<func-name\>(\<arg\>: \<type\>, ...) -> \<type\>:
///     \<statements\>
///
/// Syntax for implicitly infered `void` return type:
/// `fn` \<func-name\>(\<arg\>: \<type\>, ...):
///     \<statements\>
///
/// Syntax as a method:
/// `class` <class-name>:
///     `fn` \<func-name\>(\<arg\>: \<type\>, ...) -> \<type\>:
///         \<statements\>
///
///     `fn` \<func-name\>(\<arg\>: \<type\>, ...):
///         \<statements\>
///
#[derive(Debug, PartialEq)]
pub struct NodeFuncDef {
    pub name: String,
    pub args: VecDeque<Arg>,
    pub ret_type: Type,
    pub body: Vec<NodeStmnt>,
    pub span: Span,

    // if the function is as a method inside a class namespace
    pub namespace: Option<String>,
}

/// The node representing a function call. The `span` field refers to the whole
/// function call from the function name to the closing parenthesis.
///
/// Syntax:
/// \<func-name\>(\<expr\>, \<expr\>, ...)
/// \<class-name\>::\<func-name\>(\<expr\>, \<expr\>, ...)
#[derive(Clone, Debug, PartialEq)]
pub struct NodeFuncCall {
    pub name: String,
    pub args: Vec<NodeExpr>,
    pub span: Span,

    // if the function is called as a method from the parent (class) namespace
    pub namespace: Option<String>,
}

/// A wrapper, used to guarantee that the attribute resolution properly ends
/// with a function call node.
#[derive(Debug, PartialEq)]
pub struct NodeFuncCallStmnt(pub NodeAttrRes);

impl NodeFuncDef {
    pub fn new(reader: LineReader) -> Result<Self, ChalError> {
        Self::parse(reader, None)
    }

    pub fn method(reader: LineReader, class: String) -> Result<Self, ChalError> {
        Self::parse(reader, Some(class))
    }

    fn parse(mut reader: LineReader, namespace: Option<String>) -> Result<Self, ChalError> {
        // header refers to the first line of the function, for example:
        // fn fib(n: int) -> uint:             | header
        //     if n > 2:                       > body
        //         return fib(n-2) + fib(n-1)  > body
        //     return 1                        > body

        let mut header = reader.advance_reader();
        let start = header.current().start;

        header.expect_exact(TokenKind::Keyword(Keyword::Fn))?;
        let name = header.expect_ident()?;
        header.expect_exact(TokenKind::Delimiter(Delimiter::OpenPar))?;

        /* `first_iter` is used to check for proper use of comma separators */
        let mut args = VecDeque::<Arg>::new();
        let mut first_iter = true;
        while !header.peek_is_exact(TokenKind::Delimiter(Delimiter::ClosePar)) {
            if !first_iter {
                header.expect_exact(TokenKind::Special(Special::Comma))?;
            }

            let name = header.expect_ident()?;
            /* if the first argument of a method is `self`, the type could be implied */
            let ty: Type = if namespace.is_some()
                && first_iter
                && &name == "self"
                && !header.peek_is_exact(TokenKind::Special(Special::Colon))
            {
                Type::Custom(Box::new(namespace.clone().unwrap()))
            } else {
                header.expect_exact(TokenKind::Special(Special::Colon))?;
                header.expect_type()?
            };

            args.push_back(Arg { name, ty });
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
            namespace,
        })
    }
}

impl NodeFuncCall {
    pub fn new(mut reader: TokenReader) -> Result<Self, ChalError> {
        let start = reader.current().start;
        let mut namespace: Option<String> = None;

        if let Some(tok) = reader.peek_nth(1) {
            if tok.kind == TokenKind::Special(Special::Resolution) {
                namespace = Some(reader.expect_ident()?);
                reader.expect_exact(TokenKind::Special(Special::Resolution))?;
            }
        }

        let name = reader.expect_ident()?;
        reader.expect_exact(TokenKind::Delimiter(Delimiter::OpenPar))?;

        let mut args = Vec::<NodeExpr>::new();
        let mut first_iter = true;
        while !reader.peek_is_exact(TokenKind::Delimiter(Delimiter::ClosePar)) {
            if !first_iter {
                reader.expect_exact(TokenKind::Special(Special::Comma))?;
            }

            let arg_expr = NodeFuncCall::advance_arg(&mut reader)?;
            args.push(arg_expr);
            first_iter = false;
        }

        reader.expect_exact(TokenKind::Delimiter(Delimiter::ClosePar))?;

        let end = reader.current().end;

        Ok(NodeFuncCall {
            name,
            args,
            span: Span::new(start, end, reader.spanner()),
            namespace,
        })
    }

    fn advance_arg(reader: &mut TokenReader) -> Result<NodeExpr, ChalError> {
        let mut buffer = VecDeque::<Token>::new();
        let mut open_delims: u64 = 0;

        while !reader.is_empty() {
            let peek = reader.peek().unwrap();
            if open_delims == 0
                && (peek.kind == TokenKind::Special(Special::Comma)
                    || peek.kind == TokenKind::Delimiter(Delimiter::ClosePar))
            {
                break;
            }

            let current = reader.advance().unwrap();

            match current.kind {
                TokenKind::Delimiter(Delimiter::OpenPar)
                | TokenKind::Delimiter(Delimiter::OpenBrace)
                | TokenKind::Delimiter(Delimiter::OpenBracket) => open_delims += 1,
                TokenKind::Delimiter(Delimiter::ClosePar)
                | TokenKind::Delimiter(Delimiter::CloseBrace)
                | TokenKind::Delimiter(Delimiter::CloseBracket) => open_delims -= 1,
                _ => (),
            }
            buffer.push_back(current);
        }

        let buffer_reader = TokenReader::new(buffer, reader.current());
        NodeExpr::new(buffer_reader)
    }
}

impl TryFrom<NodeAttrRes> for NodeFuncCallStmnt {
    type Error = ChalError;

    fn try_from(node: NodeAttrRes) -> Result<Self, Self::Error> {
        let NodeAttribute::FuncCall(_) = node.last() else {
            return Err(ParserError::new(ParserErrorKind::NonFuncCallResolution, node.span).into());
        };
        Ok(NodeFuncCallStmnt(node))
    }
}
