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
pub async fn commit(w: Window, path: &str) -> Result<String, String> {

    let logger = TauriEventHandler {window: w};

    let path_owned = path.to_owned(); // Clone path into a new String
    info!("path = {}", path_owned);

    // Now use path_owned inside your async block
    tauri::async_runtime::spawn(async move {
        // Use path_owned instead of path
        info!("{}", path_owned);

        // Since path_owned is owned by the block, there's no issue with lifetimes
        Ok(format!("Hello, {}! You've been greeted from Rust asynchronously!", path_owned))
    }).await.unwrap_or_else(|e| Err(format!("Failed to scan filesystem: {}", e)))
}