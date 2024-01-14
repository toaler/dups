use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub struct CachedMetadata {
    path: PathBuf,
    is_dir_cache: Option<bool>,
    is_file_cache: Option<bool>,
    is_symlink_cache: Option<bool>,
    modified_cache: Option<SystemTime>,
}

impl CachedMetadata {
    pub(crate) fn new(p: &Path) -> Self {
        CachedMetadata {
            path: p.to_owned(),
            is_dir_cache: None,
            is_file_cache: None,
            is_symlink_cache: None,
            modified_cache: None,
        }
    }

    pub(crate) fn is_dir(&mut self) -> bool {
        self.is_dir_cache.unwrap_or_else(|| {
            let result = self.path.is_dir();
            self.is_dir_cache = Some(result);
            result
        })
    }

    pub(crate) fn is_file(&mut self) -> bool {
        self.is_file_cache.unwrap_or_else(|| {
            let result = self.path.is_file();
            self.is_file_cache = Some(result);
            result
        })
    }

    pub(crate) fn is_symlink(&mut self) -> bool {
        self.is_symlink_cache.unwrap_or_else(|| {
            let result = self.path.is_symlink();
            self.is_symlink_cache = Some(result);
            result
        })
    }

    pub(crate) fn modified(&mut self) -> SystemTime {
        // match self.modified_cache {
        //     Some(result) => result,
        //     None => {
        //         let result = self.metadata.modified().unwrap_or_default();
        //         self.modified_cache = Some(result);
        //         result
        //     }
        // }

        SystemTime::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_dir() {
        let metadata = fs::metadata("/path/to/your/directory").expect("Failed to get metadata");
        let mut cached_metadata = CachedMetadata::new(metadata);

        assert_eq!(cached_metadata.is_dir(), true);
    }

    #[test]
    fn test_is_file() {
        let metadata = fs::metadata("/path/to/your/file.txt").expect("Failed to get metadata");
        let mut cached_metadata = CachedMetadata::new(metadata);

        assert_eq!(cached_metadata.is_file(), true);
    }

    #[test]
    fn test_is_symlink() {
        // Note: You may need to create a symbolic link for this test case
        let metadata = fs::metadata("/path/to/your/symlink").expect("Failed to get metadata");
        let mut cached_metadata = CachedMetadata::new(metadata);

        assert_eq!(cached_metadata.is_symlink(), true);
    }

    #[test]
    fn test_modified() {
        let metadata = fs::metadata("/path/to/your/file.txt").expect("Failed to get metadata");
        let mut cached_metadata = CachedMetadata::new(metadata);

        let original_modified = cached_metadata.modified();

        // Simulate a change in the file modification time
        // For actual tests, you may need to wait for a short duration between these two calls
        // to ensure a change in modification time.
        std::thread::sleep(std::time::Duration::from_secs(1));

        let new_metadata = fs::metadata("/path/to/your/file.txt").expect("Failed to get metadata");
        cached_metadata = CachedMetadata::new(new_metadata);

        let new_modified = cached_metadata.modified();

        assert_ne!(original_modified, new_modified);
    }
}
