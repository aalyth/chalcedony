pub mod errors;
pub mod lexer;
// pub mod parser;

use crate::lexer::Lexer;
use crate::errors::span::Span;

#[macro_use]
extern crate lazy_static;

fn main() {
    let mut lexer = Lexer::new("
    # this is a comment
    fn main()->i8:
        let a = 5 * -3
    nf").ok().unwrap();

    while !lexer.is_empty() {
        let token = lexer.advance().unwrap();
        println!("{:#?}", token);
    }
}
