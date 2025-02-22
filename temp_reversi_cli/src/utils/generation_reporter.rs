use std::sync::Mutex;

use indicatif::{ProgressBar, ProgressStyle};
use temp_reversi_ai::utils::ProgressReporter;

pub struct GenerationReporter {
    progress_bar: Mutex<Option<ProgressBar>>,
}

impl GenerationReporter {
    pub fn new() -> Self {
        Self {
            progress_bar: Mutex::new(None),
        }
    }
}

impl ProgressReporter for GenerationReporter {
    fn on_start(&self, total: usize) {
        let bar = ProgressBar::new(total as u64);
        bar.set_style(
            ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar}] {pos}/{len} games")
                .unwrap(),
        );

        match self.progress_bar.lock() {
            Ok(mut lock) => *lock = Some(bar),
            Err(e) => eprintln!("Failed to lock progress bar: {}", e),
        }
    }

    fn on_progress(&self, _current: usize, _total: usize, message: Option<&str>) {
        match self.progress_bar.lock() {
            Ok(lock) => {
                if let Some(ref bar) = *lock {
                    bar.inc(1);
                    if let Some(msg) = message {
                        bar.set_message(msg.to_string());
                    }
                }
            }
            Err(e) => eprintln!("Failed to lock progress bar: {}", e),
        }
    }

    fn on_complete(&self) {
        match self.progress_bar.lock() {
            Ok(mut lock) => {
                if let Some(ref bar) = *lock {
                    bar.finish_with_message("✅ Data generation completed.");
                }
                *lock = None;
            }
            Err(e) => eprintln!("Failed to lock progress bar: {}", e),
        }
    }
}
