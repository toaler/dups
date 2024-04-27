use log::{error, info};
use serde::Deserialize;
use tauri::{command, Window};
use crate::services::file_api::file_management::{DeletionStatus, FileManagement};
use crate::services::file_impl::file_management_impl::FileManagementImpl;
use crate::services::scanner_api::event_handler::EventHandler;
use crate::ui::handler::tauri_event_handler::TauriEventHandler;
use crate::util::util::play_sound;

#[derive(Deserialize, Debug)]
pub struct Action {
    action: String,
    path: String,
    bytes: usize,
}

#[command]
pub async fn commit(w: Window, actions: Vec<Action>) -> Result<String, String> {
    let event_handler = TauriEventHandler {window: w};

    let result = tauri::async_runtime::spawn(async move {
        // Enumerate and log each action
        for action in actions {
            info!("Processing Action: {}, Path: {}, Bytes: {}", action.action, action.path, action.bytes);

            match action.action.as_str() {
                "delete" => {
                    // Here, implement what should happen when the action is "upload"
                    info!("Deleting from path: {}", action.path);

                    let deleter = FileManagementImpl;
                    match deleter.delete_file(&action.path) {
                        DeletionStatus::Success => {
                            info!("Deleted {}", action.path);
                            event_handler.publish("commit-event", format!("{{\"status\" : \"success\", \"path\": {:?}}}", action.path))
                        },
                        DeletionStatus::Failure(msg) => {
                            error!("Deletion failed with error: {}", msg);
                            event_handler.publish("commit-event", format!("'status' : 'failed', 'path', {:?}", action.path))
                        },
                    }
                },
                "compressing" => {
                    // Implement the download action
                    info!("Compressing file at path: {}", action.path);
                    // A function to handle download could be called here
                },
                _ => {
                    info!("Unknown action: {}", action.action);
                    // Handle unknown or unsupported actions
                }
            }
        }

        Ok("Hello from commit! You've been greeted from Rust asynchronously!".to_string())
    }).await.unwrap_or_else(|e| Err(format!("Failed to scan filesystem: {}", e)));

    play_sound("/home/btoal/git/turbo-tasker/src-tauri/sounds/hero_decorative-celebration-02.wav", 1750);
    result
}