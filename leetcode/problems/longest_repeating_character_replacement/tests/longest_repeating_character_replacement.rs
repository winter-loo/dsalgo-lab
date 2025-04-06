use longest_repeating_character_replacement::Solution;

#[test]
fn test_example_1() {
    let s = "ABAB".to_string();
    let k = 2;
    assert_eq!(Solution::character_replacement(s, k), 4);
}

#[test]
fn test_example_2() {
    let s = "AABABBA".to_string();
    let k = 1;
    assert_eq!(Solution::character_replacement(s, k), 4);
}

#[test]
fn test_no_replacements_needed() {
    let s = "AAAA".to_string();
    let k = 0;
    assert_eq!(Solution::character_replacement(s, k), 4);
}

#[test]
fn test_all_different_characters() {
    let s = "ABCD".to_string();
    let k = 2;
    assert_eq!(Solution::character_replacement(s, k), 3);
}

#[test]
fn test_k_equals_string_length() {
    let s = "ABCD".to_string();
    let k = 4;
    assert_eq!(Solution::character_replacement(s, k), 4);
}

#[test]
fn test_single_character() {
    let s = "A".to_string();
    let k = 0;
    assert_eq!(Solution::character_replacement(s, k), 1);
}

#[test]
fn test_long_string() {
    let s = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let k = 3;
    assert_eq!(Solution::character_replacement(s, k), 4);
}
