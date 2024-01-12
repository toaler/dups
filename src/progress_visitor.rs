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
    fn visit(&mut self, path: &Path) {
        // Simulate file and directory scanning logic here
        // For demonstration purposes, let's just increment the counters
        if path.is_file() {
            self.total_files_scanned += 1;
            self.files_scanned_since_last_recap += 1;
        } else if path.is_dir() {
            self.total_dirs_scanned += 1;
            self.dirs_scanned_since_last_recap += 1;
        }

        // Check if it's time for a recap (every 100000 files)
        if (self.files_scanned_since_last_recap + self.dirs_scanned_since_last_recap) % 100000 == 0 {
            self.recap();
        }
    }

    fn recap(&mut self) {
        let elapsed_time = self.recap_start_time.elapsed();
        println!(
            "Scanned {} files and {} directories. Time taken for the last 100000 files: {:?}",
            self.files_scanned_since_last_recap,
            self.dirs_scanned_since_last_recap,
            elapsed_time
        );

        // Reset counters for the next recap
        self.reset_recap_counters();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_progress_visitor() {
    //     let mut progress_visitor = ProgressVisitor::new();
    //
    //     // Simulate scanning 3000 files and directories
    //     for i in 0..30000 {
    //         let path = if i % 2 == 0 {
    //             PathBuf::from(format!("/dummy/file{}.txt", i))
    //         } else {
    //             PathBuf::from(format!("/dummy/dir{}", i))
    //         };
    //         progress_visitor.visit(&path);
    //     }
    //
    //     // The recap should occur at 10000, 20000, and 30000 files
    //     // So, the recap method should be called three times
    //     assert_eq!(progress_visitor.total_files_scanned, 30000);
    //     assert_eq!(progress_visitor.total_dirs_scanned, 15000);
    // }

    // Add more test cases as needed
}