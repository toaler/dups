use std::io::{self};

pub trait FileTypeDetector {
    fn get_file_type(&self, path: &str) -> io::Result<String>;
}