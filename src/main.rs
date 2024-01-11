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
use crate::scan_stats::ScanStats;
use crate::scan_stats_visitor::ScanStatsVisitor;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a directory path.");
        return;
    }

    let root_directory = &args[1];
    let root = Path::new(root_directory);
    let traverser = FileSystemTraversal;
    let mut nodewriter = NodeWriter {};
    let mut scan_stats_visitor = ScanStatsVisitor::new();

    traverser.traverse(&root, &mut scan_stats_visitor);

    let stats = scan_stats_visitor.get_stats();

    println!("{}", stats);
    // traverser.traverse(&root, &mut nodewriter);
}