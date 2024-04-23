use std::io;
use crate::{Visitable};
use std::time::{Instant};
use chrono::Utc;
use lazy_static::lazy_static;
use log::info;
use crate::state::resource_metadata::ResourceMetadata;
use crate::util::util::{add_groupings_u64, add_groupings_usize};
use crate::services::scanner_api::event_handler::EventHandler;

const RECAP_THRESHOLD: usize = 100000;

pub struct ProgressVisitor {
    total_files_scanned: usize,
    total_dirs_scanned: usize,
    total_size_scanned: u64,
    files_scanned_since_last_recap: usize,
    dirs_scanned_since_last_recap: usize,
    total_size_scanned_since_last_recap: u64,
    recap_start_time: Instant,
}

impl ProgressVisitor {
    pub fn new() -> Self {
        Self {
            total_files_scanned: 0,
            total_dirs_scanned: 0,
            total_size_scanned: 0,
            files_scanned_since_last_recap: 0,
            dirs_scanned_since_last_recap: 0,
            total_size_scanned_since_last_recap: 0,
            recap_start_time: Instant::now(),
        }
    }

    fn reset_recap_counters(&mut self) {
        self.files_scanned_since_last_recap = 0;
        self.dirs_scanned_since_last_recap = 0;
        self.total_size_scanned_since_last_recap = 0;
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

    #[allow(warnings)]
    pub fn total_size_scanned(&self) -> u64 { self.total_size_scanned }

    fn incremental_recap(&mut self, writer: &mut dyn io::Write, logger: &dyn EventHandler) {
        let elapsed_time = self.recap_start_time.elapsed();

        write!(
            writer,
            "resources = {} dirs = {} files = {} time = {:?}\n",
            add_groupings_usize(self.files_scanned_since_last_recap + self.dirs_scanned_since_last_recap),
            add_groupings_usize(self.dirs_scanned_since_last_recap),
            add_groupings_usize(self.files_scanned_since_last_recap),
            elapsed_time
        ).expect("TODO: panic message");

        writer.flush().expect("TODO: panic message");

        let json_payload = format!(
            r#"{{"timestamp": {:?},"resources": {}, "directories": {}, "files": {}, "size": {}, "wall_time_nanos" : "{:?}"}}"#,
            Utc::now().to_rfc3339(),
            self.files_scanned_since_last_recap + self.dirs_scanned_since_last_recap,
            self.dirs_scanned_since_last_recap,
            self.files_scanned_since_last_recap,
            self.total_size_scanned_since_last_recap,
            elapsed_time.as_nanos()
        );

        let _message = format!(
            "resources = {} dirs = {} files = {} size = {} time = {:?}\n",
            add_groupings_usize(self.files_scanned_since_last_recap + self.dirs_scanned_since_last_recap),
            add_groupings_usize(self.dirs_scanned_since_last_recap),
            add_groupings_usize(self.files_scanned_since_last_recap),
            add_groupings_u64(self.total_size_scanned_since_last_recap),
            elapsed_time.as_nanos()
        );

        // Use the logger
        logger.publish("log-event", json_payload);

        // Reset counters for the next recap
        self.reset_recap_counters();
    }
}

impl Visitable for ProgressVisitor {
    fn visit(&mut self, metadata: &ResourceMetadata, writer: &mut dyn io::Write, logger: &dyn EventHandler) {
        // Simulate file and directory scanning logic here
        // For demonstration purposes, let's just increment the counters
        if metadata.is_dir() {
            self.total_dirs_scanned += 1;
            self.dirs_scanned_since_last_recap += 1;
        } else {
            self.total_files_scanned += 1;
            self.files_scanned_since_last_recap += 1;
        }

        self.total_size_scanned += metadata.size_bytes();
        self.total_size_scanned_since_last_recap += metadata.size_bytes();

        if (self.files_scanned_since_last_recap + self.dirs_scanned_since_last_recap) % RECAP_THRESHOLD == 0 {
            self.incremental_recap(writer, logger);
        }
    }


    fn recap(&mut self, writer: &mut dyn io::Write, logger: &dyn EventHandler) {
        self.incremental_recap(writer, logger);

        write!(
            writer,
            "Total resources={} dirs = {} files = {} size = {}",
            add_groupings_usize(self.total_resources()),
            add_groupings_usize(self.total_dirs_scanned),
            add_groupings_usize(self.total_files_scanned),
            add_groupings_u64(self.total_size_scanned),
        ).expect("TODO: panic message");

        // Reset counters for the next recap
        self.reset_recap_counters();
    }

    fn name(&self) -> &'static str {
        "ProgressVisitor"
    }
}

lazy_static! {
    static ref DUMMY_METADATA: ResourceMetadata = ResourceMetadata::new(&("dummy".to_string()), false, false, 0, 0, false);
}

#[cfg(test)]
mod tests {
    use crate::services::scanner_impl::noop_event_handler::NoopEventHandler;
    // Import necessary modules for testing
    use super::*;

    #[test]
    fn test_new_progress_visitor() {
        let progress_visitor = ProgressVisitor::new();

        // Ensure initial counters are set to zero
        assert_eq!(progress_visitor.total_files_scanned, 0);
        assert_eq!(progress_visitor.total_dirs_scanned, 0);
        assert_eq!(progress_visitor.files_scanned_since_last_recap, 0);
        assert_eq!(progress_visitor.dirs_scanned_since_last_recap, 0);
    }

    #[test]
    fn test_incremental_recap() {
        let mut progress_visitor = ProgressVisitor::new();

        let mut buffer: Vec<u8> = Vec::new();
        let mut writer = io::BufWriter::new(&mut buffer);
        let logger = NoopEventHandler{};

        // Simulate scanning some files and directories
        for _ in 0..RECAP_THRESHOLD {
            progress_visitor.visit(&DUMMY_METADATA, &mut writer, &logger);
        }

        // Ensure counters are incremented and recap is triggered
        assert_eq!(progress_visitor.total_files_scanned, RECAP_THRESHOLD);
        assert_eq!(progress_visitor.total_dirs_scanned, 0);
        assert_eq!(progress_visitor.files_scanned_since_last_recap, 0);
        assert_eq!(progress_visitor.dirs_scanned_since_last_recap, 0);
    }

    #[test]
    fn test_recap() {
        let mut progress_visitor = ProgressVisitor::new();

        let mut buffer: Vec<u8> = Vec::new();
        let mut writer = io::BufWriter::new(&mut buffer);
        let logger = NoopEventHandler{};

        // Simulate scanning some files and directories
        for _ in 0..(2 * RECAP_THRESHOLD) {
            progress_visitor.visit(&DUMMY_METADATA, &mut writer, &logger);
        }

        // Ensure counters are incremented and recap is triggered
        assert_eq!(progress_visitor.total_files_scanned, 2 * RECAP_THRESHOLD);
        assert_eq!(progress_visitor.total_dirs_scanned, 0);
        assert_eq!(progress_visitor.files_scanned_since_last_recap, 0);
        assert_eq!(progress_visitor.dirs_scanned_since_last_recap, 0);

        let mut buffer: Vec<u8> = Vec::new();
        let mut writer = io::BufWriter::new(&mut buffer);
        let logger = NoopEventHandler{};

        // Trigger manual recap
        progress_visitor.recap(&mut writer, &logger);

        // Ensure counters are reset after manual recap
        assert_eq!(progress_visitor.total_files_scanned, 200000);
        assert_eq!(progress_visitor.total_dirs_scanned, 0);
        assert_eq!(progress_visitor.files_scanned_since_last_recap, 0);
        assert_eq!(progress_visitor.dirs_scanned_since_last_recap, 0);
    }
}