use partition_labels::Solution;

#[test]
fn test_example_1() {
    // Input: s = "ababcbacadefegdehijhklij"
    // Output: [9,7,8]
    let s = String::from("ababcbacadefegdehijhklij");
    assert_eq!(Solution::partition_labels(s), vec![9, 7, 8]);
}

#[test]
fn test_example_2() {
    // Input: s = "eccbbbbdec"
    // Output: [10]
    let s = String::from("eccbbbbdec");
    assert_eq!(Solution::partition_labels(s), vec![10]);
}

#[test]
fn test_single_character() {
    // Test with a single character
    let s = String::from("a");
    assert_eq!(Solution::partition_labels(s), vec![1]);
}

#[test]
fn test_all_distinct_characters() {
    // Test with all distinct characters
    let s = String::from("abcdef");
    assert_eq!(Solution::partition_labels(s), vec![1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_all_same_character() {
    // Test with all the same character
    let s = String::from("aaaaa");
    assert_eq!(Solution::partition_labels(s), vec![5]);
}

#[test]
fn test_alternating_characters() {
    // Test with alternating characters
    let s = String::from("ababab");
    assert_eq!(Solution::partition_labels(s), vec![6]);
}

#[test]
fn test_complex_pattern() {
    // Test with a more complex pattern
    let s = String::from("qiejxqfnqceocmy");
    assert_eq!(Solution::partition_labels(s), vec![13, 1, 1]);
}

#[test]
fn test_multiple_partitions() {
    // Test with multiple partitions
    let s = String::from("abaccbdeffed");
    assert_eq!(Solution::partition_labels(s), vec![6, 6]);
}
