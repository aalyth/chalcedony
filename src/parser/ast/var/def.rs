use crate::error::ChalError;
use crate::lexer::{Keyword, Operator, Special, TokenKind, Type};
use crate::parser::{ast::NodeExpr, TokenReader};

#[derive(Debug)]
pub struct NodeVarDef {
    /* the variable type */
    kind: Type,
    name: String,
    /* when default type values are implemented, this could be optional, so variable declarations
     * are possible */
    value: NodeExpr,
}

impl NodeVarDef {
    pub fn new(mut reader: TokenReader) -> Result<NodeVarDef, ChalError> {
        /* let a := 5*/
        /* let b: usize = 3 */

        reader.expect_exact(TokenKind::Keyword(Keyword::Let))?;

        let name = reader.expect_ident()?;

        let mut kind = Type::Any;
        if let Ok(_) = reader.expect_exact(TokenKind::Special(Special::Colon)) {
            kind = reader.expect_type()?;

            /* NOTE: might remove later if needed to leave only function declaration */
            reader.expect_exact(TokenKind::Operator(Operator::Eq))?;
        } else {
            reader.expect_exact(TokenKind::Operator(Operator::Walrus))?;
        }

        /* TODO! parse the rhs expression */
        /* TODO! expect the end of the line
         * reader.expect_exact(TokenKind::Newline); */

        let rhs = reader.advance_until(|tk| tk == &TokenKind::Newline)?;
        let value = NodeExpr::new(rhs, reader.span())?;

        Ok(NodeVarDef { name, kind, value })
    }
}
