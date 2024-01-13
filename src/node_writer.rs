use std::path::Path;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use crate::visitable::Visitable;

pub struct NodeWriter {}

impl Visitable for NodeWriter {
    fn visit(&mut self, path: &Path, _is_dir: bool) {
        let modified_time =     match path.metadata() {
            Ok(metadata) => {
                if let Ok(modified_time) = metadata.modified() {
                    modified_time
                } else {
                    SystemTime::now() // Default to current time if modified time retrieval fails
                }
            }
            Err(_) => SystemTime::now(), // Handle metadata retrieval error
        };

        let local_time: DateTime<Local> = modified_time.into();

        // Format the DateTime as a string
        let formatted_time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();

        println!("Visiting : {} {}", path.display(), formatted_time);
    }

    fn recap(&mut self) {
    }
}
