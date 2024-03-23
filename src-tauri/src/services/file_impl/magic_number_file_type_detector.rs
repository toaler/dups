use std::fs::File;
use std::io::{self, Read};
use crate::services::file_api::file_type_detector::FileTypeDetector;

/// Implementation of FileTypeDetector.
pub struct MagicNumberFileTypeDetector;

impl FileTypeDetector for MagicNumberFileTypeDetector {
    fn get_file_type(&self, path: &str) -> io::Result<String> {
        let file = File::open(path)?;
        let mut buffer = Vec::new();
        // Read the first few bytes into a vector; adjust the number based on your needs
        file.take(10).read_to_end(&mut buffer)?;

        let file_type = match buffer.as_slice() {
            [0x89, b'P', b'N', b'G', ..] => "PNG image",
            [0xFF, 0xD8, 0xFF, ..] => "JPEG image",
            [b'G', b'I', b'F', b'8', ..] => "GIF image",
            _ => "unknown",
        };

        Ok(file_type.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
use std::path::Path;
use super::*;

    /// Helper function to create a test file with specified bytes.
    fn create_test_file(path: &str, bytes: &[u8]) -> std::io::Result<()> {
        use std::io::Write;
        let mut file = File::create(path)?;
        file.write_all(bytes)?;
        Ok(())
    }

    /// Setup function to create example files with known magic numbers.
    fn setup() {
        std::fs::create_dir_all("tests/files").expect("Failed to create test directory");
        // PNG: The first bytes of a PNG file
        create_test_file("tests/files/example.png", &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]).unwrap();
        // JPEG: The first bytes of a JPEG file
        create_test_file("tests/files/example.jpg", &[0xFF, 0xD8, 0xFF]).unwrap();
        // GIF: The first bytes of a GIF file
        create_test_file("tests/files/example.gif", &[b'G', b'I', b'F', b'8', b'9', b'a']).unwrap();

    }

    /// Teardown function to clean up test files.
    fn teardown() {
        let path = Path::new("tests/files");
        if path.exists() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let _ = fs::remove_file(entry.path()); // Ignore errors here
                    }
                }
            }
        }
        let _ = fs::remove_dir_all(path); // Ignore the result
    }

    #[test]
    fn detects_png_file() {
        setup();
        let detector = MagicNumberFileTypeDetector;
        let file_type = detector.get_file_type("tests/files/example.png").unwrap();
        assert_eq!(file_type, "PNG image");
        teardown();
    }

    #[test]
    fn detects_jpeg_file() {
        setup();
        let detector = MagicNumberFileTypeDetector;
        let file_type = detector.get_file_type("tests/files/example.jpg").unwrap();
        assert_eq!(file_type, "JPEG image");
        teardown();
    }

    #[test]
    fn detects_gif_file() {
        setup();
        let detector = MagicNumberFileTypeDetector;
        let file_type = detector.get_file_type("tests/files/example.gif").unwrap();
        assert_eq!(file_type, "GIF image");
        teardown();
    }

    #[test]
    fn detects_unknown_file() {
        setup();
        let path = "tests/files/unknown.file";
        // Ensure the file is created. Panic with a message if not.
        create_test_file(path, &[]).expect("Failed to create unknown file for testing.");

        let detector = MagicNumberFileTypeDetector;
        // Handle the error gracefully instead of unwrapping.
        match detector.get_file_type(path) {
            Ok(file_type) => assert_eq!(file_type, "unknown", "File type should be identified as 'unknown'."),
            Err(e) => panic!("Failed to get file type: {:?}", e),
        }

        teardown();
    }
}