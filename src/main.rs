pub mod errors;
pub mod lexer;
pub mod parser;

use crate::parser::Parser; 

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
    let (mut parser, _) = Parser::new("
    # this is a comment
    let a := -5.2*--3
    # let b := 5 * -3
    fn test(test_arg: i8,) {}
    ").ok().unwrap();

    while let Some(node) = parser.advance() {
        // println!("{:#?}", node);
    }

}
