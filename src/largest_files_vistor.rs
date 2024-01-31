use std::collections::BinaryHeap;
use std::cmp::Reverse;
use crate::resource_metadata::ResourceMetadata;
use crate::visitable::Visitable;

pub(crate) struct Top50LargestResources {
    top_resources: BinaryHeap<Reverse<ResourceMetadata>>,
}

impl Visitable for Top50LargestResources {
    fn visit(&mut self, metadata: &ResourceMetadata) {
        if !metadata.is_dir() {
            if self.top_resources.len() < 50 {
                // If the heap is not full, just push the new metadata
                self.top_resources.push(Reverse(metadata.clone()));
            } else if metadata.size_bytes() > self.top_resources.peek().unwrap().0.size_bytes() {
                // If the new metadata is larger than the smallest in the heap, replace the smallest
                self.top_resources.pop();
                self.top_resources.push(Reverse(metadata.clone()));
            }
        }
    }

    fn recap(&mut self) {
        let reversed_sorted_resources: Vec<_> = self.top_resources.clone().into_sorted_vec().into_iter().collect();

        println!("Top 50 Largest Resources:");
        for (i, metadata) in reversed_sorted_resources.iter().enumerate() {
            let metadata = &metadata.0;
            let padded_ranking = format!("{:<5}", i + 1); // Padded to 5 characters for ranking
            let padded_bytes = format!("{:>16}", metadata.size_bytes()); // Padded to 50 characters for bytes
            println!("Rank: {}, Bytes: {}, Path: {}", padded_ranking, padded_bytes, metadata.get_path());
        }
    }

    fn name(&self) -> &'static str {
        "Top50LargestResources"
    }
}

impl Top50LargestResources {
    pub(crate) fn new() -> Self {
        Top50LargestResources {
            top_resources: BinaryHeap::with_capacity(50),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_50_largest_files() {
        let mut visitor = Top50LargestResources::new();

        // Add resources in ascending order of size_bytes
        for size_bytes in (1..=100).step_by(2) {
            let path = format!("/path/to/resource{}", size_bytes);
            let is_dir = false;
            let is_symlink = false;
            let modified = 123456789;
            let metadata = ResourceMetadata::new(&path, is_dir, is_symlink, modified, size_bytes);
            visitor.visit(&metadata);
        }

    }
}

