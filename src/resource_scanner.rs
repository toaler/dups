use std::collections::HashMap;

use std::{fs};
use std::os::unix::fs::MetadataExt;
use log::{debug, info};

use crate::resource_metadata::ResourceMetadata;
use crate::visitable::Visitable;


pub struct ResourceScanner {
}

impl ResourceScanner {
    pub fn new() -> ResourceScanner {
        ResourceScanner {  }
    }

    pub(crate) fn full_scan(&mut self, registry: &mut HashMap<String, ResourceMetadata>, path: &String, visitors: &mut [&mut dyn Visitable]) {
        let metadata = registry.entry(path.clone()).or_insert_with(|| {
            let m = fs::symlink_metadata(path).unwrap();
            ResourceMetadata::new(&path, m.is_dir(), m.is_symlink(), m.mtime(), m.len())
        });

        for visitor in &mut *visitors {
            visitor.visit(metadata);
        }

        if metadata.is_dir() && !metadata.is_symlink() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries {
                    if let Ok(e) = entry {
                        self.full_scan(registry, &e.path().to_string_lossy().to_string(), visitors);
                    }
                }
            }
        }
    }

    pub(crate) fn incremental_scan(&mut self, root: &String, registry: &mut HashMap<String, ResourceMetadata>, visitors: &mut [&mut dyn Visitable]) {
        let mut keys: Vec<String> = registry
            .keys()
            .cloned()
            .filter(|key| key.starts_with(root))
            .collect();

        // Sort the keys so lstat lookups have locality
        keys.sort();
        info!("Scanning resources={}", keys.len());

        self.inspect_resources_for_change(registry, keys, visitors);
    }

    fn inspect_resources_for_change(&mut self, registry: &mut HashMap<String, ResourceMetadata>, keys: Vec<String>, visitors: &mut [&mut dyn Visitable]) {
        for key in keys {
            self.inspect_resource_for_change(registry, &key, visitors);
        }
    }

    pub(crate) fn inspect_resource_for_change(&mut self, registry: &mut HashMap<String, ResourceMetadata>, key: &String, visitors: &mut [&mut dyn Visitable]) {
        match registry.get_mut(key) {
            Some(cached_metadata) => {
                match fs::symlink_metadata(key) {
                    Ok(value) => {
                        let mtime = value.mtime();

                        if cached_metadata.modified() != mtime {
                            // Cached resource is invalid
                            debug!("Resource changed : is_dir={} {} new modified time {:?}", value.is_dir(), key, mtime);

                            let current = ResourceMetadata::new(&key, value.is_dir(), value.is_symlink(), mtime, value.len());
                            if !cached_metadata.is_dir() {
                                self.sync_file(registry, &current, visitors);
                            } else {
                                self.sync_dir(registry, &current, visitors);
                            }
                        } else {
                            // Cached resource is fresh
                            Self::visit(cached_metadata, visitors);
                        }
                    }
                    Err(_value) => {
                        debug!("change detected : {} deleted", key);
                        registry.remove(key);
                    }
                }
            }
            _ => {
                // Shouldn't get here, all lookups are from keys that exist in cache
            }
        }
    }

    fn sync_file(&mut self, registry: &mut HashMap<String, ResourceMetadata>, current: &ResourceMetadata, visitors: &mut [&mut dyn Visitable]) {
        Self::update(registry, &current.get_path(), &current);
        Self::visit(&current, visitors);
    }

    fn sync_dir(&mut self, registry: &mut HashMap<String, ResourceMetadata>, current: &ResourceMetadata, visitors: &mut [&mut dyn Visitable]) {
        debug!("Resource changed : {}", current.get_path());

        Self::update(registry, &current.get_path(), &current);
        Self::visit(&current, visitors);

        match fs::read_dir(current.get_path()) {
            Ok(children) => {
                for child in children {
                    match child {
                        Ok(e) => {
                            let resource = &e.path().to_string_lossy().into_owned();

                            match registry.get(resource) {
                                Some(_v) => {
                                    // Resource is known so ignore. If it changed it was picked up in initial files
                                }
                                None => {
                                    // Resource not cached, validate existence & acquire metadata
                                    if let Ok(c) = fs::symlink_metadata(resource) {
                                        // TODO : Reduce cloning and memory allocations, also redundant to_string
                                        let new = ResourceMetadata::new(&resource.to_string(), c.is_dir(), c.is_symlink(), c.mtime(), c.len());
                                        Self::update(registry, &new.get_path().to_string(), &new);

                                        // Resource is known so ignore. If it changed it was picked up in initial files
                                        if !c.is_dir() {
                                            Self::visit(&new, visitors);
                                        } else {
                                            self.sync_dir(registry, &new, visitors);
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    fn update(registry: &mut HashMap<String, ResourceMetadata>, k: &String, v: &ResourceMetadata) {
        registry.entry(k.clone()).and_modify(|existing| {
            *existing = v.clone();
        }).or_insert(v.clone());
    }

    fn visit(cached: &ResourceMetadata, visitors: &mut [&mut dyn Visitable]) {
        for visitor in &mut *visitors {
            visitor.visit(cached);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    struct MockVisitor {
        test: String
    }

    impl Visitable for MockVisitor {
        fn visit(&mut self, resource: &ResourceMetadata) {
            // Mock implementation
            println!("test={} resource={}", self.test, resource.get_path());
        }

        fn recap(&mut self) {
            todo!()
        }

        fn name(&self) -> &'static str {
            "mock visitor"
        }
    }

    impl MockVisitor {
        pub(crate) fn new(t: &String) -> Self {
            MockVisitor {
                test : t.clone()
            }
        }
    }

    #[test]
    fn test_full_scan() {
        let mut scanner = ResourceScanner::new();
        let mut registry = HashMap::new();

        let mut v = MockVisitor::new(&String::from("test_full_scan"));
        let mut visitors: Vec<&mut dyn Visitable> = Vec::new();
        visitors.push(&mut v);

        // Create a temporary directory and some files inside for testing
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
        let file_path = temp_dir.path().join("test_file.txt");
        fs::write(&file_path, "test data").expect("Failed to write to file");

        // Perform a full scan
        scanner.full_scan(&mut registry, &file_path.to_string_lossy().to_string(), &mut visitors);

        // Assert that the registry has been populated and visitors were called
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn test_incremental_scan() {
        let mut scanner = ResourceScanner::new();
        let mut registry = HashMap::new();
        let mut v = MockVisitor::new(&String::from("test_incremental_scan"));

        let mut visitors: Vec<&mut dyn Visitable> = Vec::new();
        visitors.push(&mut v);

        // Create a temporary directory and some files inside for testing
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
        let td = temp_dir.path().to_string_lossy().to_string();

        let file_path = temp_dir.path().join("test_file.txt");
        fs::write(&file_path, "test data").expect("Failed to write to file");

        // Register root
        let p = Path::new(&td);
        let m = ResourceMetadata::new(&td, p.is_dir(), p.is_symlink(), 0, 1024);
        registry.insert(td.clone(), m);

        // Perform an incremental scan
        scanner.incremental_scan(&td, &mut registry, &mut visitors);

        // Assert that the registry has been populated and visitors were called
        assert_eq!(registry.len(), 2);
    }

    // Add more test cases for inspect_resource_for_change, sync_file, sync_dir, and other functions as needed.
}