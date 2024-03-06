use std::io;
use crate::state::resource_metadata::ResourceMetadata;
use crate::visitor::tauri_logger::EventHandler;

pub trait Visitable {
    fn visit(&mut self, metadata: &ResourceMetadata, writer: &mut dyn io::Write, logger: &dyn EventHandler);

    fn recap(&mut self, w: &mut dyn io::Write, logger: &dyn EventHandler);

    fn name(&self) -> &'static str;
}