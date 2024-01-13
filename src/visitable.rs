use std::fs::Metadata;
use std::path::Path;

pub trait Visitable {
    fn visit(&mut self, path: &Path, metadata: &Metadata);

    fn recap(&mut self);
}