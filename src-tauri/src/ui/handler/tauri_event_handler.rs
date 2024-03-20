use tauri::Window;
use crate::services::scanner_api::event_handler::EventHandler;

pub struct TauriEventHandler {
    pub(crate) window: Window,
}
impl EventHandler for TauriEventHandler {
    fn publish(&self, event: &str,  message: String) {

        self.window.emit(event, &message).expect("Failed to emit log event");
    }
}