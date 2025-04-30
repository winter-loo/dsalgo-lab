use palindromic_substrings::Solution;

#[test]
fn test_example_1() {
    // Input: s = "abc"
    // Output: 3
    // Explanation: Three palindromic strings: "a", "b", "c".
    let s = "abc".to_string();
    assert_eq!(Solution::count_substrings(s), 3);
}

#[test]
fn test_example_2() {
    // Input: s = "aaa"
    // Output: 6
    // Explanation: Six palindromic strings: "a", "a", "a", "aa", "aa", "aaa".
    let s = "aaa".to_string();
    assert_eq!(Solution::count_substrings(s), 6);
}

#[test]
fn test_single_char() {
    // Single character is a palindrome
    let s = "a".to_string();
    assert_eq!(Solution::count_substrings(s), 1);
}

#[test]
fn test_two_different_chars() {
    // Two different characters
    let s = "ab".to_string();
    assert_eq!(Solution::count_substrings(s), 2);
}

#[test]
fn test_two_same_chars() {
    // Two same characters
    let s = "aa".to_string();
    assert_eq!(Solution::count_substrings(s), 3);
}

#[test]
fn test_mixed_palindromes() {
    // String with various palindromes
    let s = "abccba".to_string();
    assert_eq!(Solution::count_substrings(s), 9);
}
