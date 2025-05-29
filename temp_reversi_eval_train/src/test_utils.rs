use std::{fs, path::PathBuf};

/// Test cleanup helper using RAII pattern
pub struct TestCleanup {
    files: Vec<PathBuf>,
    dirs: Vec<PathBuf>,
}

impl TestCleanup {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }

    pub fn add_file<P: Into<PathBuf>>(&mut self, path: P) {
        self.files.push(path.into());
    }

    pub fn add_dir<P: Into<PathBuf>>(&mut self, path: P) {
        self.dirs.push(path.into());
    }
}

impl Drop for TestCleanup {
    fn drop(&mut self) {
        // Clean up files first
        for file_path in &self.files {
            if file_path.exists() {
                let _ = fs::remove_file(file_path);
            }
        }

        // Then clean up directories
        for dir_path in &self.dirs {
            if dir_path.exists() {
                let _ = fs::remove_dir_all(dir_path);
            }
        }
    }
}

/// Mock progress reporter for testing
#[derive(Clone)]
pub struct MockProgressReporter;

impl crate::dataset_generator::ProgressReporter for MockProgressReporter {
    fn increment(&self, _delta: u64) {}
    fn finish(&self) {}
    fn set_message(&self, _message: &str) {}
}
