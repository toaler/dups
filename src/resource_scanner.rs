use std::collections::HashMap;

use std::fs;
use std::os::unix::fs::MetadataExt;

use log::{debug, info};

use crate::resource_metadata::ResourceMetadata;
use crate::visitable::Visitable;


pub struct ResourceScanner {
    cache_accesses: usize,
    cache_misses: usize,
}

impl ResourceScanner {
    pub fn new() -> ResourceScanner {
        ResourceScanner { cache_accesses: 0, cache_misses: 0 }
    }

    pub(crate) fn full_scan(&mut self, registry: &mut HashMap<String, ResourceMetadata>, path: &String, visitors: &mut [&mut dyn Visitable]) {
        self.cache_accesses += 1;
        let metadata = registry.entry(path.clone()).or_insert_with(|| {
            self.cache_misses += 1;

            let m = fs::symlink_metadata(path).unwrap();
            ResourceMetadata::new(&path, m.is_dir(), m.is_symlink(), m.mtime())
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
            debug!("evaluating key {}", key);
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

                            let current = ResourceMetadata::new(&key, value.is_dir(), value.is_symlink(), mtime);
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
                                        let new = ResourceMetadata::new(&resource.to_string(), c.is_dir(), c.is_symlink(), c.mtime());
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

    #[allow(warnings)]
    pub fn get_cache_stats(&self) -> (usize, usize) {
        (self.cache_accesses, self.cache_misses)
    }
}