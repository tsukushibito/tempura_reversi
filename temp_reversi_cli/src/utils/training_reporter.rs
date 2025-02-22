use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Mutex;
use temp_reversi_ai::utils::ProgressReporter; // utilモジュールに定義されたトレイト

pub struct TrainingReporter {
    progress_bar: Mutex<Option<ProgressBar>>,
}

impl TrainingReporter {
    pub fn new() -> Self {
        Self {
            progress_bar: Mutex::new(None),
        }
    }
}

impl ProgressReporter for TrainingReporter {
    fn on_start(&self, total: usize) {
        let bar = ProgressBar::new(total as u64);
        bar.set_style(
            ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar}] Epoch {pos}/{len}")
                .unwrap(),
        );
        if let Ok(mut lock) = self.progress_bar.lock() {
            *lock = Some(bar);
            if let Some(ref bar) = *lock {
                bar.set_message("Starting training...".to_string());
            }
        } else {
            eprintln!("Failed to lock progress bar.");
        }
    }

    fn on_progress(&self, _current: usize, _total: usize, message: Option<&str>) {
        if let Ok(lock) = self.progress_bar.lock() {
            if let Some(ref bar) = *lock {
                bar.inc(1);
                if let Some(msg) = message {
                    bar.set_message(msg.to_string());
                }
            }
        } else {
            eprintln!("Failed to lock progress bar.");
        }
    }

    fn on_complete(&self) {
        if let Ok(mut lock) = self.progress_bar.lock() {
            if let Some(ref bar) = *lock {
                bar.finish_with_message("✅ Training completed.");
            }
            *lock = None;
        } else {
            eprintln!("Failed to lock progress bar.");
        }
    }
}
