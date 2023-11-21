use crate::error::{ChalError, InternalError, Span};
use crate::lexer::{Lexer, Type};

// TODO! fix the ast/mod.rs and import only NodeProg
use crate::parser::ast::*;

use std::collections::HashMap;
use std::rc::Rc;

pub struct Parser {
    lexer: Lexer,
    span: Rc<Span>,
    /* symbol table */
    symtable: HashMap<String, Type>,
}

impl Parser {
    pub fn new(code: &str) -> Parser {
        let lexer = Lexer::new(code);
        let span = Rc::clone(lexer.span());
        Parser {
            lexer,
            span,
            symtable: HashMap::<String, Type>::new(),
        }
    }

    pub fn advance(&mut self) -> Result<NodeProg, ChalError> {
        if self.lexer.is_empty() {
            return Err(ChalError::from(InternalError::new(
                "Parser::advance(): advancing an empty parser",
            )));
        }
        NodeProg::new(self.lexer.advance_prog()?, self.span.clone())
    }

    pub fn is_empty(&self) -> bool {
        self.lexer.is_empty()
    }
}
