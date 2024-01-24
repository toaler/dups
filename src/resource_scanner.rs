use std::{fs};
use std::collections::HashMap;
use std::fs::Metadata;
use std::io::Error;
use log::{debug, error};
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
            }
        }
    }

    pub(crate) fn incremental_scan(&mut self, registry: &mut HashMap<String, CachedMetadata>, visitors: &mut [&mut dyn Visitable]) {
        let keys: Vec<String> = registry.keys().cloned().collect();
        self.inspect_resources_for_change(registry, keys, visitors);
    }

    fn inspect_resources_for_change(&mut self, registry: &mut HashMap<String, CachedMetadata>, keys: Vec<String>, visitors: &mut [&mut dyn Visitable]) {
        for key in keys {
            self.inspect_resource_for_change(registry, &key, visitors);
        }
    }

    pub(crate) fn inspect_resource_for_change(&mut self, registry: &mut HashMap<String, CachedMetadata>, key: &String, visitors: &mut [&mut dyn Visitable]) {
        match registry.get_mut(key) {
            Some(cached_metadata) => {
                match self.get_authoritative_metadata(&key, cached_metadata.is_symlink()) {
                    Ok(authoritative_metadata) => {
                        if cached_metadata.modified() != authoritative_metadata.modified().unwrap() {
                            // Cached resource is invalid
                            debug!("Resource changed : is_dir={} {} new modified time {:?}", cached_metadata.is_dir(), cached_metadata.get_path(), system_time_to_string(&authoritative_metadata.modified().unwrap()));
                            if !cached_metadata.is_dir() {
                                self.sync_file(registry, key, &authoritative_metadata, visitors);
                            } else {
                                self.sync_dir(registry, key, &authoritative_metadata, visitors);
                            }
                        } else {
                            // Cached resource is fresh
                            debug!("Visiting [2] file={:?}", key);
                            Self::visit(cached_metadata, visitors);
                        }
                    }
                    Err(error) => {
                        // Authoritative metadata lookup failed
                        match error.kind() {
                            std::io::ErrorKind::NotFound => {
                                debug!("change detected : {} deleted", key);
                                registry.remove(key);
                            }
                            std::io::ErrorKind::PermissionDenied => {
                                error!("scan_resource_for_change  : Permission denied : {}", key);
                                // Additional specific error-handling logic for PermissionDenied
                            }
                            _ => {
                                // Handle other errors
                                error!("scan_resource_for_change  : Error: {} : {}", error, key);
                                // Additional generic error-handling logic
                            }
                        }
                    }
                }
            }
            _ => {
                // Shouldn't get here, all lookups are from keys that exist in cache
            }
        }
    }

    fn sync_file(&mut self, registry: &mut HashMap<String, CachedMetadata>, key: &String, current: &Metadata, visitors: &mut [&mut dyn Visitable]) {
        let mut m = self.put_metadata(registry, key, &current);
        debug!("Visiting [1] file={:?} ", &key);
        Self::visit(&mut m, visitors);
    }

    fn sync_dir(&mut self, registry: &mut HashMap<String, CachedMetadata>, key: &String, current: &Metadata, visitors: &mut [&mut dyn Visitable]) {
        let mut m = self.put_metadata(registry, key, &current);
        debug!("Visiting [2] file={:?} ", &key);
        Self::visit(&mut m, visitors);

        match fs::read_dir(key) {
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
                                        debug!("Resource changed : {} added", resource.to_string());
                                        let mut m = self.put_metadata(registry, &resource.to_string(), &c);

                                        // Resource is known so ignore. If it changed it was picked up in initial files
                                        if !c.is_dir() {
                                            debug!("Visiting [3] file={:?} ", &resource);
                                            Self::visit(&mut m, visitors);
                                        } else {
                                            self.sync_dir(registry, &resource.to_string(), &c, visitors);
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

    fn visit(cached: &mut CachedMetadata, visitors: &mut [&mut dyn Visitable]) {
        for visitor in &mut *visitors {
            visitor.visit(cached);
        }
    }

    fn put_metadata(&mut self, registry: &mut HashMap<String, CachedMetadata>, key: &String, current: &Metadata) -> CachedMetadata {
        let m = CachedMetadata::new2(&key, current.is_dir(), current.is_symlink(), current.modified().unwrap());
        registry.insert(key.clone(), m.clone());
        m
    }

    fn get_authoritative_metadata(&mut self, path: &String, is_symlink: bool) -> Result<Metadata, Error> {
        if is_symlink {
            return fs::symlink_metadata(path);
        }
        return fs::metadata(path);
    }

    #[allow(warnings)]
    pub fn get_cache_stats(&self) -> (usize, usize) {
        (self.cache_accesses, self.cache_misses)
    }
}