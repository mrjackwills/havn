use std::{
    io::Write,
    sync::{atomic::AtomicBool, Arc},
};

use crate::terminal::color::Color;

const FRAMES: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

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

    /// Animate the loading icon until `run` is false
    async fn spin(run: Arc<AtomicBool>) {
        while run.load(std::sync::atomic::Ordering::SeqCst) {
            for i in FRAMES {
                print!("{c}{i}{r} scanning ", c = Color::Red, r = Color::Reset);
                std::io::stdout().flush().ok();
                print!("\r");
                tokio::time::sleep(std::time::Duration::from_millis(75)).await;
            }
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

    /// Stop the spinner, and re-show the cursor
    pub fn stop(&self) {
        self.0.store(false, std::sync::atomic::Ordering::SeqCst);
        Self::show_cursor();
    }
}
