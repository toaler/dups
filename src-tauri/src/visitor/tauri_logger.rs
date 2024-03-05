use tauri::Window;

pub trait Logger {
    fn log(&self, message: String);
}
pub struct TauriLogger {
    pub(crate) window: Window,
}
impl Logger for TauriLogger {
    fn log(&self, message: String) {
        self.window.emit("log-event", &message).expect("Failed to emit log event");
    }
}