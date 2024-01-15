mod file_system_traversal;
mod visitable;
mod node_writer;
mod cached_metadata;
mod scan_stats;
mod scan_stats_visitor;
mod progress_visitor;
mod trie;

use std::{env, io};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use crate::cached_metadata::CachedMetadata;
use crate::file_system_traversal::FileSystemTraversal;
use crate::node_writer::NodeWriter;
use crate::progress_visitor::ProgressVisitor;
use crate::scan_stats_visitor::ScanStatsVisitor;
use crate::visitable::Visitable;

fn main() -> Result<(), io::Error>{
    println!("Running dups!");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide a directory path.");
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Missing directory path"));
    }

    let root = &args[1];

    println!("Build visitors");
    let mut scan_stats_visitor = ScanStatsVisitor::new();
    let mut progress_visitor = ProgressVisitor::new();
    let mut node_writer = NodeWriter::new();

    let mut visitors: Vec<&mut dyn Visitable> = Vec::new();
    visitors.push(&mut progress_visitor);
    visitors.push(&mut scan_stats_visitor);

    let start_time = Instant::now();


    // Load cache

    println!("Setup cached metadata");
    let mut traverser = FileSystemTraversal::new_with_cache();

    if Path::new("output.txt").exists() {
        // Open the file for reading
        let mut file = File::open("output.txt")?;
        let reader = BufReader::new(file);

        // Iterate over each line in the file
        for line in reader.lines() {
            // Handle each line as needed
            match line {
                Ok(row) => {
                    // Split the row by comma
                    let columns: Vec<&str> = row.split(',').collect();

                    // Process each column (print them in this case)
                    let is_dir = columns[0];
                    let is_symlink = columns[1];
                    let p = &columns.get(2).unwrap().trim().to_string();

                    let is_dir_bool = is_dir == "true";
                    let is_symlink_bool = is_symlink.trim().parse().unwrap_or(false); // Change the default if needed


                    let m = CachedMetadata::new2(p, is_dir_bool, is_symlink_bool);

                    traverser.add_metadata(p, m);


                }
                Err(err) => {
                    // Handle the error if any
                    eprintln!("Error reading line: {}", err);
                }
            }
        }

        println!("Loaded {} entries into cache", traverser.metadata_size());

        println!("Starting filesystem refresh");

        let elapsed_time = start_time.elapsed();
        traverser.refresh();

        println!("Finished filesystem refresh");
        println!("Total elapsed time = {:?}", elapsed_time);

    } else  {

        println!("Starting filesystem traverse");

        traverser.traverse(&root, &mut visitors);
        println!("Finished filesystem traverse");
        let elapsed_time = start_time.elapsed();
        let (accesses, misses) = traverser.get_cache_stats();

    }



    let mut file = File::create("output.txt")?;
    for (key, mut m) in traverser.get_metadata() {

        let output_string = format!("{}, {}, {}\n", m.is_dir(), m.is_symlink(), m.get_path());
        // println!("{}", output_string);
        file.write_all(output_string.as_bytes())?;
    }

    // traverser.dump_registry();





    for visitable_instance in &mut visitors {
        visitable_instance.recap();
    }



    Ok(())
}
