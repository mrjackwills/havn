use std::{
    fmt,
    io::Write,
    sync::{atomic::AtomicBool, Arc},
};

use crate::terminal::color::Color;

/// State for the loading animation
#[derive(Debug, Default, Clone, Copy)]
pub enum Animation {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

impl Animation {
    fn next(&mut self) {
        *self = match self {
            Self::One => Self::Two,
            Self::Two => Self::Three,
            Self::Three => Self::Four,
            Self::Four => Self::Five,
            Self::Five => Self::Six,
            Self::Six => Self::Seven,
            Self::Seven => Self::Eight,
            Self::Eight => Self::Nine,
            Self::Nine => Self::Ten,
            Self::Ten => Self::One,
        }
    }
}

impl fmt::Display for Animation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let disp = match self {
            Self::One => '⠋',
            Self::Two => '⠙',
            Self::Three => '⠹',
            Self::Four => '⠸',
            Self::Five => '⠼',
            Self::Six => '⠴',
            Self::Seven => '⠦',
            Self::Eight => '⠧',
            Self::Nine => '⠇',
            Self::Ten => '⠏',
        };
        write!(f, "{}{disp}{}", Color::Red, Color::Reset)
    }
}

#[derive(Debug, Default)]
pub struct Spinner(Arc<AtomicBool>);

impl Spinner {
    /// Show the cursor on the terminal again
    pub fn show_cursor() {
        let mut std = std::io::stdout().lock();
        std.write_all(b"\x1B[?25h").ok();
        std.flush().ok();
    }

    /// Hide the cursor, so spinner line looks nicer
    fn hide_cursor() {
        let mut std = std::io::stdout().lock();
        std.write_all(b"\x1B[?25l").ok();
        std.flush().ok();
    }

    /// Animate the loading icon until run is false
    async fn spin(run: Arc<AtomicBool>) {
        let mut frame = Animation::default();
        while run.load(std::sync::atomic::Ordering::SeqCst) {
            print!("{frame} scanning ");
            std::io::stdout().flush().ok();
            print!("\r");
            frame.next();
            tokio::time::sleep(std::time::Duration::from_millis(75)).await;
        }
    }

    /// Print to stdout a spinner, with the text "scanning"
    /// Spawns into own thread
    pub fn start() -> Self {
        let spinner = Self(Arc::new(AtomicBool::new(true)));
        Self::hide_cursor();
        tokio::spawn(Self::spin(Arc::clone(&spinner.0)));
        spinner
    }

    pub fn stop(&self) {
        self.0.store(false, std::sync::atomic::Ordering::SeqCst);
        Self::show_cursor();
    }
}
