use crate::services::file_api::compression_checker::CompressionChecker;

pub(crate) struct MimeCompressionChecker;

impl CompressionChecker for MimeCompressionChecker {
    // Change the method signature to take a String instead of &str
    fn is_compressible(&self, mimetype: &String) -> i32 {
        let compressible_types = vec![
            "text/plain", "text/html", "text/css", "text/javascript", "application/json",
            "application/xml", "text/csv", "image/bmp", "image/tiff", "audio/wav",
            "audio/aiff", "application/rtf",
            // Add more as needed
        ];

        let uncompressible_types = vec![
            "image/jpeg", "image/png", "image/gif", "image/webp", "video/mp4",
            "video/x-matroska", "video/webm", "audio/mpeg", "audio/mp4", "audio/ogg",
            "audio/flac", "application/zip", "application/gzip", "application/x-rar-compressed",
            // Add more as needed
        ];

        // Since mimetype is now a String, we can borrow it for comparison
        if compressible_types.contains(&mimetype.as_str()) {
            1
        } else if uncompressible_types.contains(&mimetype.as_str()) {
            -1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::services::file_api::compression_checker::CompressionChecker;
    use crate::services::file_impl::mime_compression_checker::MimeCompressionChecker; // Ensure this path is correct

    #[test]
    fn test_compressible() {
        let checker = MimeCompressionChecker;
        // Convert test strings to String type
        assert_eq!(checker.is_compressible(&"text/plain".to_string()), 1);
        assert_eq!(checker.is_compressible(&"application/xml".to_string()), 1);
    }

    #[test]
    fn test_uncompressible() {
        let checker = MimeCompressionChecker;
        // Convert test strings to String type
        assert_eq!(checker.is_compressible(&"image/jpeg".to_string()), -1);
        assert_eq!(checker.is_compressible(&"application/zip".to_string()), -1);
    }

    #[test]
    fn test_unknown_type() {
        let checker = MimeCompressionChecker;
        // Convert test strings to String type
        assert_eq!(checker.is_compressible(&"application/unknown".to_string()), 0);
    }
}
