use longest_palindromic_substring::Solution;

#[test]
fn test_example_1() {
    // Input: s = "babad"
    // Output: "bab" or "aba"
    let s = "babad".to_string();
    let result = Solution::longest_palindrome(s);
    assert!(result == "bab" || result == "aba");
}

#[test]
fn test_example_2() {
    // Input: s = "cbbd"
    // Output: "bb"
    let s = "cbbd".to_string();
    assert_eq!(Solution::longest_palindrome(s), "bb");
}

#[test]
fn test_single_char() {
    // Single character is a palindrome
    let s = "a".to_string();
    assert_eq!(Solution::longest_palindrome(s), "a");
}

#[test]
fn test_all_same_chars() {
    // All characters are the same
    let s = "aaaaa".to_string();
    assert_eq!(Solution::longest_palindrome(s), "aaaaa");
}

#[test]
fn test_no_palindrome_longer_than_1() {
    // No palindrome longer than 1 character
    let s = "abcdef".to_string();
    // Any single character is a valid answer
    let result = Solution::longest_palindrome(s);
    assert_eq!(result.len(), 1);
}

#[test]
fn test_multiple_palindromes() {
    // String with multiple palindromes of the same length
    let s = "aacabdkacaa".to_string();
    assert_eq!(Solution::longest_palindrome(s), "aca");
}
