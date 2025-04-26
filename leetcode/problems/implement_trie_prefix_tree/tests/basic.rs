use implement_trie_prefix_tree::Trie;

#[test]
fn test_example_1() {
    // ["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
    // [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
    // Output: [null, null, true, false, true, null, true]

    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    assert!(trie.search("apple".to_string()));
    assert!(!trie.search("app".to_string()));
    assert!(trie.starts_with("app".to_string()));
    trie.insert("app".to_string());
    assert!(trie.search("app".to_string()));
}

#[test]
fn test_prefix_not_word() {
    let mut trie = Trie::new();
    trie.insert("hello".to_string());
    assert!(!trie.search("he".to_string()));
    assert!(trie.starts_with("he".to_string()));
}

#[test]
fn test_multiple_words_with_common_prefix() {
    let mut trie = Trie::new();
    trie.insert("car".to_string());
    trie.insert("card".to_string());
    trie.insert("cart".to_string());

    assert!(trie.search("car".to_string()));
    assert!(trie.search("card".to_string()));
    assert!(trie.search("cart".to_string()));
    assert!(!trie.search("ca".to_string()));
    assert!(trie.starts_with("ca".to_string()));
    assert!(trie.starts_with("car".to_string()));
    assert!(trie.starts_with("card".to_string()));
}

#[test]
fn test_non_existent_word_and_prefix() {
    let mut trie = Trie::new();
    trie.insert("hello".to_string());

    assert!(!trie.search("world".to_string()));
    assert!(!trie.starts_with("wo".to_string()));
}
