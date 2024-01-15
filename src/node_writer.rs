use std::fs::Metadata;
use std::path::Path;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use crate::cached_metadata::CachedMetadata;
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
    fn visit(&mut self, metadata: &mut CachedMetadata) {
    }

    fn recap(&mut self) {}
}
