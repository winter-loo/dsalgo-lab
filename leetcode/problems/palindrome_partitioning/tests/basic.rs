use palindrome_partitioning::Solution;
use std::collections::HashSet;

// Helper function to compare results regardless of order
fn compare_results(result: Vec<Vec<String>>, expected: Vec<Vec<String>>) -> bool {
    assert!(result.len() == expected.len());
    let result_set: HashSet<Vec<String>> = result.into_iter().collect();
    let expected_set: HashSet<Vec<String>> = expected.into_iter().collect();

    result_set == expected_set
}

#[test]
fn test_example_1() {
    // Input: s = "aab"
    // Output: [["a","a","b"],["aa","b"]]
    let s = "aab".to_string();
    let result = Solution::partition(s);

    let expected = vec![
        vec!["a".to_string(), "a".to_string(), "b".to_string()],
        vec!["aa".to_string(), "b".to_string()],
    ];

    assert!(compare_results(result, expected));
}

#[test]
fn test_example_2() {
    // Input: s = "a"
    // Output: [["a"]]
    let s = "a".to_string();
    let result = Solution::partition(s);

    let expected = vec![vec!["a".to_string()]];

    assert!(compare_results(result, expected));
}

#[test]
fn test_all_palindromes() {
    // Input: s = "aba"
    // Output: [["a","b","a"],["aba"]]
    let s = "aba".to_string();
    let result = Solution::partition(s);

    let expected = vec![
        vec!["a".to_string(), "b".to_string(), "a".to_string()],
        vec!["aba".to_string()],
    ];

    assert!(compare_results(result, expected));
}

#[test]
fn test_multiple_palindromes() {
    // Input: s = "abba"
    // Output: [["a","b","b","a"],["a","bb","a"],["abba"]]
    let s = "abba".to_string();
    let result = Solution::partition(s);
    println!("{result:?}");

    let expected = vec![
        vec![
            "a".to_string(),
            "b".to_string(),
            "b".to_string(),
            "a".to_string(),
        ],
        vec!["a".to_string(), "bb".to_string(), "a".to_string()],
        vec!["abba".to_string()],
    ];

    assert!(compare_results(result, expected));
}

#[test]
fn test_no_palindromes_longer_than_one() {
    // Input: s = "abc"
    // Output: [["a","b","c"]]
    let s = "abc".to_string();
    let result = Solution::partition(s);

    let expected = vec![vec!["a".to_string(), "b".to_string(), "c".to_string()]];

    assert!(compare_results(result, expected));
}
