use crate::errors::span::Span;
use crate::lexer::Lexer;
use crate::parser::ast::*;
// use crate::parser::ast::func::NodeFuncDef;

use std::collections::VecDeque;


pub struct Parser {
    // fields to contain variable and function definitions
    nodes: VecDeque<NodeProg> 
}

impl Parser {
    pub fn new(code: &str) -> Result<(Parser, Span), ()>{
        let mut result = Parser {
            nodes: VecDeque::<NodeProg>::new(),
        };

        let span = result.generate(code)?;
        
        Ok( (result, span) )
    }

    pub fn advance(&mut self) -> Option<NodeProg> {
        self.nodes.pop_front()
    }

    fn generate(&mut self, code: &str) -> Result<Span, ()> {
        let (mut lexer, span) = Lexer::new(code)?;
        let mut failed = false;

        while !lexer.is_empty() {
            if let Some(tokens) = lexer.advance_program(&span) {
                println!("tokens = {:#?}\n", tokens);
                let next_node = NodeProg::new(tokens, &span);
                match next_node {
                    Ok(node) => self.nodes.push_back(node),
                    Err(_)   => failed = true,
                }
            } 
        }

        if failed { return Err(()); }
        Ok(span)
    }
}
