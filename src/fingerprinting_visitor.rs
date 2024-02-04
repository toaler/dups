use crate::resource_metadata::ResourceMetadata;

// pub struct HashingVisitor {
//     hash: u64,
//     name: &'static str,
// }
//
// impl HashingVisitor {
//     pub fn new(name: &'static str) -> Self {
//         HashingVisitor { hash: 0, name }
//     }
//
//     pub fn get_hash(&self) -> u64 {
//         self.hash
//     }
// }
//
// impl super::Visitable for HashingVisitor {
//     fn visit(&mut self, metadata: &ResourceMetadata) {
//         // Assuming metadata includes the file path
//         let file_path = metadata.get_path();
//         let file_content = std::fs::read(file_path).expect("Failed to read file content");
//
//         // Calculate xxHash of the file content
//         self.hash = xxh3_64(&file_content);
//     }
//
//     fn recap(&mut self) {
//         // Optionally, you can print or use the hash in any way you need
//         println!("Hash for {}: {}", self.name, self.hash);
//     }
//
//     fn name(&self) -> &'static str {
//         self.name
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::collections::HashMap;
//
//     // ... (existing tests)
//
//     #[test]
//     fn test_hashing_visitor() {
//         let path = String::from("/tmp/a.out");
//         let is_dir = false;
//         let is_symlink = false;
//         let modified = 123456789;
//
//         let metadata = ResourceMetadata::new(&path, is_dir, is_symlink, modified, 0);
//
//         let mut hashing_visitor = HashingVisitor::new("HashingVisitor");
//         hashing_visitor.visit(&metadata);
//
//         let hash = hashing_visitor.get_hash();
//         assert_ne!(hash, 0);
//
//         // Optionally, you can add more assertions based on your specific needs
//     }
// }
//
// // Add more tests for your existing functionality...
