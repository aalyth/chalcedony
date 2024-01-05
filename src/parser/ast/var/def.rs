use crate::error::ChalError;
use crate::lexer::{Keyword, Operator, Special, TokenKind, Type};
use crate::parser::{ast::NodeExpr, TokenReader};

pub struct NodeVarDef {
    kind: Type,
    name: String,
    value: NodeExpr,
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
        let value = NodeExpr::new(rhs, reader.spanner())?;
        reader.expect_exact(TokenKind::Newline)?;

        Ok(NodeVarDef { name, kind, value })
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn disassemble(self) -> (String, Type, NodeExpr) {
        (self.name, self.kind, self.value)
    }
}
