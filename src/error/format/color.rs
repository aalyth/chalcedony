pub enum Colors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
}

pub fn color<'a>(color: Colors, msg: &str) -> String {
    let end: &str = "\x1B[0m";

    let color_ansi = match color {
        Colors::Black => "\x1B[30m",
        Colors::Red => "\x1B[31m",
        Colors::Green => "\x1B[32m",
        Colors::Yellow => "\x1B[33m",
        Colors::Blue => "\x1B[34m",
        Colors::Purple => "\x1B[35m",
        Colors::Cyan => "\x1B[36m",
    };
    format!("{}{}{}", color_ansi, msg, end)
}

pub fn err(msg: &str) -> String {
    format!("{}: {}", color(Colors::Red, "error"), msg)
}

pub fn warn(msg: &str) -> String {
    format!("{}: {}", color(Colors::Yellow, "warning"), msg)
}

pub fn internal(msg: &str) -> String {
    format!("{}: {}", color(Colors::Blue, "internal"), msg)
}
