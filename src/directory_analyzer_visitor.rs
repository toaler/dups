use std::collections::HashMap;
use crate::resource_metadata::ResourceMetadata;
use crate::visitable::Visitable;

#[derive(Debug, Default)]
struct DirectoryNode {
    name: String,
    child_files: usize,
    child_dirs: usize,
    total_size: u64,
    children: HashMap<String, DirectoryNode>,
}

pub struct DirectoryAnalyzerVisitor {
    root: DirectoryNode,
}

impl DirectoryAnalyzerVisitor {
    pub fn new() -> Self {
        DirectoryAnalyzerVisitor {
            root: DirectoryNode::default(),
        }
    }
}

impl Visitable for DirectoryAnalyzerVisitor {
    fn visit(&mut self, metadata: &ResourceMetadata) {
        let path = metadata.get_path();

        let components: Vec<&str> = path.trim_start_matches('/').split('/').collect();
        let mut current_node = &mut self.root;

        // Create a separate variable for path components
        let last_component = *components.last().unwrap();

        for component in components.iter() {
            if component != &last_component {
                current_node = current_node
                    .children
                    .entry(component.to_string())
                    .or_insert_with(|| DirectoryNode {
                        name: component.to_string(),
                        ..Default::default()
                    });
            } else {
                if metadata.is_file() {
                    current_node.child_files += 1;
                } else if metadata.is_dir() {
                    current_node.child_dirs += 1;
                }

                current_node.total_size += metadata.size_bytes();

                if metadata.is_dir() {
                    current_node = current_node
                        .children
                        .entry(component.to_string())
                        .or_insert_with(|| DirectoryNode {
                            name: component.to_string(),
                            ..Default::default()
                        });
                }
            }
        }
    }

    fn recap(&mut self) {
        // Implement recap logic if needed
        // This could print or return aggregated statistics
    }

    fn name(&self) -> &'static str {
        "DirectoryAnalyzerVisitor"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directory_analyzer_visitor() {
        let metadata1 = ResourceMetadata::new(&"/a".to_string(), true, false, 0, 96);
        let metadata2 = ResourceMetadata::new(&"/a/foo.txt".to_string(), false, false, 0, 100);
        let metadata3 = ResourceMetadata::new(&"/a/bar.txt".to_string(), false, false, 0, 150);
        let metadata4 = ResourceMetadata::new(&"/a/b".to_string(), true, false, 0, 96);
        let metadata5 = ResourceMetadata::new(&"/a/b/bif.txt".to_string(), false, false, 0, 75);

        let mut visitor = DirectoryAnalyzerVisitor::new();

        visitor.visit(&metadata1);
        visitor.visit(&metadata2);
        visitor.visit(&metadata3);
        visitor.visit(&metadata4);
        visitor.visit(&metadata5);

        // Check the root node
        assert_eq!(visitor.root.child_files, 0);
        assert_eq!(visitor.root.child_dirs, 1);
        assert_eq!(visitor.root.total_size, 96);
        assert_eq!(visitor.root.name, "");

        // Check the "/a" node
        if let Some(a_node) = visitor.root.children.get("a") {
            assert_eq!(a_node.child_files, 2);
            assert_eq!(a_node.child_dirs, 1);
            assert_eq!(a_node.total_size, 346);
            assert_eq!(a_node.name, "a");
        } else {
            panic!("Missing node for '/a'");
        }

        // Check the "/a/b" node
        if let Some(b_node) = visitor.root.children.get("a").and_then(|a_node| a_node.children.get("b")) {
            assert_eq!(b_node.child_files, 1);
            assert_eq!(b_node.child_dirs, 0);
            assert_eq!(b_node.total_size, 75);
            assert_eq!(b_node.name, "b");
        } else {
            panic!("Missing node for '/a/b'");
        }
    }
}