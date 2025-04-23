use std::collections::HashMap;

struct TrieNode {
    is_end_of_word: bool,
    children: HashMap<char, TrieNode>,
}

pub struct WordDictionary {
    root: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    pub fn new() -> Self {
        // TODO: Implement constructor
        WordDictionary {
            root: TrieNode {
                is_end_of_word: false,
                children: HashMap::new(),
            }
        }
    }
    
    pub fn add_word(&mut self, word: String) {
        // TODO: Implement add_word
    }
    
    pub fn search(&self, word: String) -> bool {
        // TODO: Implement search
        false
    }
}
