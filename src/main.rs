mod file_system_traversal;
mod visitable;
mod node_writer;
mod cached_metadata;
mod scan_stats;
mod scan_stats_visitor;
mod progress_visitor;
mod trie;

use std::env;
use std::path::Path;
use std::time::Instant;
use crate::cached_metadata::CachedMetadata;
use crate::file_system_traversal::FileSystemTraversal;
use crate::node_writer::NodeWriter;
use crate::progress_visitor::ProgressVisitor;
use crate::scan_stats_visitor::ScanStatsVisitor;
use crate::visitable::Visitable;

fn main() {
    println!("Running dups!");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a directory path.");
        return;
    }

    let root_directory = &args[1];
    let root = Path::new(root_directory);
    let traverser = FileSystemTraversal;


    println!("Build visitors");
    // let mut node_writer = NodeWriter {};
    let mut scan_stats_visitor = ScanStatsVisitor::new();
    let mut progress_visitor = ProgressVisitor::new();
    let mut node_writer = NodeWriter::new();

    // Create a Vec<&mut dyn Foo> and add mutable references to the implementations
    let mut visitors: Vec<&mut dyn Visitable> = Vec::new();
    visitors.push(&mut progress_visitor);
    visitors.push(&mut scan_stats_visitor);
    // visitors.push(&mut node_writer);

    let start_time = Instant::now();

    println!("Setup cached metadata");
    let mut metadata = CachedMetadata::new(root);
    println!("Starting filesystem traverse");
    traverser.traverse(&root, &mut metadata, &mut visitors);
    println!("Finished filesystem traverse");
    let elapsed_time = start_time.elapsed();

    for visitable_instance in visitors {
        visitable_instance.recap();
    }

    println!("Total elasped time = {:?}", elapsed_time);
}