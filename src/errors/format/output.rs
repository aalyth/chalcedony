
use crate::errors::format::color::Color;

pub struct Output {}

impl Output {
    pub fn err(msg: &str) {
        eprint!("{}: ", Color::color(Color::Red, "Error"));
        eprintln!("{}", msg);
    }

    pub fn warn(msg: &str) {
        eprint!("{}: ", Color::color(Color::Yellow, "Warning"));
        eprintln!("{}", msg);
    }
}
