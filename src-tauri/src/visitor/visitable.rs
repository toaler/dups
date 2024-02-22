use std::io;
use crate::state::resource_metadata::ResourceMetadata;

pub trait Visitable {
    fn visit(&mut self, metadata: &ResourceMetadata, writer: &mut dyn io::Write);

    fn recap(&mut self, w: &mut dyn io::Write);

    fn name(&self) -> &'static str;
}