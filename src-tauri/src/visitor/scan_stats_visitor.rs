use std::io;
use crate::state::resource_metadata::ResourceMetadata;
use crate::state::scan_stats::ScanStats;
use crate::util::util::add_groupings_u32;
use crate::visitor::tauri_logger::Logger;
use crate::visitor::visitable::Visitable;

pub(crate) struct ScanStatsVisitor {
    stats: ScanStats,
}

impl Visitable for ScanStatsVisitor {
    fn visit(&mut self, metadata: &ResourceMetadata, _writer: &mut dyn io::Write, logger: &dyn Logger) {
        if metadata.is_dir() {
            self.stats.increment_directory();
        } else {
            self.stats.increment_file();
        }
    }

    fn recap(&mut self, w: &mut dyn io::Write, logger: &dyn Logger) {
        w.write_all(b"").expect("TODO: panic message");

        // Format the string using the write! macro and write it to the writer
        write!(
            w,
            "Scanning stats: directories={} files={}",
            add_groupings_u32(self.get_stats().get_directory_count()),
            add_groupings_u32(self.get_stats().get_file_count())
        ).expect("TODO: panic message");
    }

    fn name(&self) -> &'static str {
        "ScanStatsVisitor"
    }
}

impl ScanStatsVisitor {
    pub(crate) fn new() -> Self {
        ScanStatsVisitor {
            stats: ScanStats::new(),
        }
    }

    pub(crate) fn get_stats(&self) -> &ScanStats {
        &self.stats
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;
    use std::fs::create_dir_all;
    use std::io::{Write};
    use tempfile::TempDir;

    #[test]
    fn test_visit_files_and_directories() {
        let temp_dir = TempDir::new().unwrap();

        let f = "test_file.txt";
        let d = "test_dir";

        let file_path = temp_dir.path().join(f);
        let dir_path = temp_dir.path().join(d);

        create_dummy_file(&file_path);
        create_dir_all(&dir_path).unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        let mut writer = io::BufWriter::new(&mut buffer);

        let mut visitor = ScanStatsVisitor::new();
        let file = ResourceMetadata::new(&f.to_string(), false, false, 0, 1024, false);
        let dir = ResourceMetadata::new(&d.to_string(), true, false, 0, 1024, false);
        visitor.visit(&file, &mut writer);
        visitor.visit(&dir, &mut writer);

        assert_eq!(visitor.get_stats().get_file_count(), 1);
        assert_eq!(visitor.get_stats().get_directory_count(), 1);

        let mut output = Vec::new();
        visitor.recap(&mut output);
        assert_eq!("Scanning stats: directories=1 files=1", String::from_utf8(output).unwrap());
    }

    fn create_dummy_file(file_path: &Path) {
        let mut file = std::fs::File::create(file_path).unwrap();
        writeln!(file, "Dummy content").unwrap();
    }
}