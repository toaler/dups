mod resource_scanner;
mod visitable;
mod cached_metadata;
mod scan_stats;
mod scan_stats_visitor;
mod progress_visitor;
mod util;

use std::{env, io};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path};
use std::time::Instant;
use crate::cached_metadata::CachedMetadata;
use crate::resource_scanner::ResourceScanner;
use crate::progress_visitor::ProgressVisitor;
use crate::scan_stats_visitor::ScanStatsVisitor;
use crate::util::{str_to_system_time, system_time_to_string};
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
    // let node_writer = NodeWriter::new();

    let mut visitors: Vec<&mut dyn Visitable> = Vec::new();
    visitors.push(&mut progress_visitor);
    visitors.push(&mut scan_stats_visitor);

    let start_time = Instant::now();


    // Load cache

    println!("Setup cached metadata");
    let mut scanner = ResourceScanner::new();

    if Path::new("output.txt").exists() {
        // Open the file for reading
        let file = File::open("output.txt")?;
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
                    let modified = str_to_system_time(columns[3]).unwrap();

                    let is_dir_bool = is_dir == "true";
                    let is_symlink_bool = is_symlink.trim().parse().unwrap_or(false); // Change the default if needed


                    let m = CachedMetadata::new2(p, is_dir_bool, is_symlink_bool, modified);

                    scanner.add_metadata(p, m);


                }
                Err(err) => {
                    // Handle the error if any
                    eprintln!("Error reading line: {}", err);
                }
            }
        }

        println!("Loaded {} entries into cache", scanner.metadata_size());

        println!("Starting filesystem refresh");

        let elapsed_time = start_time.elapsed();
        scanner.incremental_scan();

        println!("Finished filesystem refresh");
        println!("Total elapsed time = {:?}", elapsed_time);

    } else  {

        println!("Starting full resource scan");
        scanner.full_scan(&root, &mut visitors);
        let elapsed_time = start_time.elapsed();
        println!("Finished full resource scan elapsed time = {:?}", elapsed_time);



    }



    let mut file = File::create("output.txt")?;
    for (_key, mut m) in scanner.get_metadata() {

        let t = system_time_to_string(&m.modified());
        let output_string = format!("{},{},{},{}\n", m.is_dir(), m.is_symlink(), m.get_path(), t);
        file.write_all(output_string.as_bytes())?;
    }

    // traverser.dump_registry();

    for visitable_instance in &mut visitors {
        visitable_instance.recap();
    }



    Ok(())
}
