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
        Trie {
            root: TrieNode {
                is_end_of_word: false,
                children: HashMap::new(),
            },
        }
    }

    pub fn insert(&mut self, word: String) {
        let len = word.len();
        let mut node = &mut self.root;
        for (i, c) in word.chars().into_iter().enumerate() {
            node = node.children.entry(c).or_insert(TrieNode {
                is_end_of_word: i + 1 == len,
                children: HashMap::new(),
            });
            if i + 1 == len {
                node.is_end_of_word = true;
            }
        }
    }

    pub fn search(&self, word: String) -> bool {
        let len = word.len();
        let mut node = &self.root;
        for (i, c) in word.chars().enumerate() {
            if let Some(x) = node.children.get(&c) {
                node = x;
                if i + 1 == len {
                    return x.is_end_of_word;
                }
            } else {
                return false;
            }
        }
        false
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            if let Some(x) = node.children.get(&c) {
                node = x;
            } else {
                return false;
            }
        }
        true
    }
}
