use crate::error::ChalError;
use crate::lexer::{Keyword, Operator, Special, TokenKind, Type};
use crate::parser::{ast::NodeExpr, TokenReader};

pub struct NodeVarDef {
    pub kind: Type,
    pub name: String,
    pub value: NodeExpr,
}

impl NodeVarDef {
    pub fn new(mut reader: TokenReader) -> Result<NodeVarDef, ChalError> {
        /* let a = 5 */
        /* let b: usize = 3 */
        reader.expect_exact(TokenKind::Keyword(Keyword::Let))?;

        let name = reader.expect_ident()?;

        let mut kind = Type::Any;
        if let Ok(_) = reader.expect_exact(TokenKind::Special(Special::Colon)) {
            kind = reader.expect_type()?;

            reader.expect_exact(TokenKind::Operator(Operator::Eq))?;
        } else {
            reader.expect_exact(TokenKind::Operator(Operator::Eq))?;
        }

        let rhs = reader.advance_until(|tk| tk == &TokenKind::Newline)?;
        let rhs_reader = TokenReader::new(rhs, reader.spanner());
        let value = NodeExpr::new(rhs_reader)?;
        reader.expect_exact(TokenKind::Newline)?;

        Ok(NodeVarDef { name, kind, value })
    }
}
