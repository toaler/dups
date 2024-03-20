use std::collections::HashMap;
use std::{env, io};
use std::path::Path;
use std::time::Instant;
use log::{debug, info};
use tauri::{command, Window};
use crate::ui::handler::tauri_event_handler::TauriEventHandler;
use crate::{load_registry, save_registry};
use crate::services::scanner_impl::resource_scanner::ResourceScanner;
use crate::state::resource_metadata::ResourceMetadata;
use crate::util::util::add_groupings_usize;
use crate::services::scanner_impl::visitor::progress_visitor::ProgressVisitor;
use crate::services::scanner_impl::visitor::scan_stats_visitor::ScanStatsVisitor;
use crate::services::scanner_impl::visitor::top_k_resource_visitor::TopKResourceVisitor;
use crate::services::scanner_api::visitable::Visitable;

#[command]
pub async fn scan_filesystem(w: Window, path: &str) -> Result<String, String> {

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