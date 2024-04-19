use std::collections::HashMap;
use std::{env, io};
use std::path::Path;
use std::time::Instant;
use log::{debug, info};
use serde::Deserialize;
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

#[derive(Deserialize, Debug)]
pub struct Action {
    action: String,
    path: String,
    bytes: usize,
}

#[command]
pub async fn commit(w: Window, actions: Vec<Action>) -> Result<String, String> {
    let logger = TauriEventHandler { window: w };



    // Now use path_owned inside your async block
    tauri::async_runtime::spawn(async move {

        // Enumerate and log each action
        for action in actions {
            info!("Action: {}, Path: {}, Bytes: {}", action.action, action.path, action.bytes);
        }

        Ok(format!("Hello from commit! You've been greeted from Rust asynchronously!"))
    }).await.unwrap_or_else(|e| Err(format!("Failed to scan filesystem: {}", e)))
}