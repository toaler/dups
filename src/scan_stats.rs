use std::sync::{Arc, Mutex};
use std::fmt;

#[derive(Debug)]
pub struct ScanStats {
    files: Arc<Mutex<u32>>,
    directories: Arc<Mutex<u32>>,
}

impl ScanStats {
    pub(crate) fn new() -> Self {
        ScanStats {
            files: Arc::new(Mutex::new(0)),
            directories: Arc::new(Mutex::new(0)),
        }
    }

    pub(crate) fn increment_file(&self) {
        let mut files = self.files.lock().unwrap();
        *files += 1;
    }

    pub(crate) fn increment_directory(&self) {
        let mut directories = self.directories.lock().unwrap();
        *directories += 1;
    }

    pub(crate) fn get_file_count(&self) -> u32 {
        *self.files.lock().unwrap()
    }

    pub(crate) fn get_directory_count(&self) -> u32 {
        *self.directories.lock().unwrap()
    }
}

impl fmt::Display for ScanStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Files: {}, Directories: {}",
            self.get_file_count(),
            self.get_directory_count()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_file_and_directory() {
        let stats = ScanStats::new();

        stats.increment_file();
        stats.increment_file();
        stats.increment_directory();
        stats.increment_file();

        assert_eq!(stats.get_file_count(), 3);
        assert_eq!(stats.get_directory_count(), 1);
    }

    #[test]
    fn test_display_trait() {
        let stats = ScanStats::new();

        stats.increment_file();
        stats.increment_directory();

        assert_eq!(format!("{}", stats), "Files: 1, Directories: 1");
    }
}
