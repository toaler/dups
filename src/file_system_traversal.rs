use std::{fs};
use std::path::Path;
use crate::visitable::Visitable;

pub struct FileSystemTraversal;

impl FileSystemTraversal {
    pub(crate) fn traverse(&self, path: &Path, visitor: &dyn Visitable) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    visitor.visit(&entry.path());
                    self.traverse(&entry.path(), visitor);
                } else {
                    visitor.visit(&entry.path());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    struct MockVisitor {
        visited_paths: Vec<String>,
    }

    impl MockVisitor {
        fn new() -> Self {
            Self {
                visited_paths: Vec::new(),
            }
        }
    }

    impl Visitable for MockVisitor {
        fn visit(&self, path: &Path) {
            if let Some(path_str) = path.to_str() {
                println!("Visited: {}", path_str);
            }
        }
    }

    #[test]
    fn test_traverse_empty_directory() {
        let traversal = FileSystemTraversal {};
        let temp_dir = tempdir().expect("Unable to create temporary directory");

        let visitor = MockVisitor::new();

        traversal.traverse(temp_dir.path(), &visitor);

        assert!(visitor.visited_paths.is_empty(), "No paths should be visited in an empty directory");
    }

    #[test]
    fn test_traverse_single_file() {
        let traversal = FileSystemTraversal {};
        let temp_dir = tempdir().expect("Unable to create temporary directory");
        let temp_file_path = temp_dir.path().join("test.txt");

        File::create(&temp_file_path).expect("Unable to create temp file");

        let visitor = MockVisitor::new();

        traversal.traverse(temp_dir.path(), &visitor);

        assert_eq!(visitor.visited_paths.len(), 1, "One path should be visited in a directory with a single file");
        assert_eq!(visitor.visited_paths[0], temp_file_path.to_str().unwrap(), "The visited path should match the file created");
    }

    // Add more test cases to cover different scenarios such as nested directories, permissions, etc.
    // Ensure you cover scenarios where `fs::read_dir` might fail or return an error
}