pub enum Color {
    Green,
    Magenta,
    Red,
    Reset,
    Underline,
    Yellow,
}

/// Colorize the text using escape codes, as long as the '-m' arg is not set
impl Color {
    pub fn display(&self, monochrome: bool) -> String {
        if monochrome {
            String::new()
        } else {
            let disp = match self {
                Self::Green => "32",
                Self::Magenta => "35",
                Self::Red => "31",
                Self::Reset => "0",
                Self::Underline => "4",
                Self::Yellow => "33",
            };
            format!("\x1b[{disp}m")
        }
    }
}
