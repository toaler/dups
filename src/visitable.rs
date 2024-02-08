use std::io;
use crate::resource_metadata::ResourceMetadata;

pub trait Visitable {
    fn visit(&mut self, metadata: &ResourceMetadata);

    fn recap(&mut self, w: &mut dyn io::Write);

    fn name(&self) -> &'static str;
}