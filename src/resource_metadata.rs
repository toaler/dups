use std::{fmt};

#[derive(Clone, Debug)]
pub struct ResourceMetadata {
    path: String,
    is_dir_cache: bool,
    is_file_cache: bool,
    is_symlink_cache: bool,
    modified_cache: i64
}

impl ResourceMetadata {

    pub(crate) fn new(p: &String, is_dir: bool, is_symlink: bool, modified: i64) -> Self {
        ResourceMetadata {
            path: p.clone(),
            is_dir_cache: is_dir,
            is_file_cache: !is_dir,
            is_symlink_cache: is_symlink,
            modified_cache: modified,
        }
    }

    pub(crate) fn get_path(&self) -> &String {
        &self.path
    }

    pub(crate) fn is_dir(&self) -> bool {
        self.is_dir_cache
    }

    #[allow(warnings)]
    pub(crate) fn is_file(&self) -> bool {
        self.is_file_cache
    }

    pub(crate) fn is_symlink(&self) -> bool {
        self.is_symlink_cache
    }

    pub(crate) fn modified(&self) -> i64 {
        self.modified_cache
    }
}

impl fmt::Display for ResourceMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ResourceMetadata {{ path: {}, is_dir: {:?}, is_file: {:?}, is_symlink: {:?}, modified: {:?} }}",
            self.path,
            self.is_dir_cache,
            self.is_file_cache,
            self.is_symlink_cache,
            self.modified_cache,
        )
    }
}