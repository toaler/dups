mod resource_scanner;
mod visitable;
mod scan_stats;
mod scan_stats_visitor;
mod progress_visitor;
mod util;
mod resource_metadata;
mod largest_files_vistor;
mod directory_analyzer_visitor;
mod turbo_tasker_cli_config;

use crate::turbo_tasker_cli_config::{Command, TurboTaskerApp};
use log::{debug, error, info};
use std::io;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader};
use std::path::{Path};
use std::time::{Instant};
use std::error::Error;
use std::process::exit;
use clap::{CommandFactory, Parser};
use csv::{ReaderBuilder, WriterBuilder};
use env_logger::Env;
use crate::directory_analyzer_visitor::DirectoryAnalyzerVisitor;
use crate::resource_scanner::ResourceScanner;
use crate::progress_visitor::ProgressVisitor;
use crate::largest_files_vistor::Top50LargestResources;
use crate::resource_metadata::ResourceMetadata;
use crate::scan_stats_visitor::ScanStatsVisitor;
use crate::util::{add_groupings_usize};
use crate::visitable::Visitable;


fn main() -> Result<(), Box<dyn Error>> {
    // TODO better error handling for bubbled up Err's
    // TODO add flag to enable fingerprinting

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).target(env_logger::Target::Stdout).init();


    let cli = TurboTaskerApp::parse();
    match cli.cmd {
        Some(Command::Disk { duplicate_detection: _, root, .. }) => {
            info!("Running Turbo Tasker (tt)!!!");
            debug!("Register visitors:");

            let mut scan_stats_visitor = ScanStatsVisitor::new();
            let mut progress_visitor = ProgressVisitor::new();
            let mut top_resources_visitor = Top50LargestResources::new();
            let mut directory_analyzer_visitor = DirectoryAnalyzerVisitor::new();

            let mut visitors: Vec<&mut dyn Visitable> = vec![
                &mut progress_visitor,
                &mut scan_stats_visitor,
                &mut directory_analyzer_visitor,
                &mut top_resources_visitor,
            ];

            for v in &mut *visitors {
                debug!("Visitor registered: {}", v.name());
            }

            let start_time = Instant::now();
            let mut scanner = ResourceScanner::new();
            let mut registry: HashMap<String, ResourceMetadata> = HashMap::new();

            if Path::new("output.csv").exists() {
                info!("Incremental scan detected");
                load_registry("output.csv", &mut registry)?;

                // Add root dir in case it's not known from previous scans
                if !registry.contains_key(&root) {
                    // Create ResourceMetadata and insert into the registry
                    let p = Path::new(&root);
                    let m = ResourceMetadata::new(&root, p.is_dir(), p.is_symlink(), 0, 0, false);
                    registry.insert(root.clone(), m);
                }

                info!("Registry loaded with {} resources", add_groupings_usize(registry.len()));
                scanner.incremental_scan(&root, &mut registry, &mut visitors);
            } else {
                info!("Starting full resource scan");
                scanner.full_scan(&mut registry, &root, &mut visitors);
                info!("Finished full resource scan elapsed time = {:?}", start_time.elapsed());
            }
            info!("Change Stats : ");
            info!("added files   = {}", scanner.added_files());
            info!("added dirs    = {}", scanner.added_dirs());
            info!("updated files = {}", scanner.updated_files());
            info!("updated dirs  = {}", scanner.updated_dirs());
            info!("deleted files = {}", scanner.deleted_files());
            info!("deleted dirs  = {}", scanner.deleted_dirs());
            info!("elapsed time  = {:?}", start_time.elapsed());

            save_registry(&mut registry)?;

            for visitable_instance in &mut visitors {
                visitable_instance.recap();
            }
        },
        _ => {
            let help = TurboTaskerApp::command().render_long_help();
            println!("{help}");
            exit(1);
        }
    }

    Ok(())
}

fn save_registry(registry: &mut HashMap<String, ResourceMetadata>) -> Result<(), std::io::Error> {
    info!("Saving registry");

    // Attempt to create the file
    let out = "output.csv";
    let file = match File::create(out) {
        Ok(f) => f,
        Err(e) => {
            // Log the error and return it
            error!("Error creating file: {}, error = {}", out, e);
            return Err(e);
        }
    };

    // Create a CSV writer
    let mut writer = WriterBuilder::new().from_writer(file);

    // Write header
    for m in registry.values() {
        let t = m.modified().to_string();
        let path = m.get_path().clone();
        let dir = m.is_dir().to_string();
        let sym = m.is_symlink().to_string();
        let size = m.size_bytes().to_string();

        writer.write_record(&[path, dir, sym, t, size])?;
    }

    writer.flush()?;

    info!("Persisted registry");

    Ok(())
}

fn load_registry(file_path: &str, registry: &mut HashMap<String, ResourceMetadata>) -> Result<HashMap<String, ResourceMetadata>, Box<dyn Error>> {
    // TODO : Filter what is loaded to match the root dir that was passed in
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
        let modified_time = record.get(3).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing modified_time in CSV record"))?.parse::<i64>()?;
        let size_bytes: u64 = record.get(4).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing size in CSV record"))?.parse::<u64>()?;

        // Create ResourceMetadata and insert into the registry
        let resource_metadata = ResourceMetadata::new(&path, is_dir, is_symlink, modified_time, size_bytes, false);
        registry.insert(path, resource_metadata);
    }

    Ok(registry.clone()) // Use clone() to return a new HashMap
}
