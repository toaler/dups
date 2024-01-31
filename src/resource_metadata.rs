use std::{fmt};
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceMetadata {
    path: String,
    is_dir: bool,
    is_file: bool,
    is_symlink: bool,
    modified: i64,
    file_size_bytes: u64
}

impl ResourceMetadata {

    pub(crate) fn new(p: &String, is_dir: bool, is_symlink: bool, modified: i64, file_size_bytes: u64) -> Self {
        ResourceMetadata {
            path: p.clone(),
            is_dir,
            is_file: !is_dir,
            is_symlink,
            modified,
            file_size_bytes: file_size_bytes
        }
    }

    pub(crate) fn get_path(&self) -> &String {
        &self.path
    }

    pub(crate) fn is_dir(&self) -> bool {
        self.is_dir
    }

    #[allow(warnings)]
    pub(crate) fn is_file(&self) -> bool {
        self.is_file
    }

    pub(crate) fn is_symlink(&self) -> bool {
        self.is_symlink
    }

    pub(crate) fn modified(&self) -> i64 {
        self.modified
    }

    pub(crate) fn size_bytes(&self) -> u64 { self.file_size_bytes }
}

impl fmt::Display for ResourceMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ResourceMetadata {{ path: {}, is_dir: {:?}, is_file: {:?}, is_symlink: {:?}, modified: {:?} }}",
            self.path,
            self.is_dir,
            self.is_file,
            self.is_symlink,
            self.modified,
        )
    }
}

impl PartialOrd for ResourceMetadata {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.size_bytes().cmp(&other.size_bytes()))
    }
}

impl Ord for ResourceMetadata {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size_bytes().cmp(&other.size_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::visitable::Visitable;

    #[derive(Default)]
    struct VisitorMock {
        visited: HashMap<&'static str, bool>,
        recap_called: bool,
        name: &'static str,
    }

    impl VisitorMock {
        fn new(name: &'static str) -> Self {
            VisitorMock {
                visited: HashMap::new(),
                recap_called: false,
                name,
            }
        }
    }

    impl Visitable for VisitorMock {
        fn visit(&mut self, metadata: &ResourceMetadata) {
            self.visited.insert(self.name, true);
            // Add specific assertions based on your needs
            assert_eq!(metadata.is_dir(), true);
        }

        fn recap(&mut self) {
            self.recap_called = true;
        }

        fn name(&self) -> &'static str {
            self.name
        }
    }

    #[test]
    fn test_new_resource_metadata() {
        let path = String::from("/path/to/resource");
        let is_dir = true;
        let is_symlink = false;
        let modified = 123456789;

        let metadata = ResourceMetadata::new(&path, is_dir, is_symlink, modified, 0);

        let mut visitor = VisitorMock::new("TestVisitor");
        visitor.visit(&metadata);

        assert_eq!(metadata.get_path(), &path);
        assert_eq!(metadata.is_dir(), is_dir);
        assert_eq!(metadata.is_file(), !is_dir);
        assert_eq!(metadata.is_symlink(), is_symlink);
        assert_eq!(metadata.modified(), modified);
        assert_eq!(*visitor.visited.get("TestVisitor").unwrap_or(&false), true);
    }

    #[test]
    fn test_display_format() {
        let path = String::from("/path/to/resource");
        let is_dir = true;
        let is_symlink = false;
        let modified = 123456789;

        let metadata = ResourceMetadata::new(&path, is_dir, is_symlink, modified, 0);

        let mut visitor = VisitorMock::new("DisplayVisitor");
        visitor.visit(&metadata);

        let display_format = format!("{}", metadata);
        println!("{}", display_format);
        assert!(display_format.contains(&path));
        assert_eq!(display_format.contains("is_dir: true"), is_dir);
        assert_eq!(display_format.contains("is_file: false"), is_dir);
        assert_eq!(!display_format.contains("is_symlink: false"), is_symlink);
        assert!(display_format.contains(&modified.to_string()));
        assert_eq!(*visitor.visited.get("DisplayVisitor").unwrap_or(&false), true);
    }

    #[test]
    fn test_recap_called() {
        let mut visitor = VisitorMock::new("RecapVisitor");
        visitor.recap();
        assert_eq!(visitor.recap_called, true);
    }

    #[test]
    fn test_sort_by_size_bytes() {
        let metadata1 = ResourceMetadata::new(&String::from("/path1"), true, false, 123, 100);
        let metadata2 = ResourceMetadata::new(&String::from("/path2"), false, true, 456, 200);

        assert!(metadata1 < metadata2);
        assert!(metadata2 > metadata1);

        let mut vec = vec![metadata2.clone(), metadata1.clone()];
        vec.sort();

        assert_eq!(vec, vec![metadata1, metadata2]);
    }
}