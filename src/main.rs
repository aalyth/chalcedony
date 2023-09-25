pub mod error;
pub mod lexer;
pub mod parser;
pub mod utils;

// use crate::parser::Parser; 
use crate::lexer::Lexer;

#[macro_use]
extern crate lazy_static;

fn main() {
    /*
    let (mut parser, _) = Parser::new("
    # this is a comment
    fn main(argc: i8, args: []str) -> i8 {
        let a = 5 * -3
    }
    ").ok().unwrap();
    */
    let mut lexer = Lexer::new("
# this is a comment
let a := -5.2*--3
# let b := 5 * -3
fn test(args: i8):
    let b := 3 # test123

");

    while !lexer.is_empty() {
        let current = lexer.advance_prog();
        match current {
            Ok(line) => println!("{:#?}", line),
            Err(err) => print!("{}", err),
        }
    }
}
