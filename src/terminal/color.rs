use crate::parse_arg::CliArgs;
use std::fmt;
use std::sync::OnceLock;

#[cfg(windows)]
/// Set the MONOCHROME static variable, based on Windows version OR cli_args
pub fn text_color(cli_args: &CliArgs) {
    MONOCHROME
        .set(
            os_info::get()
                .edition()
                .map_or(true, |i| !i.contains("Windows 11") || cli_args.monochrome),
        )
        .ok();
}

#[cfg(not(windows))]
/// Set the MONOCHROME static variable, based on cli_args
pub fn text_color(cli_args: &CliArgs) {
    MONOCHROME.set(cli_args.monochrome).ok();
}

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
            Ok(())
        } else {
            write!(f, "\x1b[{disp}m")
        }
    }
}
