use crate::resource_metadata::ResourceMetadata;

pub trait Visitable {
    fn visit(&mut self, metadata: &ResourceMetadata);

    fn recap(&mut self);

    fn name(&self) -> &'static str;
}