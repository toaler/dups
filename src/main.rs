mod file_system_traversal;
mod visitable;
mod node_writer;
mod metadata_state;
mod metadata_collector_visitor;
mod scan_stats;
mod scan_stats_visitor;
mod progress_visitor;
mod trie;

use std::env;
use std::path::Path;
use std::time::Instant;
use crate::file_system_traversal::FileSystemTraversal;
use crate::node_writer::NodeWriter;
use crate::progress_visitor::ProgressVisitor;
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


    // let mut node_writer = NodeWriter {};
    let mut scan_stats_visitor = ScanStatsVisitor::new();
    let mut progress_visitor = ProgressVisitor::new();
    let mut node_writer = NodeWriter::new();

    // Create a Vec<&mut dyn Foo> and add mutable references to the implementations
    let mut visitors: Vec<&mut dyn Visitable> = Vec::new();
    visitors.push(&mut scan_stats_visitor);
    visitors.push(&mut progress_visitor);
    // visitors.push(&mut node_writer);

    let start_time = Instant::now();
    traverser.traverse(&root, true, false, &mut visitors);
    let elapsed_time = start_time.elapsed();

    for visitable_instance in visitors {
        visitable_instance.recap();
    }


    // let &stats = *visitors[1].get_stats();

    println!("Whoop there it is --> {}", scan_stats_visitor.get_stats().get_directory_count());
    println!("Total elasped time = {:?}", elapsed_time);
}