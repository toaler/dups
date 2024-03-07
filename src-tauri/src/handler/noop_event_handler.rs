use crate::handler::event_handler::EventHandler;

pub struct NoopEventHandler {
}

impl EventHandler for NoopEventHandler {
    fn publish(&self, _event: &str, _message: String) {
    }
}