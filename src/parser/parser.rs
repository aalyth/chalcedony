use crate::error::span::Spanning;
use crate::error::{ChalError, InternalError};
use crate::lexer::Lexer;

use crate::parser::ast::NodeProg;

use std::rc::Rc;

pub struct Parser {
    lexer: Lexer,
    spanner: Rc<dyn Spanning>,
}

impl Parser {
    pub fn new(code: &str) -> Parser {
        let lexer = Lexer::new(code);
        let spanner = lexer.spanner();
        Parser { lexer, spanner }
    }

    pub fn advance(&mut self) -> Result<NodeProg, ChalError> {
        if self.lexer.is_empty() {
            return Err(InternalError::new("Parser::advance(): advancing an empty parser").into());
        }
        NodeProg::new(self.lexer.advance_prog()?, self.spanner.clone())
    }

    pub fn is_empty(&self) -> bool {
        self.lexer.is_empty()
    }

    pub fn spanner(&self) -> Rc<dyn Spanning> {
        self.spanner.clone()
    }
}
