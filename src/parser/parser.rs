use crate::error::{ChalError, InternalError, Span};
use crate::lexer::Lexer;

use crate::parser::ast::NodeProg;

use std::rc::Rc;

pub struct Parser {
    lexer: Lexer,
    span: Rc<Span>,
}

impl Parser {
    pub fn new(code: &str) -> Parser {
        let lexer = Lexer::new(code);
        let span = Rc::clone(lexer.span());
        Parser { lexer, span }
    }

    pub fn advance(&mut self) -> Result<NodeProg, ChalError> {
        if self.lexer.is_empty() {
            return Err(InternalError::new("Parser::advance(): advancing an empty parser").into());
        }
        NodeProg::new(self.lexer.advance_prog()?, self.span.clone())
    }

    pub fn is_empty(&self) -> bool {
        self.lexer.is_empty()
    }

    pub fn span(&self) -> Rc<Span> {
        self.span.clone()
    }
}
