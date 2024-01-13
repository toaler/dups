use std::path::Path;
use crate::scan_stats::ScanStats;
use crate::visitable::Visitable;

pub(crate) struct ScanStatsVisitor {
    stats: ScanStats,
}

impl Visitable for ScanStatsVisitor {
    fn visit(&mut self, path: &Path, is_dir: bool) {
        if is_dir {
            self.stats.increment_directory();
        }  else {
            self.stats.increment_file();
        }
    }

    fn recap(&mut self) {
        println!("");
        println!("Scanning stats:");
        println!("directories={},files={}", self.get_stats().get_directory_count(), self.get_stats().get_file_count());
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
    use super::*;
    use std::fs::create_dir_all;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_visit_files_and_directories() {
        let temp_dir = TempDir::new().unwrap();

        let file_path = temp_dir.path().join("test_file.txt");
        let dir_path = temp_dir.path().join("test_dir");

        create_dummy_file(&file_path);
        create_dir_all(&dir_path).unwrap();

        let mut visitor = ScanStatsVisitor::new();
        visitor.visit(&file_path);
        visitor.visit(&dir_path);

        assert_eq!(visitor.get_stats().get_file_count(), 1);
        assert_eq!(visitor.get_stats().get_directory_count(), 1);
    }

    fn create_dummy_file(file_path: &Path) {
        let mut file = std::fs::File::create(file_path).unwrap();
        writeln!(file, "Dummy content").unwrap();
    }
}