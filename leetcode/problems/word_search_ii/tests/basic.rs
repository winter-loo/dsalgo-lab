use std::collections::HashSet;
use word_search_ii::Solution;

// Helper function to compare results regardless of order
fn compare_results(result: Vec<String>, expected: Vec<String>) -> bool {
    assert!(result.len() == expected.len());
    let result_set: HashSet<String> = result.into_iter().collect();
    let expected_set: HashSet<String> = expected.into_iter().collect();

    result_set == expected_set
}

#[test]
fn test_example_1() {
    // Input: board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]],
    // words = ["oath","pea","eat","rain"]
    // Output: ["eat","oath"]
    let board = vec![
        vec!['o', 'a', 'a', 'n'],
        vec!['e', 't', 'a', 'e'],
        vec!['i', 'h', 'k', 'r'],
        vec!['i', 'f', 'l', 'v'],
    ];
    let words = vec![
        "oath".to_string(),
        "pea".to_string(),
        "eat".to_string(),
        "rain".to_string(),
    ];

    let result = Solution::find_words(board, words);
    println!("result={result:?}");
    let expected = vec!["eat".to_string(), "oath".to_string()];

    assert!(compare_results(result, expected));
}

#[test]
fn test_example_2() {
    // Input: board = [["a","b"],["c","d"]], words = ["abcb"]
    // Output: []
    let board = vec![vec!['a', 'b'], vec!['c', 'd']];
    let words = vec!["abcb".to_string()];

    let result = Solution::find_words(board, words);

    assert!(result.is_empty());
}

#[test]
fn test_single_letter_board() {
    // Input: board = [["a"]], words = ["a", "b"]
    // Output: ["a"]
    let board = vec![vec!['a']];
    let words = vec!["a".to_string(), "b".to_string()];

    let result = Solution::find_words(board, words);
    let expected = vec!["a".to_string()];

    assert!(compare_results(result, expected));
}

#[test]
fn test_duplicate_letters() {
    // Input: board = [["a","a","a"],["a","a","a"],["a","a","a"]],
    // words = ["a", "aa", "aaa", "aaaa"]
    // Output: ["a", "aa", "aaa", "aaaa"]
    let board = vec![
        vec!['a', 'a', 'a'],
        vec!['a', 'a', 'a'],
        vec!['a', 'a', 'a'],
    ];
    let words = vec![
        "a".to_string(),
        "aa".to_string(),
        "aaa".to_string(),
        "aaaa".to_string(),
    ];

    let result = Solution::find_words(board, words);
    let expected = vec![
        "a".to_string(),
        "aa".to_string(),
        "aaa".to_string(),
        "aaaa".to_string(),
    ];

    assert!(compare_results(result, expected));
}

#[test]
fn test_example_3() {
    let board = vec![
        vec!['a', 'b', 'c'],
        vec!['a', 'e', 'd'],
        vec!['a', 'f', 'g'],
    ];
    let words = vec![
        "abcdefg".to_string(),
        "gfedcbaaa".to_string(),
        "eaabcdgfa".to_string(),
        "befa".to_string(),
        "dgc".to_string(),
        "ade".to_string(),
    ];

    let result = Solution::find_words(board, words);
    println!("result={result:?}");
    let expected = vec![
        "abcdefg".to_string(),
        "befa".to_string(),
        "eaabcdgfa".to_string(),
        "gfedcbaaa".to_string(),
    ];

    assert!(compare_results(result, expected));
}
