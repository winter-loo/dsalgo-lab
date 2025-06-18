use regular_expression_matching::Solution;

#[test]
fn test_example_1() {
    // Input: s = "aa", p = "a"
    // Output: false
    // Explanation: "a" does not match the entire string "aa".
    let s = "aa".to_string();
    let p = "a".to_string();
    assert_eq!(Solution::is_match(s, p), false);
}

#[test]
fn test_example_2() {
    // Input: s = "aa", p = "a*"
    // Output: true
    // Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
    let s = "aa".to_string();
    let p = "a*".to_string();
    assert_eq!(Solution::is_match(s, p), true);
}

#[test]
fn test_example_3() {
    // Input: s = "ab", p = ".*"
    // Output: true
    // Explanation: ".*" means "zero or more (*) of any character (.)".
    let s = "ab".to_string();
    let p = ".*".to_string();
    assert_eq!(Solution::is_match(s, p), true);
}

#[test]
fn test_empty_string() {
    // Test with empty string and pattern
    let s = "".to_string();
    let p = "".to_string();
    assert_eq!(Solution::is_match(s, p), true);
}

#[test]
fn test_empty_string_with_pattern() {
    // Test with empty string and non-empty pattern
    let s = "".to_string();
    let p = "a*".to_string();
    assert_eq!(Solution::is_match(s, p), true);

    let s = "".to_string();
    let p = "a*b*".to_string();
    assert_eq!(Solution::is_match(s, p), true);

    let s = "".to_string();
    let p = "a".to_string();
    assert_eq!(Solution::is_match(s, p), false);
}

#[test]
fn test_complex_pattern() {
    // Test with more complex patterns
    let s = "aab".to_string();
    let p = "c*a*b".to_string();
    assert_eq!(Solution::is_match(s, p), true);

    let s = "mississippi".to_string();
    let p = "mis*is*p*.".to_string();
    assert_eq!(Solution::is_match(s, p), false);

    let s = "mississippi".to_string();
    let p = "mis*is*ip*i".to_string();
    assert_eq!(Solution::is_match(s, p), true);
}

#[test]
fn test_dot_character() {
    // Test with dot character
    let s = "aab".to_string();
    let p = ".a.".to_string();
    assert_eq!(Solution::is_match(s, p), true);

    let s = "aaa".to_string();
    let p = "a.a".to_string();
    assert_eq!(Solution::is_match(s, p), true);

    let s = "aaa".to_string();
    let p = "ab.".to_string();
    assert_eq!(Solution::is_match(s, p), false);
}
