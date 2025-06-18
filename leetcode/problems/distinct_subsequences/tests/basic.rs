use distinct_subsequences::Solution;

#[test]
fn test_example_1() {
    // Input: s = "rabbbit", t = "rabbit"
    // Output: 3
    // Explanation: There are 3 ways to get "rabbit" from "rabbbit"
    let s = "rabbbit".to_string();
    let t = "rabbit".to_string();
    assert_eq!(Solution::num_distinct(s, t), 3);
}

#[test]
fn test_example_2() {
    // Input: s = "babgbag", t = "bag"
    // Output: 5
    // Explanation: There are 5 ways to get "bag" from "babgbag"
    let s = "babgbag".to_string();
    let t = "bag".to_string();
    assert_eq!(Solution::num_distinct(s, t), 5);
}

#[test]
fn test_empty_strings() {
    // Test with empty target string
    let s = "abc".to_string();
    let t = "".to_string();
    assert_eq!(Solution::num_distinct(s, t), 1);

    // Test with both empty strings
    let s = "".to_string();
    let t = "".to_string();
    assert_eq!(Solution::num_distinct(s, t), 1);
}

#[test]
fn test_no_subsequence() {
    // Test when t is not a subsequence of s
    let s = "abc".to_string();
    let t = "def".to_string();
    assert_eq!(Solution::num_distinct(s, t), 0);
}

#[test]
fn test_same_strings() {
    // Test when s and t are the same
    let s = "hello".to_string();
    let t = "hello".to_string();
    assert_eq!(Solution::num_distinct(s, t), 1);
}

#[test]
fn test_repeated_characters() {
    // Test with repeated characters
    let s = "aaaaaa".to_string();
    let t = "aaa".to_string();
    assert_eq!(Solution::num_distinct(s, t), 20); // C(6,3) = 20 ways
}

#[test]
fn test_longer_target() {
    // Test when t is longer than s
    let s = "abc".to_string();
    let t = "abcd".to_string();
    assert_eq!(Solution::num_distinct(s, t), 0);
}
