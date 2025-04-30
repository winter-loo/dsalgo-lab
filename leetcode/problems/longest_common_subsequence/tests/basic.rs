use longest_common_subsequence::Solution;

#[test]
fn test_example_1() {
    // Input: text1 = "abcde", text2 = "ace" 
    // Output: 3  
    // Explanation: The longest common subsequence is "ace" and its length is 3.
    let text1 = "abcde".to_string();
    let text2 = "ace".to_string();
    assert_eq!(Solution::longest_common_subsequence(text1, text2), 3);
}

#[test]
fn test_example_2() {
    // Input: text1 = "abc", text2 = "abc"
    // Output: 3
    // Explanation: The longest common subsequence is "abc" and its length is 3.
    let text1 = "abc".to_string();
    let text2 = "abc".to_string();
    assert_eq!(Solution::longest_common_subsequence(text1, text2), 3);
}

#[test]
fn test_example_3() {
    // Input: text1 = "abc", text2 = "def"
    // Output: 0
    // Explanation: There is no such common subsequence, so the result is 0.
    let text1 = "abc".to_string();
    let text2 = "def".to_string();
    assert_eq!(Solution::longest_common_subsequence(text1, text2), 0);
}

#[test]
fn test_one_empty_string() {
    // One empty string should result in LCS of 0
    let text1 = "abc".to_string();
    let text2 = "".to_string();
    assert_eq!(Solution::longest_common_subsequence(text1, text2), 0);
}

#[test]
fn test_partial_match() {
    // Partial match with characters in different positions
    let text1 = "abcdef".to_string();
    let text2 = "acf".to_string();
    assert_eq!(Solution::longest_common_subsequence(text1, text2), 3);
}

#[test]
fn test_longer_strings() {
    // Test with longer strings
    let text1 = "abcdefghijklmnopqrstuvwxyz".to_string();
    let text2 = "abcdefghijklmnopqrstuvwxyz".to_string();
    assert_eq!(Solution::longest_common_subsequence(text1, text2), 26);
}
