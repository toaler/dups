use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::{fs, io};
use chrono::{DateTime, Utc};
use crate::services::file_api::compression_checker::CompressionChecker;
use crate::services::file_api::file_type_detector::FileTypeDetector;
use crate::services::file_impl::mime_compression_checker::MimeCompressionChecker;
use crate::services::file_impl::mime_guess_file_type_detector::MimeGuessFileTypeDetector;
use crate::state::resource_metadata::ResourceMetadata;
use crate::services::scanner_api::event_handler::EventHandler;
use crate::services::scanner_api::visitable::Visitable;

pub(crate) struct TopKResourceVisitor {
    top_resources: BinaryHeap<Reverse<ResourceMetadata>>,
}

impl Visitable for TopKResourceVisitor {
    fn visit(&mut self, metadata: &ResourceMetadata, _writer: &mut dyn io::Write, _logger: &dyn EventHandler) {
        if !metadata.is_dir() {
            if self.top_resources.len() < 50 {
                // If the heap is not full, just push the new metadata
                self.top_resources.push(Reverse(metadata.clone()));
            } else if metadata.size_bytes() > self.top_resources.peek().unwrap().0.size_bytes() {
                // If the new metadata is larger than the smallest in the heap, replace the smallest
                self.top_resources.pop();
                self.top_resources.push(Reverse(metadata.clone()));
            }
        }
    }

    fn recap(&mut self, w: &mut dyn io::Write, logger: &dyn EventHandler) {
        let reversed_sorted_resources: Vec<_> = self.top_resources.clone().into_sorted_vec().into_iter().collect();

        let mut s = String::from("[");

        let mut first = true;
        write!(w, "Top 50 Largest Resources:\n").expect("TODO: panic message");
        for (i, metadata) in reversed_sorted_resources.iter().enumerate() {
            let metadata = &metadata.0;
            let padded_ranking = format!("{:<5}", i + 1); // Padded to 5 characters for ranking
            let padded_bytes = format!("{:>16}", metadata.size_bytes()); // Padded to 50 characters for bytes
            write!(w, "Rank: {}, Bytes: {}, Path: {}\n", padded_ranking, padded_bytes, metadata.get_path()).expect("TODO: panic message");

            let m = match fs::symlink_metadata(metadata.get_path()) {
                Ok(metadata) => metadata,
                Err(e) => {
                    println!("Error accessing file ({}) metadata: {:?}", metadata.get_path(), e);
                    continue; // Skip this iteration of the loop
                }
            };

            if !first {
                s.push_str(",");
            } else {
                first = false;
            }

            let custom_format = "%y%m%d";
            let now = Utc::now();
            // Last access time
            let last_access_time = m.accessed().unwrap();
            let last_access_datetime: DateTime<Utc> = last_access_time.into();
            let last_access_iso_string = last_access_datetime.format(custom_format).to_string();
            let last_access_duration = now.signed_duration_since(last_access_datetime);
            let last_access_days = last_access_duration.num_days();

            // Modified time
            let modified_time = m.modified().unwrap();
            let modified_datetime: DateTime<Utc> = modified_time.into();
            let modified_iso_string = modified_datetime.format(custom_format).to_string();
            let modified_duration = now.signed_duration_since(modified_datetime);
            let modified_days = modified_duration.num_days();


            let detector = MimeGuessFileTypeDetector;
            let mimetype = detector.get_file_type(metadata.get_path()).unwrap();
            let compression_checker = MimeCompressionChecker;


            s.push_str(&format!("{{\"rank\": \"{}\", \"bytes\": \"{}\", \"path\": \"{}\", \"mime_type\": \"{}\", \"compressible\": \"{}\", \"modified\": {:?}, \"accessed\": {:?}, \"modified_days\": {}, \"accessed_days\": {}}}",
                                i + 1, metadata.size_bytes(), metadata.get_path(), mimetype.clone(), compression_checker.is_compressible(&mimetype), modified_iso_string, last_access_iso_string, modified_days, last_access_days));

        }

        s.push_str("]");

        logger.publish("top-k-event", s);
    }

    fn name(&self) -> &'static str {
        "Top50LargestResources"
    }
}

impl TopKResourceVisitor {
    pub(crate) fn new() -> Self {
        TopKResourceVisitor {
            top_resources: BinaryHeap::with_capacity(50),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::services::scanner_impl::noop_event_handler::NoopEventHandler;
    use super::*;

    #[test]
    fn test_recap() {
        // Prepare test data
        let mut visitor = TopKResourceVisitor::new();

        // Add resources in ascending order of size_bytes
        for size_bytes in (1..=100).step_by(2) {
            let path = format!("/path/to/resource{}", size_bytes);
            let is_dir = false;
            let is_symlink = false;
            let modified = 123456789;
            let metadata = ResourceMetadata::new(&path, is_dir, is_symlink, modified, size_bytes, false);

            let mut buffer: Vec<u8> = Vec::new();
            let mut writer = io::BufWriter::new(&mut buffer);

            let logger = NoopEventHandler{};
            visitor.visit(&metadata, &mut writer, &logger);
        }

        // Create a mock writer
        let mut mock_writer = Vec::new();

        let logger = NoopEventHandler{};
        // Call the recap method
        visitor.recap(&mut mock_writer, &logger);

        // Convert the bytes written to a string
        let recap_output = String::from_utf8(mock_writer).expect("Invalid UTF-8 sequence");

        // Split the output into lines
        let lines: Vec<&str> = recap_output.trim().split('\n').collect();

        // Check if the recap starts with the correct title
        assert!(lines[0].contains("Top 50 Largest Resources"));

        // Check if the recap contains 50 lines for the top 50 largest resources
        assert_eq!(lines.len(), 51);

        // Check if the rankings are in descending order
        for i in 1..50 {
            let current_line = lines[i];
            let next_line = lines[i + 1];
            let current_ranking = current_line.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap();
            let next_ranking = next_line.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap();
            assert!(current_ranking < next_ranking);
        }
    }
}
