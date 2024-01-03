mod file_system_traversal;
mod visitable;
mod node_writer;

use std::env;
use std::path::Path;
use crate::file_system_traversal::FileSystemTraversal;
use crate::node_writer::NodeWriter;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a directory path.");
        return;
    }

    let root_directory = &args[1];
    let root = Path::new(root_directory);
    let traverser = FileSystemTraversal;
    let nodewriter = NodeWriter {};

    traverser.traverse(&root, &nodewriter);
}