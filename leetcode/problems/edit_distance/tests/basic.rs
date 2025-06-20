use edit_distance::Solution;

#[test]
fn test_example_1() {
    // Input: word1 = "horse", word2 = "ros"
    // Output: 3
    // Explanation:
    // horse -> rorse (replace 'h' with 'r')
    // rorse -> rose (remove 'r')
    // rose -> ros (remove 'e')
    let word1 = "horse".to_string();
    let word2 = "ros".to_string();
    assert_eq!(Solution::min_distance(word1, word2), 3);
}

#[test]
fn test_example_2() {
    // Input: word1 = "intention", word2 = "execution"
    // Output: 5
    // Explanation:
    // intention -> inention (remove 't')
    // inention -> enention (replace 'i' with 'e')
    // enention -> exention (replace 'n' with 'x')
    // exention -> exection (replace 'n' with 'c')
    // exection -> execution (insert 'u')
    let word1 = "intention".to_string();
    let word2 = "execution".to_string();
    assert_eq!(Solution::min_distance(word1, word2), 5);
}

#[test]
fn test_empty_strings() {
    // Test with empty strings
    let word1 = "".to_string();
    let word2 = "".to_string();
    assert_eq!(Solution::min_distance(word1, word2), 0);
}

#[test]
fn test_one_empty_string() {
    // Test with one empty string
    let word1 = "hello".to_string();
    let word2 = "".to_string();
    assert_eq!(Solution::min_distance(word1, word2), 5);

    let word1 = "".to_string();
    let word2 = "world".to_string();
    assert_eq!(Solution::min_distance(word1, word2), 5);
}

#[test]
fn test_same_strings() {
    // Test with identical strings
    let word1 = "hello".to_string();
    let word2 = "hello".to_string();
    assert_eq!(Solution::min_distance(word1, word2), 0);
}

#[test]
fn test_single_character_difference() {
    // Test with strings that differ by a single character
    let word1 = "hello".to_string();
    let word2 = "hallo".to_string();
    assert_eq!(Solution::min_distance(word1, word2), 1);
}

#[test]
fn test_completely_different_strings() {
    // Test with completely different strings of the same length
    let word1 = "abcde".to_string();
    let word2 = "fghij".to_string();
    assert_eq!(Solution::min_distance(word1, word2), 5);
}

//                  a
//              0   1
//          ┌─────────
//        0 │   0   1
//      a 1 │   1   0
//      b 2 │   2   1
#[test]
fn test_example_3() {
    let word1 = "a".to_string();
    let word2 = "ab".to_string();
    assert_eq!(Solution::min_distance(word1, word2), 1);
}

#[test]
fn test_example_4() {
    let word1 = "zoo".to_string();
    let word2 = "zo".to_string();
    assert_eq!(Solution::min_distance(word1, word2), 1);
}

#[test]
fn test_example_5() {
    let word1 = "zoologicoarchaeologist".to_string();
    let word2 = "zoogeologist".to_string();
    assert_eq!(Solution::min_distance(word1, word2), 10);
}
