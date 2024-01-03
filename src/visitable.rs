use std::path::Path;

pub trait Visitable {
    fn visit(&self, path: &Path);
}