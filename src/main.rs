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

/* Future improvements
   - type cast functions
   - itou() function
   - len() for strings
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
