use crate::visitor::tauri_logger::EventHandler;

pub struct NoopLogger {
}
impl EventHandler for NoopLogger {
    fn publish(&self, _event: &str, _message: String) {
    }
}