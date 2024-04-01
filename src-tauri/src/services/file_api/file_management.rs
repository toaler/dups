pub enum DeletionStatus {
    Success,
    Failure(String),
}

pub trait FileManagement {
    fn delete_file(&self, file_path: &str) -> DeletionStatus;
    fn delete_files(&self, files: &[&str]) -> Vec<DeletionStatus>;
}