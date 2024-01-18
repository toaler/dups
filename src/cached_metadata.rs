use std::{fmt};
use std::path::{Path};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct CachedMetadata {
    path: String,
    is_dir_cache: Option<bool>,
    is_file_cache: Option<bool>,
    is_symlink_cache: Option<bool>,
    modified_cache: Option<SystemTime>,
}

impl CachedMetadata {
    pub(crate) fn new(p: &String) -> Self {
        CachedMetadata {
            path: p.clone(),
            is_dir_cache: None,
            is_file_cache: None,
            is_symlink_cache: None,
            modified_cache: None,
        }
    }

    // TODO Clean this up
    pub(crate) fn new2(p: &String, is_dir: bool, is_symlink: bool, modified: SystemTime) -> Self {
        CachedMetadata {
            path: p.clone(),
            is_dir_cache: Some(is_dir),
            is_file_cache: None,
            is_symlink_cache: Some(is_symlink),
            modified_cache: Some(modified),
        }
    }

    pub(crate) fn get_path(&mut self) -> &String {
        &self.path
    }

    pub(crate) fn is_dir(&mut self) -> bool {
        self.is_dir_cache.unwrap_or_else(|| {
            let result = Path::new(&self.path).is_dir();
            self.is_dir_cache = Some(result);
            result
        }).clone()
    }

    #[allow(warnings)]
    pub(crate) fn is_file(&mut self) -> bool {
        self.is_file_cache.unwrap_or_else(|| {
            let result = Path::new(&self.path).is_file();
            self.is_file_cache = Some(result);
            result
        })
    }

    pub(crate) fn is_symlink(&mut self) -> bool {
        self.is_symlink_cache.unwrap_or_else(|| {
            let result = Path::new(&self.path).is_symlink();
            self.is_symlink_cache = Some(result);
            result
        })
    }

    pub(crate) fn modified(&mut self) -> SystemTime {
        self.modified_cache.unwrap_or_else(|| {
            let result = match Path::new(&self.path).metadata() {
                Ok(metadata) => metadata.modified().unwrap_or_else(|_| SystemTime::now()),
                Err(_) => UNIX_EPOCH, // Default to current time on error
            };
            self.modified_cache = Some(result);
            result
        })
    }
}

impl fmt::Display for CachedMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CachedMetadata {{ path: {}, is_dir: {:?}, is_file: {:?}, is_symlink: {:?}, modified: {:?} }}",
            self.path,
            self.is_dir_cache,
            self.is_file_cache,
            self.is_symlink_cache,
            self.modified_cache,
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_dir() {
        let metadata = fs::metadata("/path/to/your/directory").expect("Failed to get metadata");
        let mut cached_metadata = CachedMetadata::new(&metadata);

        assert_eq!(cached_metadata.is_dir(), true);
    }

    #[test]
    fn test_is_file() {
        let metadata = fs::metadata("/path/to/your/file.txt").expect("Failed to get metadata");
        let mut cached_metadata = CachedMetadata::new(&metadata);

        assert_eq!(cached_metadata.is_file(), true);
    }

    #[test]
    fn test_is_symlink() {
        // Note: You may need to create a symbolic link for this test case
        let metadata = fs::metadata("/path/to/your/symlink").expect("Failed to get metadata");
        let mut cached_metadata = CachedMetadata::new(&metadata);

        assert_eq!(cached_metadata.is_symlink(), true);
    }

    #[test]
    fn test_modified() {
        let metadata = fs::metadata("/path/to/your/file.txt").expect("Failed to get metadata");
        let mut cached_metadata = CachedMetadata::new(&metadata);

        let original_modified = cached_metadata.modified();

        // Simulate a change in the file modification time
        // For actual tests, you may need to wait for a short duration between these two calls
        // to ensure a change in modification time.
        std::thread::sleep(std::time::Duration::from_secs(1));

        let new_metadata = fs::metadata("/path/to/your/file.txt").expect("Failed to get metadata");
        cached_metadata = CachedMetadata::new(&new_metadata);

        let new_modified = cached_metadata.modified();

        assert_ne!(original_modified, new_modified);
    }
}
