pub mod error;
pub mod lexer;
pub mod parser;

use crate::parser::Parser;

#[macro_use]
extern crate lazy_static;

fn main() {
    let mut parser = Parser::new(
        "
# let a = -5.2*--3
let c := fib(-min(2 + 3 * 4, - 5 + 7 * 6 / 3), - 2 * 3 / 2) + fib( min(5, 6) - 2 ) * 2
let c := fib(min(2 + 3 * 4, 5 + 7 * 6 / 3), 2 * 3 / 2) + fib( min(5, 6) - 2 ) * 2
# let d := 2 || 3 + !(12 / 4 * 2)

fn main(args: i8, argv: str) -> str:
    let b := ajaj 
    if -5 + 2 > 3 :
        b += 5
    elif  b == 5:
        let c := -2
        return print(\"bueno\")

    while 5 == 6:
        print(\"hello\")
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
