pub mod errors;
pub mod lexer;

use crate::lexer::Lexer;
use crate::errors::span::Span;
use crate::errors::span::pos::Position;

fn main() {
    let lexer_opt = Lexer::new("
    # this is a comment
    $ this is a multiline comment
    asdfasdasd $
    fn main(argc: i8, argv: str) -> null {
        auto a = 5 * -3; 
    }");


    let mut lexer: Lexer;
    match lexer_opt {
        None => return (),
        Some(lex) => lexer = lex,
    }

    while !lexer.is_empty() {
        println!("{:#?}\n", lexer.advance());
    }

}
