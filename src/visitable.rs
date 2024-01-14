use std::fs::Metadata;
use std::path::Path;
use crate::cached_metadata::CachedMetadata;

pub trait Visitable {
    fn visit(&mut self, path: &Path, metadata: &mut CachedMetadata);

    fn recap(&mut self);
}