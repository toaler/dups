use std::{fs};
use std::collections::HashMap;
use std::ops::Add;
use std::path::{Path, PathBuf};
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

        for key in keys {
            if let Ok(current) = fs::metadata(&key) {
                if let Some(mut cached) = self.registry.get_mut(&key) {
                    let file_size = current.len();
                    let modification_time = current.modified();

                    if cached.modified() != current.modified().unwrap() {

                        println!("change detected : is_dir={} {} changed new modified time {:?}", cached.is_dir(), cached.get_path(), system_time_to_string(&current.modified().unwrap()));

                        // TODO think about validating that the current and cached entities are of the same file type (file/dir).
                        // TODO for example if a dir changed to a file or a file changed to a dir.
                        if !cached.is_dir() {
                            let m = CachedMetadata::new2(&key, current.is_dir(), current.is_symlink(), current.modified().unwrap());
                            self.registry.insert(key.clone(), m);
                        } else {
                            // if metadata.is_dir() && !metadata.is_symlink() {
                                if let Ok(entries) = fs::read_dir(&key) {
                                    for entry in entries {
                                        if let Ok(e) = entry {
                                            // Using entry() to check if the key exists and insert if absent
                                            let record =  self.registry.entry(e.path().to_string_lossy().parse().unwrap());

                                            match record {
                                                std::collections::hash_map::Entry::Occupied(_) => {
                                                    // Key exists, perform additional actions if needed
                                                    // Additional code for a hit
                                                }
                                                std::collections::hash_map::Entry::Vacant(record) => {
                                                    // Key does not exist, insert value and perform additional actions
                                                    self.registry.insert(e.path().to_string_lossy().parse::<String>().unwrap().clone(),
                                                                         CachedMetadata::new2(&key, current.is_dir(), current.is_symlink(), current.modified().unwrap()));
                                                    // Additional code for a miss
                                                    println!("change detected : {:?} added", e);
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    // Todo do something different here
                                }
                            // }
                        }
                    }
                    // Additional logic for checking actual file mtime versus expected
                    // and updating metadata entry or running ReadDir, etc.
                }
            } else {
                // Handle error getting file metadata
                // TODO: file may no longer exist, remove it from the data structure
                println!("change detected : {} deleted", key);
                self.registry.remove(&key);
            }
        }
    }
    //
    // pub(crate) fn change_detection(&mut self) {
    //
    //     let keys: Vec<String> = self.registry.keys().cloned().collect();
    //
    //     for (key, mut cached) in &mut self.registry {
    //
    //         // Attempt to get metadata for the file
    //         if let Ok(current) = fs::metadata(key) {
    //             // Access various metadata properties
    //             let file_size = current.len();
    //             let modification_time = current.modified();
    //
    //            // println!("diff {} {} {:?}", key, system_time_to_string(&cached.modified()), system_time_to_string(&current.modified().unwrap()));
    //
    //             if cached.modified() != current.modified().unwrap() {
    //                 // File changed
    //                 println!("File {} changed", cached.get_path());
    //
    //                 // TODO think about validating that the current and cached entities are of the same file type (file/dir).
    //                 // TODO for example if a dir changed to a file or a file changed to a dir.
    //                 if !cached.is_dir() {
    //                     // update in-memory db
    //                     if !cached.is_dir() {
    //                         // update in-memory db
    //                         let m = CachedMetadata::new2(key, current.is_dir(), current.is_symlink(), current.modified().unwrap());
    //
    //                         // Update the entry in the registry
    //                         self.registry.insert(key.clone(), m);
    //                     }
    //                 }
    //
    //             } else {
    //                 // println!("File {} same", cached.get_path());
    //             }
    //
    //             // TODO check actual file mtime versus expected
    //             // TODO 1. update metadata entry
    //             // TODO 2. If dir mtime changed run ReadDir and add new files
    //
    //         } else {
    //             //println!("Error getting file metadata file = {}", key);
    //
    //             // TODO file may no longer exists so remove it from data structure
    //         }
    //     }
    //
    // }
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