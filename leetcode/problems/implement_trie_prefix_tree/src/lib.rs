use std::collections::HashMap;

struct TrieNode {
    is_end_of_word: bool,
    children: HashMap<char, TrieNode>,
}

pub struct Trie {
    root: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    pub fn new() -> Self {
        // TODO: Implement constructor
        Trie {
            root: TrieNode {
                is_end_of_word: false,
                children: HashMap::new(),
            }
        }
    }
    
    pub fn insert(&mut self, word: String) {
        // TODO: Implement insert
    }
    
    pub fn search(&self, word: String) -> bool {
        // TODO: Implement search
        false
    }
    
    pub fn starts_with(&self, prefix: String) -> bool {
        // TODO: Implement starts_with
        false
    }
}
