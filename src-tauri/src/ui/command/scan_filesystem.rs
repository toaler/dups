use std::collections::HashMap;
use std::{env};
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::time::{Instant};
use log::{debug, info};
use tauri::command;
use crate::ui::handler::tauri_event_handler::TauriEventHandler;
use crate::state::resource_metadata::ResourceMetadata;
use crate::services::scanner_impl::{
    resource_scanner::ResourceScanner,
    visitor::{progress_visitor::ProgressVisitor, scan_stats_visitor::ScanStatsVisitor, top_k_resource_visitor::TopKResourceVisitor},
};
use crate::services::scanner_api::visitable::Visitable;
use crate::{load_registry, save_registry};
use crate::util::util::play_sound;

#[command]
pub async fn scan_filesystem(w: tauri::Window, uid: &str, path: &str) -> Result<&'static str, String> {
    info!("[{}] scan_filesystem start scanning root = {}", uid, path);
    let temp_dir = env::temp_dir();
    let file_path = temp_dir.join("output.csv");
    let logger = TauriEventHandler { window: w };
    let path_owned = path.to_owned();

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

    let result = handle.await.unwrap_or_else(|e| Err(format!("Failed to scan filesystem: {}", e)));
    play_sound("/home/btoal/git/turbo-tasker/src-tauri/sounds/notification_decorative-01.wav", 1000);
    info!("[{}] scan_filesystem end", uid);
    result
}
