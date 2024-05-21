use crate::error::{span::Span, ChalError, ParserError, ParserErrorKind};
use crate::lexer::{Keyword, Operator, Special, Token, TokenKind};
use crate::parser::{ast::NodeExpr, TokenReader};

use crate::common::Type;

/// The node representing the creation of variables or constants. The [`span`]
/// field refers to the span of the object at the left side of the expression,
/// i.e. the variable that is being created.
///
/// Syntax:
/// let \<var-name\> = \<expression\>
/// let \<var-name\>: \<type\> = \<expression\>
/// const \<var-name\> = \<expression\>
/// const \<var-name\>: \<type\> = \<expression\>
#[derive(Debug, PartialEq)]
pub struct NodeVarDef {
    pub ty: Type,
    pub name: String,
    pub value: NodeExpr,
    pub is_const: bool,
    pub span: Span,
}

/// The node representing a variable's call. Essentialy boils down to a single
/// `TokenKind::Identifier()` with the corresponding variable's name inside.
///
/// Syntax:
/// \<var_name\>
#[derive(Clone, Debug, PartialEq)]
pub struct NodeVarCall {
    pub name: String,
    pub span: Span,
}

impl NodeVarDef {
    pub fn new(mut reader: TokenReader) -> Result<NodeVarDef, ChalError> {
        let mut is_const = false;
        if reader.peek_is_exact(TokenKind::Keyword(Keyword::Const)) {
            is_const = true;
            reader.advance();
        } else {
            reader.expect_exact(TokenKind::Keyword(Keyword::Let))?;
        }

        let name = reader.expect_ident()?;
        let span = reader.current();

        let mut ty = Type::Any;
        if reader
            .expect_exact(TokenKind::Special(Special::Colon))
            .is_ok()
        {
            ty = reader.expect_type()?;
        }
        reader.expect_exact(TokenKind::Operator(Operator::Eq))?;

        let rhs = reader.advance_until(|tk| tk == &TokenKind::Newline)?;
        let rhs_reader = TokenReader::new(rhs, reader.current());
        let value = NodeExpr::new(rhs_reader)?;
        reader.expect_exact(TokenKind::Newline)?;

        Ok(NodeVarDef {
            name,
            ty,
            value,
            is_const,
            span,
        })
    }
}

impl NodeVarCall {
    pub fn new(token: Token) -> Result<Self, ChalError> {
        let kind = token.kind;
        let TokenKind::Identifier(name) = kind else {
            return Err(ParserError::new(
                ParserErrorKind::InvalidToken(TokenKind::Identifier(String::new()), kind.clone()),
                token.span,
            )
            .into());
        };
        Ok(NodeVarCall {
            name: name.clone(),
            span: token.span,
        })
    }
}
