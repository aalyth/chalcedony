
pub enum Color { 
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
}

impl Color {
    const END: &str = "\x1B[0m";

    pub fn color<'a>(color: Color, msg: &str) -> String {
        let color_ansi = match color {
            Color::Black  => "\x1B[30m",
            Color::Red    => "\x1B[31m",
            Color::Green  => "\x1B[32m",
            Color::Yellow => "\x1B[33m",
            Color::Blue   => "\x1B[34m",
            Color::Purple => "\x1B[35m",
            Color::Cyan   => "\x1B[36m",
        };
        format!("{}{}{}", color_ansi, msg, Color::END)
    }
}
