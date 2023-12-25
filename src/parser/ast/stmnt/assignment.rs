use crate::error::{ChalError, ParserError, Span};
use crate::lexer::{Operator, Token, TokenKind};
use crate::parser::ast::{operators::AssignOprType, NodeExpr, NodeVarCall};
use crate::parser::TokenReader;

use std::rc::Rc;

#[derive(Debug)]
pub struct NodeAssign {
    lhs: NodeVarCall,
    opr: AssignOprType,
    rhs: NodeExpr,
}

trait IntoAssignmentOpr {
    fn try_into_assignment_opr(self, span: Rc<Span>) -> Result<AssignOprType, ChalError>;
}

impl IntoAssignmentOpr for Token {
    fn try_into_assignment_opr(self, span: Rc<Span>) -> Result<AssignOprType, ChalError> {
        match self.kind() {
            TokenKind::Operator(Operator::Eq) => Ok(AssignOprType::Eq),
            TokenKind::Operator(Operator::AddEq) => Ok(AssignOprType::AddEq),
            TokenKind::Operator(Operator::SubEq) => Ok(AssignOprType::SubEq),
            TokenKind::Operator(Operator::MulEq) => Ok(AssignOprType::MulEq),
            TokenKind::Operator(Operator::DivEq) => Ok(AssignOprType::DivEq),
            TokenKind::Operator(Operator::ModEq) => Ok(AssignOprType::ModEq),
            _ => Err(ChalError::from(ParserError::invalid_assignment_operator(
                self.start(),
                self.end(),
                span.clone(),
            ))),
        }
    }
}

impl NodeAssign {
    pub fn new(mut reader: TokenReader) -> Result<Self, ChalError> {
        let lhs_raw = reader.expect(TokenKind::Identifier(String::new()))?;
        let lhs = NodeVarCall::new(lhs_raw, reader.span())?;

        let opr = reader
            .expect(TokenKind::Operator(crate::lexer::Operator::Eq))?
            .try_into_assignment_opr(reader.span())?;

        let rhs_raw = reader.advance_until(|tk| *tk == TokenKind::Newline)?;
        let rhs = NodeExpr::new(rhs_raw, reader.span())?;

        Ok(NodeAssign { lhs, opr, rhs })
    }
}
