#[allow(dead_code)]
pub enum Colors {
    Gray,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
}

pub fn color(color: Colors, msg: &str) -> String {
    let end: &str = "\x1B[0m";

    let color_ansi = match color {
        Colors::Gray => "\x1B[90m",
        Colors::Red => "\x1B[91m",
        Colors::Green => "\x1B[92m",
        Colors::Yellow => "\x1B[93m",
        Colors::Blue => "\x1B[94m",
        Colors::Purple => "\x1B[95m",
        Colors::Cyan => "\x1B[96m",
    };
    format!("{}{}{}", color_ansi, msg, end)
}

pub fn err(msg: &str) -> String {
    format!("{}: {}", color(Colors::Red, "error"), msg)
}

#[allow(dead_code)]
pub fn warn(msg: &str) -> String {
    format!("{}: {}", color(Colors::Yellow, "warning"), msg)
}
