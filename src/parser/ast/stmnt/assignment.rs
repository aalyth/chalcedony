use crate::error::{ChalError, ParserError};
use crate::lexer::{Operator, Token, TokenKind};
use crate::parser::ast::{NodeExpr, NodeVarCall};
use crate::parser::TokenReader;

use crate::common::operators::AssignOprType;

#[derive(Debug)]
pub struct NodeAssign {
    pub lhs: NodeVarCall,
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
            _ => Err(ParserError::invalid_assignment_operator(self.span).into()),
        }
    }
}

impl NodeAssign {
    pub fn new(mut reader: TokenReader) -> Result<Self, ChalError> {
        let lhs_raw = reader.expect(TokenKind::Identifier(String::new()))?;
        let lhs = NodeVarCall::new(lhs_raw)?;

        let opr = reader
            .expect(TokenKind::Operator(Operator::Eq))?
            .try_into_assignment_opr()?;

        let rhs_raw = reader.advance_until(|tk| *tk == TokenKind::Newline)?;
        let rhs_reader = TokenReader::new(rhs_raw, reader.current());
        let rhs = NodeExpr::new(rhs_reader)?;

        Ok(NodeAssign { lhs, opr, rhs })
    }
}
