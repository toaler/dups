pub trait CompressionChecker {
    fn is_compressible(&self, mimetype: &String) -> i32;
}