use crate::error::{ChalError, ParserError, ParserErrorKind};
use crate::lexer::{Operator, Token, TokenKind};
use crate::parser::ast::{NodeAttrRes, NodeAttribute, NodeExpr};
use crate::parser::TokenReader;

use crate::common::operators::AssignOprType;

/// The node representing changes to variable/members in the source code.
///
/// Syntax:
/// \<attribute-resolution\> \<opr\> \<expression\>
///
/// for reference to `<attribute-resolution>` see `NodeAttrRes`
#[derive(Debug, PartialEq)]
pub struct NodeAssign {
    pub lhs: NodeAttrRes,
    pub opr: AssignOprType,
    pub rhs: NodeExpr,
}

trait IntoAssignmentOpr {
    fn try_into_assignment_opr(self) -> Result<AssignOprType, ChalError>;
}

impl IntoAssignmentOpr for Token {
    fn try_into_assignment_opr(self) -> Result<AssignOprType, ChalError> {
        match self.kind {
            TokenKind::Operator(Operator::Eq) => Ok(AssignOprType::Eq),
            TokenKind::Operator(Operator::AddEq) => Ok(AssignOprType::AddEq),
            TokenKind::Operator(Operator::SubEq) => Ok(AssignOprType::SubEq),
            TokenKind::Operator(Operator::MulEq) => Ok(AssignOprType::MulEq),
            TokenKind::Operator(Operator::DivEq) => Ok(AssignOprType::DivEq),
            TokenKind::Operator(Operator::ModEq) => Ok(AssignOprType::ModEq),
            _ => {
                Err(ParserError::new(ParserErrorKind::InvalidAssignmentOperator, self.span).into())
            }
        }
    }
}

impl NodeAssign {
    pub fn new(lhs: NodeAttrRes, mut reader: TokenReader) -> Result<Self, ChalError> {
        let opr = reader
            .expect(TokenKind::Operator(Operator::Eq))?
            .try_into_assignment_opr()?;

        let rhs_raw = reader.advance_until(|tk| *tk == TokenKind::Newline)?;
        let rhs_reader = TokenReader::new(rhs_raw, reader.current());
        let rhs = NodeExpr::new(rhs_reader)?;

        for attribute in &lhs.resolution {
            if let NodeAttribute::FuncCall(node) = attribute {
                return Err(ParserError::new(
                    ParserErrorKind::FuncCallAssignment,
                    node.span.clone(),
                )
                .into());
            }
        }

        reader.expect_exact(TokenKind::Newline)?;

        Ok(NodeAssign { lhs, opr, rhs })
    }
}
