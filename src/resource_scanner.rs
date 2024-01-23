use std::{fs};
use std::collections::HashMap;
use std::fs::Metadata;
use std::io::Error;
use log::{debug, error, info};
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
        self.scan_resources_for_change(registry, keys, visitors);
    }

    fn scan_resources_for_change(&mut self, registry: &mut HashMap<String, CachedMetadata>, keys: Vec<String>, visitors: &mut [&mut dyn Visitable]) {
        for key in keys {
            self.scan_resource_for_change(registry, &key, visitors);
        }
    }

    fn get_metadata(&mut self, path: &String, is_symlink: bool) -> Result<Metadata, Error> {
        if is_symlink {
            return fs::symlink_metadata(path);
        }
        return fs::metadata(path);
    }

    pub(crate) fn scan_resource_for_change(&mut self, registry: &mut HashMap<String, CachedMetadata>, key: &String, visitors: &mut [&mut dyn Visitable]) {
        if let Some(cached) = registry.get_mut(key) {
            match self.get_metadata(&key, cached.is_symlink()) {
                Ok(current) => {
                    if cached.modified() != current.modified().unwrap() {
                        info!("change detected : is_dir={} {} changed new modified time {:?}", cached.is_dir(), cached.get_path(), system_time_to_string(&current.modified().unwrap()));
                        if !cached.is_dir() {
                            self.sync_file(registry, key, &current, visitors);
                        } else {
                            self.sync_dir(registry, key, &current, visitors);
                        }
                    } else {
                        debug!("Visiting file={:?} ", key);
                        for visitor in &mut *visitors {
                            visitor.visit(cached);
                        }
                    }
                }
                Err(error) => {
                    // Handle the case when there's an error obtaining metadata
                    match error.kind() {
                        std::io::ErrorKind::NotFound => {
                            info!("change detected : {} deleted", key);
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
    }

    fn sync_file(&mut self, registry: &mut HashMap<String, CachedMetadata>, key: &String, current: &Metadata, visitors: &mut [&mut dyn Visitable]) {
        self.put_metadata(registry, key, &current, visitors);
    }

    fn sync_dir(&mut self, registry: &mut HashMap<String, CachedMetadata>, key: &String, current: &Metadata, visitors: &mut [&mut dyn Visitable]) {
        if let Ok(children) = fs::read_dir(key) {
            for child in children {
                if let Ok(e) = child {

                    let resource = &e.path().to_string_lossy().into_owned();
                    let value = registry.get(resource);

                    match value {
                        Some(v) => {
                            // Resource is known so ignore. If it changed it was picked up in initial files
                            for visitor in &mut *visitors {
                                visitor.visit(&mut v.clone());
                            }
                        }
                        None => {
                            // Resource does not exist, insert value and perform additional actions
                            // Need to acquire metadata for file

                            if let Ok(c) = fs::metadata(resource) {
                                info!("change detected : {:?} added", resource);
                                self.put_metadata(registry, &resource.to_string(), &c, visitors);
                                // Resource is known so ignore. If it changed it was picked up in initial files


                                if c.is_dir() {
                                    self.sync_dir(registry, &resource.to_string(), &c, visitors);
                                }
                            }
                        }
                    }
                }
            }
        }

        self.put_metadata(registry, key, &current, visitors);
    }

    fn put_metadata(&mut self, registry: &mut HashMap<String, CachedMetadata>, key: &String, current: &Metadata, visitors: &mut [&mut dyn Visitable]) {
        let m = CachedMetadata::new2(&key, current.is_dir(), current.is_symlink(), current.modified().unwrap());
        registry.insert(key.clone(), m.clone());
        debug!("Visiting file={:?} ", key);
        for visitor in &mut *visitors {
            visitor.visit(&mut m.clone());
        }
    }

    #[allow(warnings)]
    pub fn get_cache_stats(&self) -> (usize, usize) {
        (self.cache_accesses, self.cache_misses)
    }
}