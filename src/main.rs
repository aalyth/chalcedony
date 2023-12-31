pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod utils;
pub mod vm;

use crate::interpreter::Chalcedony;

#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs;

// TODO: add error for empty void function body
// TODO: add pass feature for empty void function body

// TODO: add type casts

// TODO: add short circuit logic operators

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
