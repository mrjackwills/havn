use std::{
    fmt,
    io::Write,
    sync::{atomic::AtomicBool, Arc},
};

use crate::terminal::color::Color;

use super::color::WIN_10;

const FRAMES: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

const WIN_10_FRAMES: [char; 10] = ['|', '/', '-', '\\', '|', '/', '-', '\\', '|', '|'];

#[derive(Debug, Default)]
pub struct Spinner(Arc<AtomicBool>);

enum Cursor {
    Show,
    Hide,
}

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if WIN_10.load(std::sync::atomic::Ordering::SeqCst) {
            Ok(())
        } else {
            let disp = match self {
                Self::Show => "h",
                Self::Hide => "l",
            };
            write!(f, "\x1b[?25{disp}")
        }
    }
}

impl Spinner {
    /// Show the cursor on the terminal again
    pub fn show_cursor() {
        print!("{}", Cursor::Show);
    }

    /// Hide the cursor, so spinner line looks nicer
    fn hide_cursor() {
        print!("{}", Cursor::Hide);
    }

    /// Animate the loading icon until `run` is false
    /// Should this be converted to non-async, and spawned using std::thread?
    async fn spin(run: Arc<AtomicBool>) {
        while run.load(std::sync::atomic::Ordering::SeqCst) {
            let frames = if WIN_10.load(std::sync::atomic::Ordering::SeqCst) {
                WIN_10_FRAMES
            } else {
                FRAMES
            };
            for i in frames {
                print!("{c}{i}{r} scanning ", c = Color::Red, r = Color::Reset);
                std::io::stdout().flush().ok();
                print!("\r");
                tokio::time::sleep(std::time::Duration::from_millis(75)).await;
            }
        }
    }

    pub fn new() -> Self {
        Self(Arc::new(AtomicBool::new(false)))
    }

    /// Print to stdout a spinner, with the text "scanning"
    /// Spawns into own thread
    pub fn start(&self) {
        if !self.0.swap(true, std::sync::atomic::Ordering::SeqCst) {
            Self::hide_cursor();
            tokio::spawn(Self::spin(Arc::clone(&self.0)));
        }
    }

    /// Stop the spinner, and re-show the cursor
    pub fn stop(&self) {
        self.0.store(false, std::sync::atomic::Ordering::SeqCst);
        Self::show_cursor();
    }
}

// todo test spinners somehow
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::Cursor;
    use crate::terminal::color::WIN_10;

    #[test]
    /// Cursor shown, hidden, but ignored when WIN_10 is true
    fn test_spinner_cursor() {
        assert_eq!(
            [27, 91, 63, 50, 53, 104],
            Cursor::Show.to_string().as_bytes()
        );
        assert_eq!(
            [27, 91, 63, 50, 53, 108],
            Cursor::Hide.to_string().as_bytes()
        );

        WIN_10.store(true, std::sync::atomic::Ordering::SeqCst);

        assert!(Cursor::Show.to_string().as_bytes().is_empty());
        assert!(Cursor::Hide.to_string().as_bytes().is_empty());
    }
}
