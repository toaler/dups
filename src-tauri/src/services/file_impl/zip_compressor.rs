use std::fs::File;
use std::io;
use log::info;
use crate::services::file_api::compressor::Compressor;
use std::io::{Read, Write};
use zip::write::FileOptions;
use zip::ZipWriter;

struct ZipCompressor;

impl Compressor for ZipCompressor {
    fn compress(&self, files: &[&str]) -> Vec<io::Result<String>> {
        files.iter().map(|&file_path| {
            info!("Compressing file: {}", file_path);
            let compressed_file_name = format!("{}.zip", file_path);
            let file = File::create(&compressed_file_name);
            match file {
                Ok(file) => {
                    let mut zip = ZipWriter::new(file);
                    let options = FileOptions::default()
                        .compression_method(zip::CompressionMethod::Stored); // Using stored for simplicity; change as needed
                    let mut buffer = Vec::new();
                    {
                        // Scope to ensure file is closed and all writes are flushed before we read
                        let mut f = File::open(file_path)?;
                        f.read_to_end(&mut buffer)?;
                    }
                    zip.start_file(file_path, options)?;
                    zip.write_all(&buffer)?;
                    zip.finish()?;
                    Ok(compressed_file_name)
                }
                Err(e) => Err(e),
            }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::{NamedTempFile, tempdir};

    #[test]
    fn test_compress_single_file() {
        let compressor = ZipCompressor;
        let dir = tempdir().unwrap(); // Create a temporary directory

        // Create a temporary file within the temporary directory
        let mut temp_file = NamedTempFile::new_in(dir.path()).unwrap();
        writeln!(temp_file.as_file_mut(), "Test content").unwrap();
        let temp_path = temp_file.path().to_str().unwrap();

        // Compress the file
        let results = compressor.compress(&[temp_path]);
        assert_eq!(results.len(), 1);
        assert!(results[0].is_ok());

        // Verify the compressed file exists
        let compressed_file_path = results[0].as_ref().unwrap();
        assert!(fs::metadata(compressed_file_path).is_ok());

        // Cleanup: The tempdir's Drop trait automatically deletes the directory and its contents
        // No need to explicitly delete the compressed file
    }
}