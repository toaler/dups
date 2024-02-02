use std::collections::hash_map::Entry::{Occupied, Vacant};
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

    // Recursive function to enumerate and display statistics
    fn recap_recursive(&self, node: &DirectoryNode, depth: usize) {
        // Print information about the current node
        println!(
            "{:indent$}{}: {} files, {} directories, {} bytes",
            "",
            node.name,
            node.child_files,
            node.child_dirs,
            node.total_size,
            indent = depth * 2
        );

        // Recursively call the function for child nodes
        for child_node in node.children.values() {
            self.recap_recursive(child_node, depth + 1);
        }
    }
}

impl Visitable for DirectoryAnalyzerVisitor {
    fn visit(&mut self, metadata: &ResourceMetadata) {
        let path = metadata.get_path();

        let components: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        let mut current_node = &mut self.root;
        let mut d = DirectoryNode {
            name: "".to_string(),
            ..Default::default()
        };

        for i in 0..components.len() {
            let component = components[i];
            current_node = match current_node.children.entry(component.to_string()) {
                Occupied(entry) => entry.into_mut(),
                Vacant(entry) => {
                    // check if reached filename part of path
                    if i == components.len() - 1 {
                        // at filename so update stats of parent dir
                        if metadata.is_file() {
                            current_node.child_files += 1;
                        } else if metadata.is_dir() {
                            current_node.child_dirs += 1;
                        }
                        current_node.total_size += metadata.size_bytes();
                    }

                    // add new node in tree if at a non filename node OR
                    // at filename and it's a directory
                    if !(i == components.len() - 1 && metadata.is_file()) {
                        let new_node = DirectoryNode {
                            name: component.to_string(),
                            ..Default::default()
                        };
                        entry.insert(new_node)
                    } else {
                        &mut d
                    }
                }
            };
        }
    }

    fn recap(&mut self) {
        // Start the recursive enumeration from the root
        self.recap_recursive(&self.root, 0);
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