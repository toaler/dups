mod resource_scanner;
mod visitable;
mod cached_metadata;
mod scan_stats;
mod scan_stats_visitor;
mod progress_visitor;
mod util;

use std::{env, io};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use std::path::{Path};
use std::time::Instant;
use crate::cached_metadata::CachedMetadata;
use crate::resource_scanner::ResourceScanner;
use crate::progress_visitor::ProgressVisitor;
use crate::scan_stats_visitor::ScanStatsVisitor;
use crate::util::{str_to_system_time, system_time_to_string};
use crate::visitable::Visitable;

fn main() -> Result<(), io::Error> {
    let root = match process_args() {
        Ok(value) => value,
        Err(value) => return value,
    };

    println!("Build visitors");
    let mut scan_stats_visitor = ScanStatsVisitor::new();
    let mut progress_visitor = ProgressVisitor::new();
    let mut visitors: Vec<&mut dyn Visitable> = Vec::new();
    visitors.push(&mut progress_visitor);
    visitors.push(&mut scan_stats_visitor);

    let start_time = Instant::now();
    let mut scanner = ResourceScanner::new();
    let mut registry: HashMap<String, CachedMetadata> = HashMap::new();

    if Path::new("output.txt").exists() {
        load_registry(&mut scanner, &mut registry)?;
        scanner.incremental_scan(&mut registry);
    } else {
        println!("Starting full resource scan");
        scanner.full_scan(&mut registry, &root, &mut visitors);
        println!("Finished full resource scan elapsed time = {:?}", start_time.elapsed());
    }
    println!("Total elapsed time = {:?}", start_time.elapsed());

    save_registry(&mut registry)?;

    for visitable_instance in &mut visitors {
        visitable_instance.recap();
    }

    Ok(())
}

fn process_args() -> Result<String, Result<(), Error>> {
    let args: Vec<String> = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        eprintln!("Please provide a directory path.");
        return Err(Err(io::Error::new(io::ErrorKind::InvalidInput, "Missing directory path")));
    }

    let r = args[1].clone();
    Ok(r)
}

// TODO figure out better encoding that deals with file characters like whitespace/comma
fn save_registry(registry: &mut HashMap<String, CachedMetadata>) -> Result<(), Error> {
    let mut file = File::create("output.txt")?;
    for (_key, m) in registry {
        let t = system_time_to_string(&m.modified());
        let output_string = format!("{},{},{},{}\n",
                                    m.is_dir(), m.is_symlink(), m.get_path(), t);
        file.write_all(output_string.as_bytes())?;
    }
    Ok(())
}

fn load_registry(scanner: &mut ResourceScanner, mut registry: &mut HashMap<String, CachedMetadata>) -> Result<(), Error> {
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

                scanner.add_metadata(&mut registry, p, m);
            }
            Err(err) => {
                // Handle the error if any
                eprintln!("Error reading line: {}", err);
            }
        }
    }

    println!("Loaded {} entries into cache", registry.len());
    println!("Starting filesystem refresh");


    Ok(())
}
