pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod utils;
pub mod vm;

use crate::interpreter::Chalcedony;

#[macro_use]
extern crate lazy_static;
extern crate fxhash;

fn main() {
    let mut interpreter = Chalcedony::new();
    interpreter.interpret(
        "
# let a := 1 + 2 * 3 - 4 / -5
# let c := !(5 - a)
# let b := 34.5*(23+1.5)/2

# let a = -5.2*--3
# let c := fib(min(2 + 3 * 4, 5 + 7 * 6 / 3), 2 * 3 / 2) + fib( min(5, 6) - 2 ) * 2
# let a := -(-(-5))

# let c := fib(-min(2 + 3 * 4, - 5 + 7 * 6 / 3), - 2 * 3 / 2) + fib( min(5, 6) - 2 ) * 2
# let d := 2 || 3 + !(12 / 4 * 2)

    # while i < 50:
    #     print(fib(i))
    #     i += 1

    #     if i == 42:
    #         print(\"nice\")
    #     elif i < 42:
    #         print(\"below nice\")
    #     else:
    #         print(\"more than nice\")

fn fib(n: uint) -> uint:
    let a := 1
    let b := 1
    while n > 0:
        let c := a + b
        a = b
        b = c
        n -= 1
    return b

fn fib2(n: uint) -> uint:
    if n > 2:
        return fib2(n-1) + fib2(n-2)
    return 1

fn main():
    let n := 35 
    print('Fib ' + n + ': ' + fib2(n))
",
    );
    interpreter.vm.execute(vec![200]);
    /*
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
    */

    /*
    let mut cvm = CVM::new();
    let mut code = Vec::<u8>::new();
    for i in vec![1.0, 2.0, 3.0, 2.5, -15.0, 69.0] {
        code.push(3);
        code.append(&mut Vec::from((i as f64).to_ne_bytes()));
    }
    code.push(200);
    println!("CODE: {:#?}", code);
    cvm.interpret(code);
    cvm.interpret(vec![10, 200]);
    cvm.interpret(vec![11, 200]);
    cvm.interpret(vec![12, 200]);
    cvm.interpret(vec![13, 200]);
    cvm.interpret(vec![14, 200]);
    println!("{:?}", cvm.query_type());
    */
}
