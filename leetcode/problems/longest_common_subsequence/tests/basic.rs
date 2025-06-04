use longest_common_subsequence::Solution;

#[test]
fn test_example_0() {
    let text1 = "abcde".to_string();
    let text2 = "aceb".to_string();
    assert_eq!(Solution::longest_common_subsequence(text1, text2), 3);
}

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

#[test]
fn test_example_4() {
    let text1 = "ezupkr".to_string();
    let text2 = "ubmrapg".to_string();
    assert_eq!(Solution::longest_common_subsequence(text1, text2), 2);
    //              g   p   a   r   m   b   u
    //   I/J    7   6   5   4   3   2   1   0
    //       +-------------------------------
    //     6 |  0   0   0   0   0   0   0   0
    //  r  5 |  0   0   0   0   1   1   1   1
    //  k  4 |  0   0   0   0   1   1   1   1
    //  p  3 |  0   0   1   1   1   1   1   1
    //  u  2 |  0   0   1   1   1   1   1   2
    //  z  1 |  0   0   1   1   1   1   1   1
    //  e  0 |  0   0   1   1   1   1   1   1
    //
    //  -------alternative/regular method----------------
    //
    //              u   b   m   r   a   p   g
    //   I/J    0   1   2   3   4   5   6   7
    //       +-------------------------------
    //     0 |  0   0   0   0   0   0   0   0
    //  e  1 |  0   0   0   0   0   0   0   0
    //  z  2 |  0   0   0   0   0   0   0   0
    //  u  3 |  0   1   1   1   1   1   1   1
    //  p  4 |  0   1   1   1   1   1   2   2
    //  k  5 |  0   1   1   1   1   1   1   1
    //  r  6 |  0   1   1   1   2   2   2   2
}

#[test]
fn test_example_5() {
    let text1 = "oxcpqrsvwf".to_string();
    let text2 = "shmtulqrypy".to_string();
    assert_eq!(Solution::longest_common_subsequence(text1, text2), 2);
}

#[test]
fn test_example_6() {
    let text1 = "hofubmnylkra".to_string();
    let text2 = "pqhgxgdofcvmr".to_string();
    assert_eq!(Solution::longest_common_subsequence(text1, text2), 5);
}

#[test]
fn test_example_7() {
    // in this example, we could know that when 'q' matches we should not
    // choose it.
    let text1 = "mhunuzqrkzsnidwbun".to_string();
    let text2 = "szulspmhwpazoxijwbq".to_string();
    assert_eq!(Solution::longest_common_subsequence(text1, text2), 6);
}
