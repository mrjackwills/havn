use std::fmt;

/// Colorize the text using escape codes
pub enum Color {
    Green,
    Magenta,
    Red,
    Reset,
    Underline,
    Yellow,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let disp = match self {
            Self::Green => "32",
            Self::Magenta => "35",
            Self::Red => "31",
            Self::Reset => "0",
            Self::Underline => "4",
            Self::Yellow => "33",
        };
        write!(f, "\x1b[{disp}m",)
    }
}
