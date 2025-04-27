use design_add_and_search_words_data_structure::WordDictionary;

#[test]
fn test_example() {
    // ["WordDictionary","addWord","addWord","addWord","search","search","search","search"]
    // [[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]
    // Output: [null,null,null,null,false,true,true,true]

    let mut word_dictionary = WordDictionary::new();
    word_dictionary.add_word("bad".to_string());
    word_dictionary.add_word("dad".to_string());
    word_dictionary.add_word("mad".to_string());
    assert!(!word_dictionary.search("pad".to_string()));
    assert!(word_dictionary.search("bad".to_string()));
    assert!(word_dictionary.search(".ad".to_string()));
    assert!(word_dictionary.search("b..".to_string()));
}

#[test]
fn test_all_wildcards() {
    let mut word_dictionary = WordDictionary::new();
    word_dictionary.add_word("hello".to_string());
    assert!(word_dictionary.search(".....".to_string()));
    assert!(!word_dictionary.search("......".to_string()));
}

#[test]
fn test_multiple_words_with_wildcards() {
    let mut word_dictionary = WordDictionary::new();
    word_dictionary.add_word("car".to_string());
    word_dictionary.add_word("card".to_string());
    word_dictionary.add_word("cart".to_string());

    assert!(word_dictionary.search("car".to_string()));
    assert!(word_dictionary.search("car.".to_string()));
    assert!(!word_dictionary.search("car..".to_string()));
    assert!(word_dictionary.search("c..t".to_string()));
    assert!(word_dictionary.search("...t".to_string()));
    assert!(!word_dictionary.search("....t".to_string()));
}

#[test]
fn test_wildcard_at_different_positions() {
    let mut word_dictionary = WordDictionary::new();
    word_dictionary.add_word("apple".to_string());

    assert!(word_dictionary.search("apple".to_string()));
    assert!(word_dictionary.search(".pple".to_string()));
    assert!(word_dictionary.search("a.ple".to_string()));
    assert!(word_dictionary.search("ap.le".to_string()));
    assert!(word_dictionary.search("app.e".to_string()));
    assert!(word_dictionary.search("appl.".to_string()));
    assert!(word_dictionary.search("a..le".to_string()));
    assert!(word_dictionary.search("..p..".to_string()));
    assert!(!word_dictionary.search("banana".to_string()));
}

#[test]
fn test_example_2() {
    let mut word_dictionary = WordDictionary::new();
    word_dictionary.add_word("at".to_string());
    word_dictionary.add_word("and".to_string());
    word_dictionary.add_word("an".to_string());
    word_dictionary.add_word("and".to_string());
    assert!(!word_dictionary.search("a".to_string()));
    assert!(!word_dictionary.search(".at".to_string()));
    word_dictionary.add_word("bat".to_string());
    assert!(word_dictionary.search(".at".to_string()));
    assert!(word_dictionary.search("an.".to_string()));
    assert!(!word_dictionary.search("a.d.".to_string()));
    assert!(!word_dictionary.search("b.".to_string()));
    assert!(word_dictionary.search("a.d".to_string()));
    assert!(!word_dictionary.search(".".to_string()));
}
