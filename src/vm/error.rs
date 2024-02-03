/*
use crate::error::span::{Position, Span, Spanning};
use crate::error::{ChalError, InternalError, RuntimeError};
use crate::lexer::Type;

use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Debug)]
pub enum CVMErrorKind {
    ExpectedObject,
    UnknownInstruction,
    InvalidInstruction,
    InvalidBinOperation(Type, Type), /* lhs, rhs */
    InvalidUnOperation(Type),
    UnknownVariable(String),
    UnknownFunction(String),
    TypeAssertionFail(Type, Type), /* exp, recv */
    InvalidType(Type, Type),       /* exp, recv */
}

#[derive(Debug)]
pub struct CVMError {
    start: Position,
    end: Position,
    spanner_id: u16,
    kind: CVMErrorKind,
}

impl CVMError {
    pub fn new(kind: CVMErrorKind, start: Position, end: Position, spanner_id: u16) -> Self {
        CVMError {
            kind,
            start,
            end,
            spanner_id,
        }
    }

    pub fn into(self, span_lookup: &BTreeMap<u16, Rc<dyn Spanning>>) -> ChalError {
        let Some(spanner) = span_lookup.get(&self.spanner_id) else {
            return InternalError::new("CVMError::into(): invalid span_id").into();
        };

        match self.kind {
            CVMErrorKind::InvalidInstruction => {
                InternalError::new("invalid bytecode instruction").into()
            }

            CVMErrorKind::ExpectedObject => {
                InternalError::new("invalid bytecode - expected object on the top of the stack")
                    .into()
            }

            CVMErrorKind::UnknownInstruction => InternalError::new("unknown instruction").into(),

            CVMErrorKind::UnknownVariable(var) => RuntimeError::unknown_variable(
                var,
                Span::new(self.start, self.end, spanner.clone()),
            )
            .into(),

            CVMErrorKind::UnknownFunction(func) => RuntimeError::unknown_function(
                func,
                Span::new(self.start, self.end, spanner.clone()),
            )
            .into(),

            CVMErrorKind::TypeAssertionFail(exp, recv) => RuntimeError::invalid_type(
                exp,
                recv,
                Span::new(self.start, self.end, spanner.clone()),
            )
            .into(),

            CVMErrorKind::InvalidBinOperation(lhs, rhs) => RuntimeError::invalid_operation(
                lhs,
                rhs,
                Span::new(self.start, self.end, spanner.clone()),
            )
            .into(),

            CVMErrorKind::InvalidUnOperation(ty) => RuntimeError::invalid_un_operation(
                ty,
                Span::new(self.start, self.end, spanner.clone()),
            )
            .into(),

            CVMErrorKind::InvalidType(exp, recv) => RuntimeError::invalid_type(
                exp,
                recv,
                Span::new(self.start, self.end, spanner.clone()),
            )
            .into(),
        }
    }
}
*/
