mod util;
mod visitor;
mod state;
mod config;
mod scanner;
mod handler;
mod command;

use crate::command::scan_filesystem::scan_filesystem;
use log::{error, info, LevelFilter};
use std::{env, io};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{PathBuf};
use std::error::Error;
use csv::{ReaderBuilder, WriterBuilder};
use tauri::{generate_context};
use state::resource_metadata::ResourceMetadata;
use visitor::visitable::Visitable;

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan_filesystem])
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
