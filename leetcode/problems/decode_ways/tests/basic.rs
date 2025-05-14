use decode_ways::Solution;

#[test]
fn test_example_1() {
    // Input: s = "12"
    // Output: 2
    // Explanation: "12" could be decoded as "AB" (1 2) or "L" (12).
    let s = "12".to_string();
    assert_eq!(Solution::num_decodings(s), 2);
}

#[test]
fn test_example_2() {
    // Input: s = "226"
    // Output: 3
    // Explanation: "226" could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).
    let s = "226".to_string();
    assert_eq!(Solution::num_decodings(s), 3);
}

#[test]
fn test_example_3() {
    // Input: s = "06"
    // Output: 0
    // Explanation: "06" cannot be mapped to "F" because of the leading zero.
    let s = "06".to_string();
    assert_eq!(Solution::num_decodings(s), 0);
}

#[test]
fn test_single_digit() {
    // A single digit that is not zero has one way to decode
    let s = "5".to_string();
    assert_eq!(Solution::num_decodings(s), 1);
}

#[test]
fn test_zero_digit() {
    // A single zero digit has no valid decoding
    let s = "0".to_string();
    assert_eq!(Solution::num_decodings(s), 0);
}

#[test]
fn test_with_zeros() {
    // String with zeros that can only be decoded as part of two-digit numbers
    let s = "1201234".to_string();
    assert_eq!(Solution::num_decodings(s), 3);
}

#[test]
fn test_example_4() {
    let s = "11111".to_string();
    assert_eq!(Solution::num_decodings(s), 8);
}

#[test]
fn test_example_5() {
    let s = "120".to_string();
    assert_eq!(Solution::num_decodings(s), 1);
}

#[test]
fn test_example_6() {
    let s = "1201".to_string();
    assert_eq!(Solution::num_decodings(s), 1);
}

// resulting Time Limit Exceeded
#[test]
fn test_perf() {
    let s = "111111111111111111111111111111111111111111111".to_string();
    assert_eq!(Solution::num_decodings(s), 1836311903);
}
