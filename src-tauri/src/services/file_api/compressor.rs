use std::io::Result as IoResult;

pub trait Compressor {
    /// Compresses a list of file paths and returns a list of results.
    ///
    /// # Arguments
    /// * `files` - A slice of file paths to compress.
    fn compress(&self, files: &[&str]) -> Vec<IoResult<String>>;
}
