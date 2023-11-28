pub mod error;
pub mod lexer;
pub mod parser;
pub mod utils;

// use crate::lexer::Lexer;
use crate::parser::Parser;

#[macro_use]
extern crate lazy_static;

fn main() {
    /*
        let mut lexer = Lexer::new("
    # this is a comment
    let a := -5.2*--3
    # let b := 5 * -3
    fn main(args: i8):
        let b := 3 # test123

    ");

        while !lexer.is_empty() {
            let current = lexer.advance_prog();
            match current {
                Ok(line) => println!("{:#?}", line),
                Err(err) => print!("{}", err),
            }
        }
    */
    /* TODO proper unary negation (-) handling inside the lexer */
    let mut parser = Parser::new(
        "
# let a = -5.2*--3
let b: i8 = 15
let c := fib(-min(2 + 3 * 4, - 5 + 7 * 6 / 3), - 2 * 3 / 2) + fib( min(5, 6) - 2 ) * 2
# let c := fib(min(2 + 3 * 4, 5 + 7 * 6 / 3), 2 * 3 / 2) + fib( min(5, 6) - 2 ) * 2
let d := 2 || 3 + !(12 / 4 * 2)
fn main(args: i8, argv: str):
    let b := 3
    let c := fib(-min(2 + 3 ** 4, - 5 + 7 * 6 / 3), - 2 * 3 / 2) + fib( min(5, 6) - 2 ) * 2

    if a < 16:
        b += 12
        while c == 42:
            d -= 15
        e *= 10
    elif a == 5:
        f *= 12
    else:
        g = g + 5
        

",
    );

    while !parser.is_empty() {
        let current = parser.advance();
        match current {
            Ok(node) => println!("{:#?}\n", node),
            Err(err) => {
                print!("{}\n", err);
                continue;
            }
        }
    }
    println!("bueno");
}
