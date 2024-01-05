use crate::error::{ChalError, ParserError};
use crate::lexer::{Operator, Token, TokenKind};
use crate::parser::ast::{operators::AssignOprType, NodeExpr, NodeVarCall};
use crate::parser::TokenReader;

pub struct NodeAssign {
    lhs: NodeVarCall,
    opr: AssignOprType,
    rhs: NodeExpr,
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
        let lhs = NodeVarCall::new(lhs_raw, reader.spanner())?;

        let opr = reader
            .expect(TokenKind::Operator(Operator::Eq))?
            .try_into_assignment_opr()?;

        let rhs_raw = reader.advance_until(|tk| *tk == TokenKind::Newline)?;
        let rhs = NodeExpr::new(rhs_raw, reader.spanner())?;

        Ok(NodeAssign { lhs, opr, rhs })
    }

    pub fn disassemble(self) -> (NodeVarCall, AssignOprType, NodeExpr) {
        (self.lhs, self.opr, self.rhs)
    }
}
