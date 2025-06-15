use interleaving_string::Solution;

#[test]
fn test_example_1() {
    // Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
    // Output: true
    let s1 = "aabcc".to_string();
    let s2 = "dbbca".to_string();
    let s3 = "aadbbcbcac".to_string();
    assert_eq!(Solution::is_interleave(s1, s2, s3), true);
}

#[test]
fn test_example_2() {
    // Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
    // Output: false
    let s1 = "aabcc".to_string();
    let s2 = "dbbca".to_string();
    let s3 = "aadbbbaccc".to_string();
    assert_eq!(Solution::is_interleave(s1, s2, s3), false);
}

#[test]
fn test_example_3() {
    // Input: s1 = "", s2 = "", s3 = ""
    // Output: true
    let s1 = "".to_string();
    let s2 = "".to_string();
    let s3 = "".to_string();
    assert_eq!(Solution::is_interleave(s1, s2, s3), true);
}

#[test]
fn test_length_mismatch() {
    // Length mismatch should return false
    let s1 = "abc".to_string();
    let s2 = "def".to_string();
    let s3 = "abcde".to_string();
    assert_eq!(Solution::is_interleave(s1, s2, s3), false);
}

#[test]
fn test_one_empty_string() {
    // One string is empty
    let s1 = "abc".to_string();
    let s2 = "".to_string();
    let s3 = "abc".to_string();
    assert_eq!(Solution::is_interleave(s1, s2, s3), true);
}

#[test]
fn test_complex_interleaving() {
    // More complex interleaving
    let s1 = "abcde".to_string();
    let s2 = "fghij".to_string();
    let s3 = "afbgchdiej".to_string();
    assert_eq!(Solution::is_interleave(s1, s2, s3), true);
}

#[test]
fn test_example_4() {
    let s1 = "ab".to_string();
    let s2 = "ccd".to_string();
    let s3 = "acdab".to_string();
    assert_eq!(Solution::is_interleave(s1, s2, s3), false);
}
