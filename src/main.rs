pub mod errors;
pub mod lexer;
pub mod parser;
pub mod utils;

use crate::lexer::Lexer;

#[macro_use]
extern crate lazy_static;

fn main() {
    let (mut lexer, _) = Lexer::new("
    # this is a comment
    fn main(argc: i8, args: []str) -> i8 {
        let a = 5 * -3
    }
    ").ok().unwrap();

    while !lexer.is_empty() {
        let token = lexer.advance().unwrap();
        println!("{:#?}", token);
    }
}
