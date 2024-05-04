use crate::error::{span::Span, ChalError};
use crate::lexer::{Keyword, Operator, Special, TokenKind};
use crate::parser::{ast::NodeExpr, TokenReader};

use crate::common::Type;

/// The node representing the creation of a variable. The `span` refers to
/// the span of the object at the left side of the expression, i.e. the variable
/// that is being created.
///
/// Syntax:
/// let \<var_name\> = \<expression\>
/// let \<var_name\>: \<type\> = \<expression\>
#[derive(Debug, PartialEq)]
pub struct NodeVarDef {
    pub ty: Type,
    pub name: String,
    pub value: NodeExpr,
    pub span: Span,
}

impl NodeVarDef {
    pub fn new(mut reader: TokenReader) -> Result<NodeVarDef, ChalError> {
        reader.expect_exact(TokenKind::Keyword(Keyword::Let))?;

        let name = reader.expect_ident()?;
        let span = reader.current();

        let mut ty = Type::Any;
        if reader
            .expect_exact(TokenKind::Special(Special::Colon))
            .is_ok()
        {
            ty = reader.expect_type()?;

            reader.expect_exact(TokenKind::Operator(Operator::Eq))?;
        } else {
            reader.expect_exact(TokenKind::Operator(Operator::Eq))?;
        }

        let rhs = reader.advance_until(|tk| tk == &TokenKind::Newline)?;
        let rhs_reader = TokenReader::new(rhs, reader.current());
        let value = NodeExpr::new(rhs_reader)?;
        reader.expect_exact(TokenKind::Newline)?;

        Ok(NodeVarDef {
            name,
            ty,
            value,
            span,
        })
    }
}
