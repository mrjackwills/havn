use std::fmt;
#[cfg(windows)]
use std::sync::OnceLock;

#[cfg(windows)]
/// Set the MONOCHROME static variable, based on Windows version
pub fn windows_text_color() {
    MONOCHROME
        .set(
            os_info::get()
                .edition()
                .map_or(true, |i| !i.contains("Windows 11")),
        )
        .ok();
}

#[cfg(windows)]
pub static MONOCHROME: OnceLock<bool> = OnceLock::new();

/// Colorize the text using escape codes
pub enum Color {
    Green,
    Magenta,
    Red,
    Reset,
    Underline,
    Yellow,
}

/// On Windows, check if MONOCHROME is set, and if so don't apply escape codes
#[cfg(windows)]
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

        if MONOCHROME.get().map_or(false, |i| *i) {
            write!(f, "")
        } else {
            write!(f, "\x1b[{disp}m")
        }
    }
}

#[cfg(not(windows))]
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
        write!(f, "\x1b[{disp}m")
    }
}
