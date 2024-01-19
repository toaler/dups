use std::{fs};
use std::collections::HashMap;
use std::fs::Metadata;
use crate::cached_metadata::CachedMetadata;
use crate::util::system_time_to_string;
use crate::visitable::Visitable;

pub struct ResourceScanner {
    cache_accesses: usize,
    cache_misses: usize,
}

impl ResourceScanner {
    pub fn new() -> ResourceScanner {
        ResourceScanner { cache_accesses: 0, cache_misses: 0 }
    }



    pub(crate) fn full_scan(&mut self, registry: &mut HashMap<String, CachedMetadata>, path: &String, visitors: &mut [&mut dyn Visitable]) {
        self.cache_accesses += 1;
        let metadata = registry.entry(path.clone()).or_insert_with(|| {
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
                        self.full_scan(registry, &e.path().to_string_lossy().to_string(), visitors);
                    }
                }
            } else {
                // Todo do something different here
            }
        }
    }

    pub(crate) fn incremental_scan(&mut self, registry: &mut HashMap<String, CachedMetadata>) {
        let keys: Vec<String> = registry.keys().cloned().collect();
        self.scan_resources_for_change(registry, keys);
    }

    fn scan_resources_for_change(&mut self, registry: &mut HashMap<String, CachedMetadata>, keys: Vec<String>) {
        for key in keys {
            self.scan_resource_for_change(registry, &key);
        }
    }

    pub(crate) fn scan_resource_for_change(&mut self, registry: &mut HashMap<String, CachedMetadata>, key: &String) {

        match fs::metadata(&key) {
            Ok(current) => {
                if let Some(cached) = registry.get_mut(key) {
                    if cached.modified() != current.modified().unwrap() {
                        println!("change detected : is_dir={} {} changed new modified time {:?}", cached.is_dir(), cached.get_path(), system_time_to_string(&current.modified().unwrap()));
                        if !cached.is_dir() {
                            self.sync_file(registry, key, &current);
                        } else {
                            self.sync_dir(registry, key, &current);
                        }
                    } else {
                        // resource current
                    }
                }
            }
            Err(error) => {
                // Handle the case when there's an error obtaining metadata
                match error.kind() {
                    std::io::ErrorKind::NotFound => {
                        eprintln!("File or directory not found.");
                        if let Some(cached) = registry.get_mut(key) {
                            if cached.is_dangling() {
                                // check if file exists via path and if it doesn't then remove it
                                return
                            }
                        }


                        // Additional specific error-handling logic for NotFound
                    }
                    std::io::ErrorKind::PermissionDenied => {
                        eprintln!("Permission denied.");
                        // Additional specific error-handling logic for PermissionDenied
                    }
                    _ => {
                        // Handle other errors
                        eprintln!("Error: {}", error);
                        // Additional generic error-handling logic
                    }
                }
                // Handle error getting file metadata
                // TODO: file may no longer exist, remove it from the data structure
                println!("change detected : {} deleted", key);
                registry.remove(key);
            }
        }
    }

    fn sync_file(&mut self, registry: &mut HashMap<String, CachedMetadata>, key: &String, current: &Metadata) {
        self.put_metadata(registry, key, &current);
        // Resource current
    }

    fn sync_dir(&mut self, registry: &mut HashMap<String, CachedMetadata>, key: &String, current: &Metadata) {
        if let Ok(children) = fs::read_dir(key) {
            for child in children {
                if let Ok(e) = child {
                    // Look only for new files/dir's added. Deleted files/dir's in a dir will get pruned on initial scan

                    let resource = &e.path().to_string_lossy().into_owned();
                    let value = registry.get(resource);

                    match value {
                        Some(_value) => {
                            // Resource is known so ignore. If it changed it was picked up in initial files
                        }
                        None => {
                            // Resource does not exist, insert value and perform additional actions
                            // Need to acquire metadata for file

                            if let Ok(c) = fs::metadata(resource) {
                                println!("change detected : {:?} added", resource);
                                self.put_metadata(registry, &resource.to_string(), &c);

                                if c.is_dir() {
                                    self.sync_dir(registry, &resource.to_string(), &c);
                                }

                                // Resource updated
                            }
                        }
                    }
                }
            }
        } else {
            // Todo do something different here
        }

        // update the changed dir metadata as well
        self.put_metadata(registry, key, &current);
    }



    pub fn add_metadata(&mut self, registry: &mut HashMap<String, CachedMetadata>, path: &String, metadata: CachedMetadata) {
        registry.insert(path.clone(), metadata);
    }
    fn put_metadata(&mut self, registry: &mut HashMap<String, CachedMetadata>, key: &String, current: &Metadata) {
        let m = CachedMetadata::new2(&key, current.is_dir(), current.is_symlink(), current.modified().unwrap());
        registry.insert(key.clone(), m);
    }

    #[allow(warnings)]
    pub fn get_cache_stats(&self) -> (usize, usize) {
        (self.cache_accesses, self.cache_misses)
    }


}