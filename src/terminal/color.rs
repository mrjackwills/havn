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
            Self::Green => "\x1b[32m",
            Self::Magenta => "\x1b[35m",
            Self::Red => "\x1b[31m",
            Self::Reset => "\x1b[0m",
            Self::Underline => "\x1b[4m",
            Self::Yellow => "\x1b[33m",
        };
        write!(f, "{disp}",)
    }
}
