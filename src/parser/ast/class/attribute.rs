use crate::error::{span::Span, ChalError, ParserError, ParserErrorKind};
use crate::lexer::{Delimiter, Special, TokenKind};
use crate::parser::ast::{NodeFuncCall, NodeVarCall};
use crate::parser::TokenReader;

/// The structure representing a single unit inside an attribute resolution. An
/// attribute could be a member access (=VarCall), or a method call (=FuncCall).
#[derive(Clone, Debug, PartialEq)]
pub enum NodeAttribute {
    VarCall(NodeVarCall),
    FuncCall(NodeFuncCall),
}

// The structure denoting the series of attribute accesses. An atribute is
// considered as member access (=VarCall), or method calling (=FuncCall). The
// first element in the resolution series refers to an "outer" scope (a var call
// is a global/local/arg, a func call is calling an actual function, not a
// method) and the following attributes refer to a proper member access or
// method call.
//
// Syntax:
// \<attribute\>.\<attribute\>.\<attribute\>(...)
// \<namespace\>::\<func-call-attribute\>.\<attribute\>.\<attribute\>(...)
//
// where an `<attribute>` is either a function call (a method) or a variable
// call (member call)
#[derive(Clone, Debug, PartialEq)]
pub struct NodeAttrRes {
    pub resolution: Vec<NodeAttribute>,
    pub span: Span,
}

impl NodeAttribute {
    fn new(reader: &mut TokenReader, first_iter: bool) -> Result<Self, ChalError> {
        match &reader.peek().unwrap().kind {
            TokenKind::Identifier(_) => {}
            kind => {
                return Err(ParserError::new(
                    ParserErrorKind::InvalidToken(
                        TokenKind::Identifier(String::new()),
                        kind.clone(),
                    ),
                    reader.current(),
                )
                .into())
            }
        }

        // there are no more tokens left in the reader, so it's a var attribute
        let Some(peek) = reader.peek_nth(1) else {
            let token = reader.advance().unwrap();
            return Ok(NodeAttribute::VarCall(NodeVarCall::new(token)?));
        };

        match peek.kind {
            TokenKind::Delimiter(Delimiter::OpenPar) => {
                let buffer = reader.advance_scope(
                    TokenKind::Delimiter(Delimiter::OpenPar),
                    TokenKind::Delimiter(Delimiter::ClosePar),
                );
                Ok(NodeAttribute::FuncCall(NodeFuncCall::new(buffer)?))
            }

            TokenKind::Special(Special::Resolution) if first_iter => {
                let buffer = reader.advance_scope(
                    TokenKind::Delimiter(Delimiter::OpenPar),
                    TokenKind::Delimiter(Delimiter::ClosePar),
                );
                Ok(NodeAttribute::FuncCall(NodeFuncCall::new(buffer)?))
            }

            _ => {
                let token = reader.advance().unwrap();
                Ok(NodeAttribute::VarCall(NodeVarCall::new(token)?))
            }
        }
    }

    pub fn as_var_call(&self) -> Option<&NodeVarCall> {
        match self {
            NodeAttribute::VarCall(node) => Some(node),
            _ => None,
        }
    }
}

impl NodeAttrRes {
    // Parses the attribute resolution in a non-greedy manner - the attributes
    // are parsed until a non-attribute token is found.
    pub fn new(reader: &mut TokenReader) -> Result<Self, ChalError> {
        let mut resolution = Vec::<NodeAttribute>::new();
        let start = reader.current().start;
        resolution.push(NodeAttribute::new(reader, true)?);

        while !reader.is_empty() && reader.peek_is_exact(TokenKind::Special(Special::Dot)) {
            /* remove the dot */
            reader.advance();
            resolution.push(NodeAttribute::new(reader, false)?);
        }
        let span = Span::new(start, reader.current().end, reader.spanner());

        Ok(NodeAttrRes { resolution, span })
    }

    pub fn first(&self) -> &NodeAttribute {
        self.resolution
            .first()
            .expect("resolutions should never be empty")
    }

    pub fn last(&self) -> &NodeAttribute {
        self.resolution
            .last()
            .expect("resolutions should never be empty")
    }
}
