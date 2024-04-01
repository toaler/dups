use std::fs;
use log::{error, info};
use crate::services::file_api::file_management::{DeletionStatus, FileManagement};

pub struct FileManagementImpl;

impl FileManagement for FileManagementImpl {
    fn delete_file(&self, file_path: &str) -> DeletionStatus {
        match fs::remove_file(file_path) {
            Ok(_) => {
                info!("Successfully deleted file: {}", file_path);
                DeletionStatus::Success
            },
            Err(e) => {
                error!("Failed to delete file: {}. Error: {}", file_path, e);
                DeletionStatus::Failure(e.to_string())
            }
        }
    }

    fn delete_files(&self, files: &[&str]) -> Vec<DeletionStatus> {
        files.iter().map(|&file| self.delete_file(file)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_delete_file() {
        let deleter = FileManagementImpl;
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Temporary file content").unwrap();
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        match deleter.delete_file(&temp_path) {
            DeletionStatus::Success => assert!(!Path::new(&temp_path).exists()),
            DeletionStatus::Failure(msg) => panic!("Deletion failed with error: {}", msg),
        }
    }

    #[test]
    fn test_delete_files() {
        let deleter = FileManagementImpl;
        let mut temp_file1 = NamedTempFile::new().unwrap();
        let mut temp_file2 = NamedTempFile::new().unwrap();
        writeln!(temp_file1, "Temporary file 1 content").unwrap();
        writeln!(temp_file2, "Temporary file 2 content").unwrap();
        let temp_path1 = temp_file1.path().to_str().unwrap().to_string();
        let temp_path2 = temp_file2.path().to_str().unwrap().to_string();

        let statuses = deleter.delete_files(&[&temp_path1, &temp_path2]);

        for status in statuses {
            match status {
                DeletionStatus::Success => (),
                DeletionStatus::Failure(msg) => panic!("Deletion failed with error: {}", msg),
            }
        }

        assert!(!Path::new(&temp_path1).exists());
        assert!(!Path::new(&temp_path2).exists());
    }
}

