use std::fmt::{Display, Debug};
use std::time::SystemTime;

#[derive(Debug)]
pub struct MetadataState {
    path: String,
    is_dir: bool,
    modified_time: SystemTime,
}

impl MetadataState {
    // Constructor
    pub(crate) fn new(path: String, is_dir: bool, modified_time: SystemTime) -> Self {
        MetadataState {
            path,
            is_dir,
            modified_time,
        }
    }

    // Getter methods
    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    pub fn get_modified_time(&self) -> &SystemTime {
        &self.modified_time
    }
}

// Implement Display trait for pretty printing
impl Display for MetadataState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MetadataState {{ path: {}, is_dir: {}, modified_time: {:?} }}",
            self.get_path(),
            self.is_dir(),
            self.get_modified_time()
        )
    }
}

// Implement PartialEq trait for equality comparison
impl PartialEq for MetadataState {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
            && self.is_dir == other.is_dir
            && self.modified_time == other.modified_time
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use crate::metadata_state::MetadataState;

    #[test]
    fn test_metadata_construction() {
        let time = SystemTime::now();
        let metadata = MetadataState::new(
            String::from("/path/to/file"),
            false,
            time
        );

        // Display trait
        assert_eq!("/path/to/file", metadata.path);
        assert_eq!(false, metadata.is_dir);
        assert_eq!(time, metadata.modified_time);

        // PartialEq trait
        let metadata2 = MetadataState::new(
            String::from("/path/to/file"),
            false,
            time
        );

        assert_eq!(metadata, metadata2);
    }
}
