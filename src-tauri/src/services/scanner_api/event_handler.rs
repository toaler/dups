pub trait EventHandler {
    fn publish(&self, event: &str, message: String);
}