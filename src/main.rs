pub mod error;
pub mod lexer;
pub mod parser;
pub mod utils;
pub mod vm;

use crate::parser::Parser;
use vm::CVM;

#[macro_use]
extern crate lazy_static;

fn main() {
    let mut parser = Parser::new(
        "
# let a := 1 + 2 * 3 - 4 / -5
# let a = -5.2*--3
# let c := fib(min(2 + 3 * 4, 5 + 7 * 6 / 3), 2 * 3 / 2) + fib( min(5, 6) - 2 ) * 2

let c := fib(-min(2 + 3 * 4, - 5 + 7 * 6 / 3), - 2 * 3 / 2) + fib( min(5, 6) - 2 ) * 2
let d := 2 || 3 + !(12 / 4 * 2)

fn fib(n: u64) -> u64:
    if n > 2:
        return fib(n-1) + fib(n-2)
    return 1

fn main(args: i8, argv: str):
    let i := 0
    while i < 50:
        print(fib(i))
        i += 1

        if i == 42:
            print(\"nice\")
        elif i < 42:
            print(\"below nice\")
        else:
            print(\"more than nice\")
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
    /*
    let mut cvm = CVM::new();
    let mut code = Vec::<u8>::new();
    for i in vec![1.0, 2.0, 3.0, 2.5, -15.0] {
        code.push(3);
        code.append(&mut Vec::from((i as f64).to_ne_bytes()));
    }
    code.push(200);
    println!("CODE: {:#?}", code);
    cvm.interpret(code);
    */
}
