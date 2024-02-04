mod common;
mod error;
mod interpreter;
mod lexer;
mod parser;
mod utils;
mod vm;

use crate::interpreter::Chalcedony;

extern crate ahash;

use std::env;
use std::fs;

// TODO: add compile time type assertions

// TODO: add itou function

// TODO: add 'continue' and 'break' statements
// TODO: add short circuit logic operators

// TODO: add div by zero checks

/* Future improvements
   - short cuircuit logic operators
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Error: expected only 1 argument - a file to execute");
        std::process::exit(1);
    }

    let Ok(script) = fs::read_to_string(args[1].clone()) else {
        eprintln!("Error: could not open the passed script");
        std::process::exit(1);
    };

    let mut interpreter = Chalcedony::new();
    interpreter.interpret(&script);
}
