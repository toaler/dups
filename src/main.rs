mod file_system_traversal;
mod visitable;
mod node_writer;
mod metadata_state;
mod metadata_collector_visitor;
mod scan_stats;
mod scan_stats_visitor;

use std::env;
use std::path::Path;
use crate::file_system_traversal::FileSystemTraversal;
use crate::node_writer::NodeWriter;
use crate::scan_stats_visitor::ScanStatsVisitor;
use crate::visitable::Visitable;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a directory path.");
        return;
    }

    let root_directory = &args[1];
    let root = Path::new(root_directory);
    let traverser = FileSystemTraversal;
    let node_writer: Box<NodeWriter> = Box::new(NodeWriter {});
    let scan_stats_visitor: Box<ScanStatsVisitor> = Box::new(ScanStatsVisitor::new());

    let mut visitors: Vec<Box<dyn Visitable>> = vec![node_writer, scan_stats_visitor];

    traverser.traverse(&root, &mut visitors);

    for visitable_instance in visitors {
        visitable_instance.recap();
    }

    // let &stats = *visitors[1].get_stats();

    // println!("{}", stats);
}