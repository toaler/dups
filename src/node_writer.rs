use std::fs::Metadata;
use std::path::Path;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use crate::scan_stats::ScanStats;
use crate::scan_stats_visitor::ScanStatsVisitor;
use crate::visitable::Visitable;

pub struct NodeWriter {}

impl NodeWriter {
    pub(crate) fn new() -> Self {
        NodeWriter {}
    }
}

impl Visitable for NodeWriter {
    fn visit(&mut self, path: &Path, metadata: &Metadata) {
        println!("file={:?},dir={},file={},modified={:?}", path.file_name(),
                 metadata.is_dir(), metadata.is_file(), metadata.modified());
    }

    fn recap(&mut self) {}
}
