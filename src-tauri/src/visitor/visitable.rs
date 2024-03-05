use std::io;
use crate::state::resource_metadata::ResourceMetadata;
use crate::visitor::tauri_logger::Logger;

pub trait Visitable {
    fn visit(&mut self, metadata: &ResourceMetadata, writer: &mut dyn io::Write, logger: &dyn Logger);

    fn recap(&mut self, w: &mut dyn io::Write, logger: &dyn Logger);

    fn name(&self) -> &'static str;
}