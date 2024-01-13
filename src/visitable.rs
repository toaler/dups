use std::path::Path;

pub trait Visitable {
    fn visit(&mut self, path: &Path, is_dir: bool);

    fn recap(&mut self);
}