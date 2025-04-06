use longest_substring_without_repeating_characters::Solution;

#[test]
fn test_example_1() {
    let s = "abcabcbb".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 3);
}

#[test]
fn test_example_2() {
    let s = "bbbbb".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 1);
}

#[test]
fn test_example_3() {
    let s = "pwwkew".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 3);
}

#[test]
fn test_empty_string() {
    let s = "".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 0);
}

#[test]
fn test_single_character() {
    let s = "a".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 1);
}

#[test]
fn test_all_unique_characters() {
    let s = "abcdefg".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 7);
}

#[test]
fn test_with_spaces_and_symbols() {
    let s = "ab c!d#e".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 7);
}

#[test]
fn test_repeating_at_end() {
    let s = "abcdefgabcdefg".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 7);
}

#[test]
fn test_repeating_in_middle() {
    let s = "abcbdef".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 5);
}
