use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie { root: TrieNode::default() }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;

        for ch in word.chars() {
            node = node.children.entry(ch).or_insert(TrieNode::default());
        }

        node.is_end = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut node = &self.root;

        for ch in word.chars() {
            if let Some(next_node) = node.children.get(&ch) {
                node = next_node;
            } else {
                return false;
            }
        }

        node.is_end
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;

        for ch in prefix.chars() {
            if let Some(next_node) = node.children.get(&ch) {
                node = next_node;
            } else {
                return false;
            }
        }

        true
    }

    pub fn delete(&mut self, word: &str) {
        let root = &mut self.root;
        Trie::delete_recursive(root, word, 0);
    }

    fn delete_recursive(node: &mut TrieNode, word: &str, depth: usize) {
        if depth == word.len() {
            // Reached the end of the word
            if node.is_end {
                node.is_end = false;
            }
            return;
        }

        if let Some(next_node) = node.children.get_mut(&word.chars().nth(depth).unwrap()) {
            Trie::delete_recursive(next_node, word, depth + 1);

            // Remove child node if it has no children and is not the end of another word
            if !next_node.is_end && next_node.children.is_empty() {
                node.children.remove(&word.chars().nth(depth).unwrap());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut trie = Trie::new();

        trie.insert("hello");
        trie.insert("world");

        assert!(trie.search("hello"));
        assert!(trie.search("world"));
        assert!(!trie.search("worlds"));
        assert!(!trie.search("foo"));
    }

    #[test]
    fn test_starts_with() {
        let mut trie = Trie::new();

        trie.insert("hello");
        trie.insert("world");

        assert!(trie.starts_with("hell"));
        assert!(trie.starts_with("worl"));
        assert!(!trie.starts_with("foo"));
    }

    #[test]
    fn test_delete() {
        let mut trie = Trie::new();

        trie.insert("hello");
        trie.insert("world");

        assert!(trie.search("hello"));
        trie.delete("hello");
        assert!(!trie.search("hello"));

        assert!(trie.search("world"));
        trie.delete("world");
        assert!(!trie.search("world"));
    }
}
