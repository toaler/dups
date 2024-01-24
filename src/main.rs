mod resource_scanner;
mod visitable;
mod cached_metadata;
mod scan_stats;
mod scan_stats_visitor;
mod progress_visitor;
mod util;

use log::{debug, error, info};

use std::{env, io};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader};
use std::path::{Path};
use std::time::{Instant, SystemTime};
use std::error::Error;
use csv::{ReaderBuilder, WriterBuilder};
use crate::cached_metadata::CachedMetadata;
use crate::resource_scanner::ResourceScanner;
use crate::progress_visitor::ProgressVisitor;
use crate::scan_stats_visitor::ScanStatsVisitor;
use crate::util::{str_to_system_time, system_time_to_string};
use crate::visitable::Visitable;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Running dups!!!");

    let root = process_args()?;

    debug!("Register visitors:");
    let mut scan_stats_visitor = ScanStatsVisitor::new();
    let mut progress_visitor = ProgressVisitor::new();
    let mut visitors: Vec<&mut dyn Visitable> = Vec::new();
    visitors.push(&mut progress_visitor);
    visitors.push(&mut scan_stats_visitor);

    for v in &mut *visitors {
        debug!("Visitor registered: {}", v.name());
    }

    let start_time = Instant::now();
    let mut scanner = ResourceScanner::new();
    let mut registry: HashMap<String, CachedMetadata> = HashMap::new();

    if Path::new("output.csv").exists() {
        info!("Incremental scan detected");
        load_registry("output.csv", &mut registry)?;

        // Add root dir in case it's not known from previous scans
        if !registry.contains_key(&root) {
            // Create CachedMetadata and insert into the registry
            let p = Path::new(&root);
            let m = CachedMetadata::new2(&root, p.is_dir(), p.is_symlink(), SystemTime::now());
            registry.insert(root.clone(), m);
        }

        info!("Registry loaded with {} resources", registry.len());
        scanner.incremental_scan(&mut registry, &mut visitors);
    } else {
        info!("Starting full resource scan");
        scanner.full_scan(&mut registry, &root, &mut visitors);
        info!("Finished full resource scan elapsed time = {:?}", start_time.elapsed());
    }
    info!("Total elapsed time = {:?}", start_time.elapsed());

    save_registry(&mut registry)?;

    for visitable_instance in &mut visitors {
        visitable_instance.recap();
    }

    Ok(())
}

fn process_args() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        error!("Please provide a directory path.");
        return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "Missing directory path")));
    }

    let r = args[1].clone();
    Ok(r)
}

// TODO figure out better encoding that deals with file characters like whitespace/comma
fn save_registry(registry: &mut HashMap<String, CachedMetadata>) -> Result<(), std::io::Error> {
    let file = File::create("output.csv")?;

    // Create a CSV writer
    let mut writer = WriterBuilder::new().from_writer(file);

    // Write header
    for (_key, m) in registry {
        let t = system_time_to_string(&m.modified());
        let path = m.get_path().clone();
        let dir = m.is_dir().to_string();
        let sym = m.is_symlink().to_string();

        writer.write_record(&[path, dir, sym, t])?;
    }

    writer.flush()?;

    info!("CSV file created successfully.");

    Ok(())
}

fn load_registry(file_path: &str, registry: &mut HashMap<String, CachedMetadata>) -> Result<HashMap<String, CachedMetadata>, Box<dyn Error>> {
    // Open the file using BufReader for efficiency
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Create a CSV reader
    let mut csv_reader = ReaderBuilder::new().has_headers(false).from_reader(reader);

    // Iterate over CSV records
    for record in csv_reader.records() {
        let record = record?;

        // Assuming the CSV file structure is [path, dir, sym, t]
        let path = record.get(0).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing path in CSV record"))?.to_string();
        let is_dir: bool = record.get(1).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing is_dir in CSV record"))?.parse()?;
        let is_symlink: bool = record.get(2).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing is_symlink in CSV record"))?.parse()?;
        // Assuming the system_time_from_string function parses the time correctly
        let modified_time = str_to_system_time(record.get(3).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing modified_time in CSV record"))?)?;

        // Create CachedMetadata and insert into the registry
        let cached_metadata = CachedMetadata::new2(&path, is_dir, is_symlink, modified_time);
        registry.insert(path, cached_metadata);
    }

    Ok(registry.clone()) // Use clone() to return a new HashMap
}
