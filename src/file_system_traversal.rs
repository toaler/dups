use std::{fs};
use std::collections::HashMap;
use std::fs::Metadata;
use crate::cached_metadata::CachedMetadata;
use crate::util::system_time_to_string;
use crate::visitable::Visitable;

pub struct FileSystemTraversal {
    registry: HashMap<String, CachedMetadata>,
    cache_accesses: usize,
    cache_misses: usize,
}

impl FileSystemTraversal {
    pub fn new_with_cache() -> FileSystemTraversal {
        FileSystemTraversal { registry: HashMap::new(), cache_accesses: 0, cache_misses: 0 }
    }

    pub fn add_metadata(&mut self, path: &String, metadata: CachedMetadata) {
        self.registry.insert(path.clone(), metadata);
    }

    pub fn get_metadata(self) -> HashMap<String, CachedMetadata> {
        self.registry
    }

    pub fn metadata_size(&self) -> usize {
        self.registry.len()
    }
    pub fn get_cache_stats(&self) -> (usize, usize) {
        (self.cache_accesses, self.cache_misses)
    }

    pub(crate) fn traverse(&mut self, path: &String, visitors: &mut [&mut dyn Visitable]) {
        self.cache_accesses += 1;
        let mut metadata = self.registry.entry(path.clone()).or_insert_with(|| {
            self.cache_misses += 1;
            CachedMetadata::new(&path)
        });

        for visitor in &mut *visitors {
            visitor.visit(metadata);
        }

        if metadata.is_dir() && !metadata.is_symlink() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries {
                    if let Ok(e) = entry {
                        self.traverse(&e.path().to_string_lossy().to_string(), visitors);
                    }
                }
            } else {
                // Todo do something different here
            }
        }
    }

    pub(crate) fn change_detection(&mut self) {
        let keys: Vec<String> = self.registry.keys().cloned().collect();

        self.scan_resources_for_change(keys);
    }

    fn scan_resources_for_change(&mut self, keys: Vec<String>) {
        for key in keys {
            self.scan_resource_for_change(&key);
        }
    }

    pub(crate) fn scan_resource_for_change(&mut self, key: &String) {
        if let Ok(current) = fs::metadata(&key) {
            if let Some(mut cached) = self.registry.get_mut(key) {
                if cached.modified() != current.modified().unwrap() {
                    println!("change detected : is_dir={} {} changed new modified time {:?}", cached.is_dir(), cached.get_path(), system_time_to_string(&current.modified().unwrap()));

                    if !cached.is_dir() {
                        self.sync_file(key, &current);
                    } else {
                        self.sync_dir(key, &current);
                    }
                }
            }
        } else {
            // Handle error getting file metadata
            // TODO: file may no longer exist, remove it from the data structure
            println!("change detected : {} deleted", key);
            self.registry.remove(key);
        }
    }

    fn sync_file(&mut self, key: &String, current: &Metadata) {
        self.put_metadata(key, &current);
    }

    fn sync_dir(&mut self, key: &String, current: &Metadata) {
        if let Ok(children) = fs::read_dir(key) {
            for child in children {
                if let Ok(e) = child {
                    // Look only for new files/dir's added. Deleted files/dir's in a dir will get pruned on initial scan

                    let resource = &e.path().to_string_lossy().into_owned();
                    let value = self.registry.get(resource);

                    match value {
                        Some(value) => {
                            // Resource is known so ignore. If it changed it was picked up in initial files
                        }
                        None => {
                            // Resource does not exist, insert value and perform additional actions
                            // Need to acquire metadata for file

                            if let Ok(c) = fs::metadata(resource) {
                                println!("change detected : {:?} added", resource);
                                self.put_metadata(&resource.to_string(), &c);

                                if (c.is_dir()) {
                                    self.sync_dir(&resource.to_string(), &c);
                                }
                            }
                        }
                    }
                }
            }
        } else {
            // Todo do something different here
        }

        // update the changed dir metadata as well
        self.put_metadata(key, &current);
    }

    fn put_metadata(&mut self, key: &String, current: &Metadata) {
        let m = CachedMetadata::new2(&key, current.is_dir(), current.is_symlink(), current.modified().unwrap());
        self.registry.insert(key.clone(), m);
    }
}

// #[cfg(test)]
// mod tests {
//     use std::fs;
//     use std::path::Path;
//     use crate::visitable::Visitable;
//     use super::*;
//
//     // Mock Visitable implementation for testing
//     struct MockVisitor {
//         visited_paths: Vec<String>,
//     }
//
//     impl Visitable for MockVisitor {
//         fn visit(&mut self, path: &Path, _is_dir: bool) {
//             self.visited_paths.push(path.to_str().unwrap().to_string());
//         }
//
//         fn recap(&mut self) {
//         }
//     }
//
//     #[test]
//     fn test_traverse_single_file() {
//         let mut mock_visitor = MockVisitor { visited_paths: Vec::new() };
//
//         // Create a Vec<&mut dyn Foo> and add mutable references to the implementations
//         let mut visitors: Vec<&mut dyn Visitable> = Vec::new();
//         visitors.push(&mut mock_visitor);
//
//         let traversal = FileSystemTraversal {};
//         let test_file = Path::new("path_to_single_file");
//
//         // Create a single file for testing
//         fs::write(&test_file, "Test content").expect("Unable to create test file");
//         traversal.traverse(&test_file, false, &mut visitors);
//
//         assert_eq!(mock_visitor.visited_paths, vec![test_file.to_str().unwrap().to_string()]);
//
//         // Clean up: Delete the created file
//         fs::remove_file(&test_file).expect("Unable to delete test file");
//     }
//
//     #[test]
//     fn test_traverse_directory_structure() {
//
//         let mut mock_visitor = MockVisitor { visited_paths: Vec::new() };
//
//         // Create a Vec<&mut dyn Foo> and add mutable references to the implementations
//         let mut visitors: Vec<&mut dyn Visitable> = Vec::new();
//         visitors.push(&mut mock_visitor);
//
//         let traversal = FileSystemTraversal {};
//         let test_directory = Path::new("test_directory");
//
//         // Create a directory structure for testing
//         fs::create_dir_all(test_directory.join("subdir1/subdir2")).expect("Unable to create test directory structure");
//         fs::write(test_directory.join("file1.txt"), "Content").expect("Unable to create test file");
//         fs::write(test_directory.join("subdir1/file2.txt"), "Content").expect("Unable to create test file");
//         fs::write(test_directory.join("subdir1/subdir2/file3.txt"), "Content").expect("Unable to create test file");
//
//         traversal.traverse(&test_directory, true, &mut visitors);
//
//         let expected_paths = vec![
//             test_directory.to_str().unwrap().to_string(),
//             test_directory.join("file1.txt").to_str().unwrap().to_string(),
//             test_directory.join("subdir1").to_str().unwrap().to_string(),
//             test_directory.join("subdir1/file2.txt").to_str().unwrap().to_string(),
//             test_directory.join("subdir1/subdir2").to_str().unwrap().to_string(),
//             test_directory.join("subdir1/subdir2/file3.txt").to_str().unwrap().to_string(),
//         ];
//
//         assert_eq!(mock_visitor.visited_paths, expected_paths);
//
//         // Clean up: Delete the created directory structure
//         fs::remove_dir_all(&test_directory).expect("Unable to delete test directory structure");
//     }
// }