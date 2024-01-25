use crate::Visitable;
use std::time::{Instant};
use log::info;
use crate::resource_metadata::ResourceMetadata;
use crate::util::add_groupings_usize;

const RECAP_THRESHOLD: usize = 100000;

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

    // Getter methods
    pub fn total_resources(&self) -> usize {
        self.total_files_scanned + self.total_dirs_scanned
    }

    #[allow(warnings)]
    pub fn total_files_scanned(&self) -> usize {
        self.total_files_scanned
    }

    #[allow(warnings)]
    pub fn total_dirs_scanned(&self) -> usize {
        self.total_dirs_scanned
    }

    fn incremental_recap(&mut self) {
        let elapsed_time = self.recap_start_time.elapsed();

        info!(
            "resources = {} dirs = {} files = {} time = {:?}",
            add_groupings_usize(self.files_scanned_since_last_recap + self.dirs_scanned_since_last_recap),
            add_groupings_usize(self.dirs_scanned_since_last_recap),
            add_groupings_usize(self.files_scanned_since_last_recap),
            elapsed_time
        );

        // Reset counters for the next recap
        self.reset_recap_counters();
    }
}

impl Visitable for ProgressVisitor {
    fn visit(&mut self, metadata: &ResourceMetadata) {
        // Simulate file and directory scanning logic here
        // For demonstration purposes, let's just increment the counters
        if metadata.is_dir() {
            self.total_dirs_scanned += 1;
            self.dirs_scanned_since_last_recap += 1;
        } else {
            self.total_files_scanned += 1;
            self.files_scanned_since_last_recap += 1;
        }

        if (self.files_scanned_since_last_recap + self.dirs_scanned_since_last_recap) % RECAP_THRESHOLD == 0 {
            self.incremental_recap();
        }
    }


    fn recap(&mut self) {
        self.incremental_recap();

        info!(
            "Total resources={} dirs = {} files = {}",
            add_groupings_usize(self.total_resources()),
            add_groupings_usize(self.total_dirs_scanned),
            add_groupings_usize(self.total_files_scanned)

        );

        // Reset counters for the next recap
        self.reset_recap_counters();
    }

    fn name(&self) -> &'static str {
        "ProgressVisitor"
    }
}