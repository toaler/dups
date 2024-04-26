use std::collections::HashMap;
use std::{env};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;
use std::time::{Duration, Instant};
use log::{debug, info};
use tauri::command;
use rodio::{Decoder, OutputStream, Source};
use tokio::task; // Import tokio's task for spawning async tasks
use crate::ui::handler::tauri_event_handler::TauriEventHandler;
use crate::state::resource_metadata::ResourceMetadata;
use crate::services::scanner_impl::{
    resource_scanner::ResourceScanner,
    visitor::{progress_visitor::ProgressVisitor, scan_stats_visitor::ScanStatsVisitor, top_k_resource_visitor::TopKResourceVisitor}
};
use crate::services::scanner_api::visitable::Visitable;
use crate::{load_registry, save_registry};

#[command]
pub async fn scan_filesystem(w: tauri::Window, path: &str) -> Result<&'static str, String> {
    let temp_dir = env::temp_dir();
    let file_path = temp_dir.join("output.csv");
    let logger = TauriEventHandler { window: w };
    let path_owned = path.to_owned();
    info!("Starting scan at root = {}", path_owned);

    let handle = tokio::spawn(async move {
        let root = path_owned.clone();
        debug!("Register visitors:");

        let mut scan_stats_visitor = ScanStatsVisitor::new();
        let mut progress_visitor = ProgressVisitor::new();
        let mut top_resources_visitor = TopKResourceVisitor::new();
        let mut visitors: Vec<&mut dyn Visitable> = vec![
            &mut progress_visitor,
            &mut scan_stats_visitor,
            &mut top_resources_visitor,
        ];

        for v in &mut *visitors {
            debug!("Visitor registered: {}", v.name());
        }

        let start_time = Instant::now();
        let mut scanner = ResourceScanner::new();
        let mut registry: HashMap<String, ResourceMetadata> = HashMap::new();
        let mut writer = BufWriter::new(io::stdout());

        if Path::new(&file_path).exists() {
            load_registry(&mut registry, &file_path).expect("Failed to load registry");
            if !registry.contains_key(&root) {
                let p = Path::new(&root);
                let m = ResourceMetadata::new(&root, p.is_dir(), p.is_symlink(), 0, 0, false);
                registry.insert(root.clone(), m);
            }
            info!("Registry loaded with {} resources", registry.len());
            scanner.incremental_scan(&root, &mut registry, &mut visitors, &mut writer, &logger);
        } else {
            info!("Starting full resource scan");
            scanner.full_scan(&mut registry, &root, &mut visitors, &mut writer, &logger);
            info!("Finished full resource scan elapsed time = {:?}", start_time.elapsed());
        }

        save_registry(&mut registry, &file_path).expect("Failed to save registry");

        for visitable_instance in &mut visitors {
            info!("executing {}", visitable_instance.name());
            visitable_instance.recap(&mut writer, &logger);
            writer.flush().unwrap();
        }

        Ok("Successful scan")
    });

    info!("Waiting for handle.await");
    let result = handle.await.unwrap_or_else(|e| Err(format!("Failed to scan filesystem: {}", e)));

    // Always play sound after await, regardless of result
    play_sound();

    // Return the result after sound has been played
    result
}

fn play_sound() {
    // Log the attempt to play the sound
    info!("Attempting to play sound");
    // Spawns a blocking task using tokio's spawn_blocking
    let _ = task::spawn_blocking(|| {
        // Assuming the sound playing logic is blocking
        let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to get output stream");
        let file_path = "/home/btoal/git/turbo-tasker/src-tauri/sounds/notification_decorative-01.wav";
        let file = BufReader::new(File::open(file_path).expect("Failed to open sound file"));
        let source = Decoder::new(file).expect("Failed to decode sound file");
        stream_handle.play_raw(source.convert_samples()).expect("Failed to play sound");

        // Sleep to allow the sound to play out
        std::thread::sleep(Duration::from_millis(1000));
    });
}
