use std::collections::HashMap;
use std::{fs, io};
use std::os::unix::fs::MetadataExt;
use log::{debug, info};
use crate::state::resource_metadata::ResourceMetadata;
use crate::services::scanner_api::event_handler::EventHandler;
use crate::services::scanner_impl::visitor::visitable::Visitable;


pub struct ResourceScanner {
    added_files: u64,
    added_dirs: u64,
    deleted_files: u64,
    deleted_dirs: u64,
    updated_files: u64,
    updated_dirs: u64,
}

impl ResourceScanner {
    pub fn new() -> ResourceScanner {
        ResourceScanner {
            added_files: 0,
            added_dirs: 0,
            deleted_files: 0,
            deleted_dirs: 0,
            updated_files: 0,
            updated_dirs: 0,
        }
    }

    pub fn added_files(&self) -> u64 { self.added_files }
    pub fn added_dirs(&self) -> u64 { self.added_dirs }
    pub fn updated_files(&self) -> u64 { self.updated_files }
    pub fn updated_dirs(&self) -> u64 { self.updated_dirs }
    pub fn deleted_files(&self) -> u64 { self.deleted_files }
    pub fn deleted_dirs(&self) -> u64 { self.deleted_dirs }

    #[warn(clippy::only_used_in_recursion)]
    pub fn full_scan(&mut self, registry: &mut HashMap<String, ResourceMetadata>, path: &String, visitors: &mut [&mut dyn Visitable], writer: &mut dyn io::Write, logger: &dyn EventHandler) {
        let metadata = registry.entry(path.clone()).or_insert_with(|| {
            let m = fs::symlink_metadata(path).unwrap();
            ResourceMetadata::new(path, m.is_dir(), m.is_symlink(), m.mtime(), m.len(), false)
        });

        for visitor in &mut *visitors {
            visitor.visit(metadata, writer, logger);
        }

        if metadata.is_dir() && !metadata.is_symlink() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries {
                    if let Ok(e) = entry {
                        self.full_scan(registry, &e.path().to_string_lossy().to_string(), visitors, writer, logger);
                    }
                }
            }
        }
    }

    pub fn incremental_scan(&mut self, root: &String, registry: &mut HashMap<String, ResourceMetadata>, visitors: &mut [&mut dyn Visitable], writer: &mut dyn io::Write, logger: &dyn EventHandler) {
        let mut keys: Vec<String> = registry
            .keys()
            .filter(|key| key.starts_with(root))
            .cloned()
            .collect();

        // Sort the keys so lstat lookups have locality
        keys.sort();
        info!("Scanning resources={}", keys.len());

        self.inspect_resources_for_change(registry, keys, visitors, writer, logger);
    }

    fn inspect_resources_for_change(&mut self, registry: &mut HashMap<String, ResourceMetadata>, keys: Vec<String>, visitors: &mut [&mut dyn Visitable], writer: &mut dyn io::Write, logger: &dyn EventHandler) {
        for key in keys {
            self.inspect_resource_for_change(registry, &key, visitors, writer, logger);
        }
    }

    fn inspect_resource_for_change(&mut self, registry: &mut HashMap<String, ResourceMetadata>, key: &String, visitors: &mut [&mut dyn Visitable], writer: &mut dyn io::Write, logger: &dyn EventHandler) {
        let resource = registry.get_mut(key);
        match resource {
            Some(ref cached_metadata) => {
                match fs::symlink_metadata(key) {
                    Ok(value) => {
                        let mtime = value.mtime();

                        if cached_metadata.modified() != mtime {
                            // Cached resource is invalid
                            debug!("Resource changed : is_dir={} {} new modified time {:?}", value.is_dir(), key, mtime);

                            let current = ResourceMetadata::new(key, value.is_dir(), value.is_symlink(), mtime, value.len(), false);
                            if !cached_metadata.is_dir() {
                                self.sync_file(registry, &current, visitors, writer, logger);
                            } else {
                                self.sync_dir(registry, &current, visitors, writer, logger);
                            }
                        } else {
                            // Cached resource is fresh
                            Self::visit(cached_metadata, visitors, writer, logger);
                        }
                    }
                    Err(_value) => {
                        debug!("change detected : {} deleted", key);
                        if resource.unwrap().is_dir() {
                            self.deleted_dirs += 1;
                        } else {
                            self.deleted_files += 1;
                        }
                        registry.remove(key);
                    }
                }
            }
            _ => {
                // Shouldn't get here, all lookups are from keys that exist in cache
            }
        }
    }

    fn sync_file(&mut self, registry: &mut HashMap<String, ResourceMetadata>, current: &ResourceMetadata, visitors: &mut [&mut dyn Visitable], writer: &mut dyn io::Write, logger: &dyn EventHandler) {
        Self::update(registry, current.get_path(), current);
        self.updated_files += 1;
        Self::visit(current, visitors, writer, logger);
    }

    fn sync_dir(&mut self, registry: &mut HashMap<String, ResourceMetadata>, current: &ResourceMetadata, visitors: &mut [&mut dyn Visitable], writer: &mut dyn io::Write, logger: &dyn EventHandler) {
        debug!("Resource changed : {}", current.get_path());

        Self::update(registry, current.get_path(), current);
        self.updated_dirs += 1;
        Self::visit(current, visitors, writer, logger);

        match fs::read_dir(current.get_path()) {
            Ok(children) => {
                for child in children {
                    match child {
                        Ok(e) => {
                            let resource = &e.path().to_string_lossy().into_owned();

                            match registry.get(resource) {
                                Some(_v) => {
                                    // Resource is known so ignore. If it changed it was picked as it's scanned
                                }
                                None => {
                                    // Resource not cached, validate existence & acquire metadata
                                    if let Ok(c) = fs::symlink_metadata(resource) {
                                        let new = ResourceMetadata::new(&resource.to_string(), c.is_dir(), c.is_symlink(), c.mtime(), c.len(), false);
                                        Self::update(registry, &new.get_path().to_string(), &new);

                                        if !c.is_dir() {
                                            self.added_files += 1;
                                            Self::visit(&new, visitors, writer, logger);
                                        } else {
                                            self.sync_dir(registry, &new, visitors, writer, logger);
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

    fn update(registry: &mut HashMap<String, ResourceMetadata>, k: &str, v: &ResourceMetadata) {
        registry.entry(k.to_string()).and_modify(|existing| {
            *existing = v.clone();
        }).or_insert(v.clone());
    }

    fn visit(cached: &ResourceMetadata, visitors: &mut [&mut dyn Visitable], writer: &mut dyn io::Write, logger: &dyn EventHandler) {
        for visitor in &mut *visitors {
            visitor.visit(cached, writer, logger);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::path::Path;
    use crate::services::scanner_impl::noop_event_handler::NoopEventHandler;
    use super::*;

    struct MockVisitor {
        test: String,
    }

    impl Visitable for MockVisitor {
        fn visit(&mut self, resource: &ResourceMetadata, _writer: &mut dyn io::Write, _logger: &dyn EventHandler) {
            // Mock implementation
            println!("test={} resource={}", self.test, resource.get_path());
        }

        fn recap(&mut self, _w: &mut dyn io::Write, _logger: &dyn EventHandler) {}

        fn name(&self) -> &'static str {
            "mock visitor"
        }
    }

    impl MockVisitor {
        pub fn new(t: &String) -> Self {
            MockVisitor {
                test: t.clone()
            }
        }
    }

    #[test]
    fn test_full_scan() {
        let mut scanner = ResourceScanner::new();
        let mut registry = HashMap::new();
        let mut buffer: Vec<u8> = Vec::new();
        let mut writer = io::BufWriter::new(&mut buffer);

        let mut v = MockVisitor::new(&String::from("test_full_scan"));
        let mut visitors: Vec<&mut dyn Visitable> = Vec::new();
        visitors.push(&mut v);

        // Create a temporary directory and some files inside for testing
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
        let file_path = temp_dir.path().join("test_file.txt");
        fs::write(&file_path, "test data").expect("Failed to write to file");
        let logger = NoopEventHandler{};
        // Perform a full scan
        scanner.full_scan(&mut registry, &file_path.to_string_lossy().to_string(), &mut visitors, &mut writer, &logger);

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
        let m = ResourceMetadata::new(&td, p.is_dir(), p.is_symlink(), 0, 1024, false);
        registry.insert(td.clone(), m);

        let mut buffer: Vec<u8> = Vec::new();
        let mut writer = io::BufWriter::new(&mut buffer);
        let logger = NoopEventHandler{};
        // Perform an incremental scan
        scanner.incremental_scan(&td, &mut registry, &mut visitors, &mut writer, &logger);

        // Assert that the registry has been populated and visitors were called
        assert_eq!(registry.len(), 2);
    }

    // Add more test cases for inspect_resource_for_change, sync_file, sync_dir, and other functions as needed.
}