use crate::visitor::tauri_logger::Logger;

pub struct NoopLogger {
}
impl Logger for NoopLogger {
    fn log(&self, _message: String) {
    }
}