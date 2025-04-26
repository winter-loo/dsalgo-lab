use std::collections::HashMap;

struct TrieNode {
    is_end_of_word: bool,
    children: HashMap<char, TrieNode>,
}

pub struct WordDictionary {
    root: TrieNode,
}

impl WordDictionary {
    pub fn new() -> Self {
        WordDictionary {
            root: TrieNode {
                is_end_of_word: false,
                children: HashMap::new(),
            },
        }
    }

    pub fn add_word(&mut self, word: String) {
        let len = word.len();
        let mut node = &mut self.root;
        for (i, c) in word.chars().enumerate() {
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
        fn dfs(node: &TrieNode, word: &[char], index: usize) -> bool {
            if index == word.len() {
                return node.is_end_of_word;
            }

            if word[index] == '.' {
                // For wildcard, try all possible children
                for child in node.children.values() {
                    if dfs(child, word, index + 1) {
                        return true;
                    }
                }
            } else {
                // For specific character, check if it exists in children
                if let Some(child) = node.children.get(&word[index]) {
                    return dfs(child, word, index + 1);
                }
            }
            false
        }

        dfs(&self.root, &word.chars().collect::<Vec<_>>(), 0)
    }
}
