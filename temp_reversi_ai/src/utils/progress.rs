/// Trait for reporting progress during data generation and training.
pub trait ProgressReporter {
    /// Called when the progress starts.
    fn on_start(&self, total: usize);

    /// Called to report the current progress with an optional message.
    fn on_progress(&self, current: usize, total: usize, message: Option<&str>);

    /// Called when the progress is complete.
    fn on_complete(&self);
}
