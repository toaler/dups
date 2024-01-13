use std::path::Path;
use crate::Visitable;
use std::time::{Instant};

pub struct ProgressVisitor {
    total_files_scanned: usize,
    total_dirs_scanned: usize,
    files_scanned_since_last_recap: usize,
    dirs_scanned_since_last_recap: usize,
    recap_start_time: Instant,
}

impl ProgressVisitor {
    pub fn new() -> Self {
        Self {
            total_files_scanned: 0,
            total_dirs_scanned: 0,
            files_scanned_since_last_recap: 0,
            dirs_scanned_since_last_recap: 0,
            recap_start_time: Instant::now(),
        }
    }

    fn reset_recap_counters(&mut self) {
        self.files_scanned_since_last_recap = 0;
        self.dirs_scanned_since_last_recap = 0;
        self.recap_start_time = Instant::now();
    }
}

impl Visitable for ProgressVisitor {
    fn visit(&mut self, _path: &Path, is_dir: bool) {
        // Simulate file and directory scanning logic here
        // For demonstration purposes, let's just increment the counters
        if is_dir {
            self.total_dirs_scanned += 1;
            self.dirs_scanned_since_last_recap += 1;
        } else {
            self.total_files_scanned += 1;
            self.files_scanned_since_last_recap += 1;
        }

        // Check if it's time for a recap (every 100000 files)
        if (self.files_scanned_since_last_recap + self.dirs_scanned_since_last_recap) % 100000 == 0 {
            self.recap();
        }
    }

    fn recap(&mut self) {
        let elapsed_time = self.recap_start_time.elapsed();

        println!(
            "Entities = 10000, files = {}, dirs = {}, time = {:?}",
            self.files_scanned_since_last_recap,
            self.dirs_scanned_since_last_recap,
            elapsed_time
        );

        // Reset counters for the next recap
        self.reset_recap_counters();
    }
}