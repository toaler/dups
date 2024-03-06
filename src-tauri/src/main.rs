#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod util;
mod visitor;
mod state;
mod config;
mod scanner;
mod handler;

use log::{debug, error, info, LevelFilter};
use std::{env, io};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::error::Error;
use csv::{ReaderBuilder, WriterBuilder};
use tauri::{command, generate_context};
use state::resource_metadata::ResourceMetadata;
use visitor::top_k_resource_visitor::TopKResourceVisitor;
use visitor::scan_stats_visitor::ScanStatsVisitor;
use visitor::visitable::Visitable;
use visitor::progress_visitor::ProgressVisitor;
use scanner::resource_scanner::ResourceScanner;
use util::util::add_groupings_usize;

use tauri::Window;
use crate::handler::tauri_event_handler::{TauriEventHandler};

#[command]
fn emit_log(window: Window, event: &str, message: &str) {
    window.emit(event, format!("{}: {}", chrono::Local::now().format("%H:%M:%S"), message)).expect("failed to emit log event");
}

#[command]
async fn scan_filesystem(w: Window, path: &str) -> Result<String, String> {

    let temp_dir = env::temp_dir();
    let file_path = temp_dir.join("output.csv");

    let logger = TauriEventHandler {window: w};

    let path_owned = path.to_owned(); // Clone path into a new String
    info!("path = {}", path_owned);

    // Now use path_owned inside your async block
    tauri::async_runtime::spawn(async move {
        // Use path_owned instead of path
        info!("{}", path_owned);
        // Your async filesystem scanning logic here
        // Make sure to replace path with path_owned in the rest of your async block

        let root = path_owned.to_string();

        info!("Running Turbo Tasker (tt)!!!");
        debug!("Register visitors:");

        let mut scan_stats_visitor = ScanStatsVisitor::new();
        let mut progress_visitor = ProgressVisitor::new();
        let mut top_resources_visitor = TopKResourceVisitor::new();
        // let mut directory_analyzer_visitor = DirectoryAnalyzerVisitor::new();

        let mut visitors: Vec<&mut dyn Visitable> = vec![
            &mut progress_visitor,
            &mut scan_stats_visitor,
            // &mut directory_analyzer_visitor,
            &mut top_resources_visitor,
        ];

        for v in &mut *visitors {
            debug!("Visitor registered: {}", v.name());
        }

        let start_time = Instant::now();
        let mut scanner = ResourceScanner::new();
        let mut registry: HashMap<String, ResourceMetadata> = HashMap::new();
        let mut writer = io::BufWriter::new(io::stdout());

        if Path::new(file_path.to_str().unwrap()).exists() {
            info!("Incremental scan detected");
            load_registry(&mut registry, &file_path).expect("TODO: panic message");

            // Add root dir in case it's not known from previous scans
            if !registry.contains_key(&root) {
                // Create ResourceMetadata and insert into the registry
                let p = Path::new(&root);
                let m = ResourceMetadata::new(&root, p.is_dir(), p.is_symlink(), 0, 0, false);
                registry.insert(root.clone(), m);
            }

            info!("Registry loaded with {} resources", add_groupings_usize(registry.len()));
            scanner.incremental_scan(&root, &mut registry, &mut visitors, &mut writer, &logger);
        } else {
            info!("Starting full resource scan");
            scanner.full_scan(&mut registry, &root, &mut visitors, &mut writer, &logger);
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

        save_registry(&mut registry, &file_path).expect("TODO: panic message");

        for visitable_instance in &mut visitors {
            visitable_instance.recap(&mut writer, &logger);
        }

        // Since path_owned is owned by the block, there's no issue with lifetimes
        Ok(format!("Hello, {}! You've been greeted from Rust asynchronously!", path_owned))
    }).await.unwrap_or_else(|e| Err(format!("Failed to scan filesystem: {}", e)))
}

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan_filesystem, emit_log])
        .run(generate_context!())
        .expect("error while running tauri application");
}

fn save_registry(registry: &mut HashMap<String, ResourceMetadata>, file_path: &PathBuf) -> Result<(), std::io::Error> {
    info!("Saving registry");

    let file = match File::create(file_path) {
        Ok(f) => f,
        Err(e) => {
            // Log the error and return it
            error!("Error creating file: {}, error = {}", file_path.to_string_lossy(), e);
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

fn load_registry(registry: &mut HashMap<String, ResourceMetadata>, file_path: &PathBuf) -> Result<HashMap<String, ResourceMetadata>, Box<dyn Error>> {
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
