
pub enum Color { 
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
}

pub fn color<'a>(color: Color, msg: &str) -> String {
    let end: &str = "\x1B[0m";

    let color_ansi = match color {
        Color::Black  => "\x1B[30m",
        Color::Red    => "\x1B[31m",
        Color::Green  => "\x1B[32m",
        Color::Yellow => "\x1B[33m",
        Color::Blue   => "\x1B[34m",
        Color::Purple => "\x1B[35m",
        Color::Cyan   => "\x1B[36m",
    };
    format!("{}{}{}", color_ansi, msg, end)
}

pub fn err(msg: &str) -> String {
    format!("{}: {}", color(Color::Red, "error"), msg)
}

pub fn warn(msg: &str) -> String {
    format!("{}: {}", color(Color::Yellow, "warning"), msg)
}

pub fn internal(msg: &str) -> String {
    format!("{}: {}", color(Color::Blue, "internal"), msg)
}
