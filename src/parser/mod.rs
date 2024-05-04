//! The part of the `Chalcedony` interpreter, responsible for converting the
//! received stream of tokens into the `Abstract Syntax Tree (AST)`. For details
//! about the `AST` refer to the [`ast`] module.
//!
//! It is important to note that `Chalcedony` does not use any sort of parser
//! generators or any other form of grammar blueprinting - the whole `parser`
//! implementation is a custom handwritten parser with a lookup of 2, i.e. any
//! node inside the `AST` can be computed with at most 2 lookaheads.

pub mod ast;
mod line_reader;
mod token_reader;

pub use line_reader::LineReader;
pub use token_reader::TokenReader;

use crate::error::span::Spanning;
use crate::error::ChalError;
use crate::lexer::Lexer;

use crate::parser::ast::NodeProg;

use std::rc::Rc;

/// The structure used to go over the lexed stream of tokens and transform them
/// into the Abstract Syntax Tree. For each possible node refer to `NodeProg`
/// and each individual node variant inside it.
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
            panic!("Parser::advance(): advancing an empty parser");
        }
        NodeProg::new(self.lexer.advance_prog()?, self.spanner.clone())
    }

    pub fn is_empty(&self) -> bool {
        self.lexer.is_empty()
    }
}
