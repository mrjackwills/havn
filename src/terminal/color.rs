use crate::parse_arg::CliArgs;
use std::fmt;
use std::sync::atomic::AtomicBool;

pub static MONOCHROME: AtomicBool = AtomicBool::new(false);
pub static WIN_10: AtomicBool = AtomicBool::new(false);

#[cfg(windows)]
/// Set the MONOCHROME & WIN_10 static atomic bools, MONOCHROME based on Windows version OR cli_args
pub fn text_color(cli_args: &CliArgs) {
    let win_10 = os_info::get()
        .edition()
        .is_some_and(|i| !i.contains("Windows 11"));
    MONOCHROME.store(
        win_10 || cli_args.monochrome,
        std::sync::atomic::Ordering::SeqCst,
    );
    WIN_10.store(win_10, std::sync::atomic::Ordering::SeqCst);
}

#[cfg(not(windows))]
/// Set the MONOCHROME static variable, based on cli_args
pub fn text_color(cli_args: &CliArgs) {
    MONOCHROME.store(cli_args.monochrome, std::sync::atomic::Ordering::SeqCst);
}

/// Colorize the text using escape codes
/// Will be ignored if MONOCHROME is true
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
        if MONOCHROME.load(std::sync::atomic::Ordering::SeqCst) {
            Ok(())
        } else {
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
}
