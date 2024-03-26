use crate::services::file_api::file_type_detector::FileTypeDetector;

pub struct MimeGuessFileTypeDetector;

impl FileTypeDetector for MimeGuessFileTypeDetector {
    fn get_file_type(&self, path: &str) -> std::io::Result<String> {
        let mime_type = mime_guess::from_path(path)
            .first_or_octet_stream().to_string(); // Gets the first MIME type guessed, or "application/octet-stream" if none found.

        Ok(mime_type)
    }
}


#[cfg(test)]
mod tests {
    use crate::services::file_api::file_type_detector::FileTypeDetector;
    use crate::services::file_impl::mime_guess_file_type_detector::MimeGuessFileTypeDetector;

    #[test]
    fn test_known_extension() {
        let file_path = "test.txt";
        let mime_type = MimeGuessFileTypeDetector.get_file_type(file_path);
        assert_eq!(mime_type.unwrap(), "text/plain");
    }

    #[test]
    fn test_unknown_extension() {
        // Setup: create a temporary file with an unknown extension
        let file_path = "test.unknownext";
        let mime_type = MimeGuessFileTypeDetector.get_file_type(file_path);
        assert_eq!(mime_type.unwrap(), "application/octet-stream");
    }

    // Additional tests can be designed for different file types and edge cases
}