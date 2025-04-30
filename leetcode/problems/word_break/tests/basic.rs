use word_break::Solution;

#[test]
fn test_example_1() {
    // Input: s = "leetcode", wordDict = ["leet","code"]
    // Output: true
    let s = "leetcode".to_string();
    let word_dict = vec!["leet".to_string(), "code".to_string()];
    assert_eq!(Solution::word_break(s, word_dict), true);
}

#[test]
fn test_example_2() {
    // Input: s = "applepenapple", wordDict = ["apple","pen"]
    // Output: true
    let s = "applepenapple".to_string();
    let word_dict = vec!["apple".to_string(), "pen".to_string()];
    assert_eq!(Solution::word_break(s, word_dict), true);
}

#[test]
fn test_example_3() {
    // Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
    // Output: false
    let s = "catsandog".to_string();
    let word_dict = vec![
        "cats".to_string(),
        "dog".to_string(),
        "sand".to_string(),
        "and".to_string(),
        "cat".to_string(),
    ];
    assert_eq!(Solution::word_break(s, word_dict), false);
}

#[test]
fn test_empty_dict() {
    // Empty dictionary should return false for any non-empty string
    let s = "a".to_string();
    let word_dict: Vec<String> = vec![];
    assert_eq!(Solution::word_break(s, word_dict), false);
}

#[test]
fn test_single_char() {
    // Single character in dictionary and string
    let s = "a".to_string();
    let word_dict = vec!["a".to_string()];
    assert_eq!(Solution::word_break(s, word_dict), true);
}

#[test]
fn test_repeated_words() {
    // Test with repeated words in the segmentation
    let s = "aaaaaaa".to_string();
    let word_dict = vec!["a".to_string(), "aa".to_string(), "aaa".to_string()];
    assert_eq!(Solution::word_break(s, word_dict), true);
}
