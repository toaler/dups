use log::{info};
use serde::Deserialize;
use tauri::{command, Window};

#[derive(Deserialize, Debug)]
pub struct Action {
    action: String,
    path: String,
    bytes: usize,
}

#[command]
pub async fn commit(_w: Window, actions: Vec<Action>) -> Result<String, String> {
    // Now use path_owned inside your async block
    tauri::async_runtime::spawn(async move {

        // Enumerate and log each action
        for action in actions {
            info!("Action: {}, Path: {}, Bytes: {}", action.action, action.path, action.bytes);
        }

        Ok(format!("Hello from commit! You've been greeted from Rust asynchronously!"))
    }).await.unwrap_or_else(|e| Err(format!("Failed to scan filesystem: {}", e)))
}