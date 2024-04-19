mod util;
mod state;
mod config;
mod ui;
mod services;

use crate::ui::command::scan_filesystem::scan_filesystem;
use crate::ui::command::staged_resource_manager::commit;
use log::{error, info, LevelFilter};
use std::{env, io};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::path::{PathBuf};
use std::error::Error;
use csv::{ReaderBuilder, WriterBuilder};
use tauri::{generate_context};
use state::resource_metadata::ResourceMetadata;
use services::scanner_api::visitable::Visitable;

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan_filesystem, commit])
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

        let record = match record {
            Ok(record) => record,
            Err(e) => {
                eprintln!("Failed to read a record: {:?}", e);
                continue; // Skip this record and continue with the next
            }
        };

        // Assuming the CSV file structure is [path, dir, sym, t]
        let path = match record.get(0).ok_or_else(|| io::Error::new(ErrorKind::InvalidData, "Missing path in CSV record")) {
            Ok(v) => v.to_string(),
            Err(e) => {
                eprintln!("Error parsing path: {}", e);
                continue; // Skip this record and continue with the next
            }
        };

        let is_dir = match record.get(1).ok_or_else(|| io::Error::new(ErrorKind::InvalidData, "Missing is_dir in CSV record")) {
            Ok(v) => match v.parse::<bool>() {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("Error parsing is_dir: {}", e);
                    continue;
                }
            },
            Err(e) => {
                eprintln!("Error retrieving is_dir: {}", e);
                continue;
            }
        };

        let is_symlink = match record.get(2).ok_or_else(|| io::Error::new(ErrorKind::InvalidData, "Missing is_symlink in CSV record")) {
            Ok(v) => match v.parse::<bool>() {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("Error parsing is_symlink: {}", e);
                    continue;
                }
            },
            Err(e) => {
                eprintln!("Error retrieving is_symlink: {}", e);
                continue;
            }
        };

        let modified_time = match record.get(3).ok_or_else(|| io::Error::new(ErrorKind::InvalidData, "Missing modified_time in CSV record")) {
            Ok(v) => match v.parse::<i64>() {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Error parsing modified_time: {}", e);
                    continue;
                }
            },
            Err(e) => {
                eprintln!("Error retrieving modified_time: {}", e);
                continue;
            }
        };

        let size_bytes = match record.get(4).ok_or_else(|| io::Error::new(ErrorKind::InvalidData, "Missing size in CSV record")) {
            Ok(v) => match v.parse::<u64>() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error parsing size_bytes: {}", e);
                    continue;
                }
            },
            Err(e) => {
                eprintln!("Error retrieving size_bytes: {}", e);
                continue;
            }
        };
        // Create ResourceMetadata and insert into the registry
        let resource_metadata = ResourceMetadata::new(&path, is_dir, is_symlink, modified_time, size_bytes, false);
        registry.insert(path, resource_metadata);
    }
    info!("Incremental scan detected");
    Ok(registry.clone()) // Use clone() to return a new HashMap
}
