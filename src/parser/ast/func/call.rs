use crate::error::span::{Span, Spanning};
use crate::error::ChalError;
use crate::lexer::{Delimiter, Special, Token, TokenKind};
use crate::parser::ast::NodeExpr;

use crate::parser::TokenReader;

use std::collections::VecDeque;
use std::rc::Rc;

pub struct NodeFuncCall {
    pub name: String,
    pub args: Vec<NodeExpr>,
    pub span: Span,
}

impl NodeFuncCall {
    pub fn new(tokens: VecDeque<Token>, spanner: Rc<dyn Spanning>) -> Result<Self, ChalError> {
        let mut reader = TokenReader::new(tokens, spanner.clone());

        let start = reader.current().start;

        let name = reader.expect_ident()?;
        reader.expect_exact(TokenKind::Delimiter(Delimiter::OpenPar))?;

        let mut args = Vec::<NodeExpr>::new();
        let mut first_iter = true;
        while !reader.peek_is_exact(TokenKind::Delimiter(Delimiter::ClosePar)) {
            if !first_iter {
                reader.expect_exact(TokenKind::Special(Special::Comma))?;
            }

            let arg_expr = NodeFuncCall::advance_arg(&mut reader, spanner.clone())?;
            args.push(arg_expr);
            first_iter = false;
        }

        reader.expect_exact(TokenKind::Delimiter(Delimiter::ClosePar))?;

        let end = reader.current().end;

        Ok(NodeFuncCall {
            name,
            args,
            span: Span::new(start, end, spanner.clone()),
        })
    }

    fn advance_arg(
        reader: &mut TokenReader,
        spanner: Rc<dyn Spanning>,
    ) -> Result<NodeExpr, ChalError> {
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
                TokenKind::Delimiter(Delimiter::OpenPar) => open_delims += 1,
                TokenKind::Delimiter(Delimiter::ClosePar) => open_delims -= 1,
                _ => (),
            }
            buffer.push_back(current);
        }

        NodeExpr::new(buffer, spanner)
    }
}
