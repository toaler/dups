use std::path::Path;
use std::time::SystemTime;
use crate::metadata_state::MetadataState;
use crate::visitable::Visitable;

#[derive(Debug)]
pub struct MetadataCollectorVisitor {
    files: Vec<MetadataState>,
}

impl MetadataCollectorVisitor {
    // Function to add MetadataState to the files vector
    pub fn add_metadata_state(&mut self, metadata_state: MetadataState) {
        self.files.push(metadata_state);
    }

    // Function to retrieve a read-only reference to the files vector
    fn get_files(&self) -> &Vec<MetadataState> {
        &self.files
    }
}

// TODO write test case for visit
impl Visitable for MetadataCollectorVisitor {
    fn visit(&mut self, path: &Path) {
        let modified_time = match path.metadata() {
            Ok(metadata) => {
                if let Ok(modified_time) = metadata.modified() {
                    modified_time
                } else {
                    SystemTime::now() // Default to current time if modified time retrieval fails
                }
            }
            Err(_) => SystemTime::now(), // Handle metadata retrieval error
        };

        let metadata = MetadataState::new(path.to_string_lossy().to_string(),
                                          path.is_dir(), modified_time);

        self.add_metadata_state(metadata);
    }

    fn recap(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn test_metadata_construction() {
        let time = SystemTime::now();
        let metadata = MetadataState::new(
            String::from("/path/to/file"),
            false,
            time,
        );

        // Display trait
        assert_eq!("/path/to/file", metadata.get_path());
        assert_eq!(false, metadata.is_dir());
        assert_eq!(&time, metadata.get_modified_time());
    }

    #[test]
    fn test_add_metadata_state() {
        let mut collector = MetadataCollectorVisitor { files: Vec::new() };

        let metadata_state = MetadataState::new(
            String::from("/path/to/file"),
            false,
            SystemTime::now(),
        );
        collector.add_metadata_state(metadata_state);

        assert_eq!(collector.files.len(), 1);
    }

    #[test]
    fn test_get_files() {
        let mut collector = MetadataCollectorVisitor { files: Vec::new() };

        let time = SystemTime::now();
        let metadata_state = MetadataState::new(
            String::from("/path/to/file"),
            false,
            time,
        );
        collector.add_metadata_state(metadata_state);

        let files = collector.get_files();

        // Check if the retrieved vector contains the added MetadataState
        assert_eq!(files.len(), 1);

        // let f1 = &files[0];

        assert_eq!(files[0], MetadataState::new(
            String::from("/path/to/file"),
            false,
            time,
        ));
    }

    #[test]
    fn test_iterate_files() {
        let mut collector = MetadataCollectorVisitor { files: Vec::new() };

        let time = SystemTime::now();
        let metadata_state1 = MetadataState::new(
            String::from("/path/to/file1"),
            false,
            time,
        );
        collector.add_metadata_state(metadata_state1);

        let metadata_state2 = MetadataState::new(
            String::from("/path/to/file2"),
            true,
            time,
        );
        collector.add_metadata_state(metadata_state2);

        // Run the iterate_files function
        let files = collector.get_files();
        assert_eq!(files.len(), 2);

        assert_eq!(files[0], MetadataState::new(
            String::from("/path/to/file1"),
            false,
            time,
        ));

        assert_eq!(files[1], MetadataState::new(
            String::from("/path/to/file2"),
            true,
            time,
        ));
    }
}