use permutation_in_string::Solution;

#[test]
fn test_example_1() {
    let s1 = "ab".to_string();
    let s2 = "eidbaooo".to_string();
    assert_eq!(Solution::check_inclusion(s1, s2), true);
}

#[test]
fn test_example_2() {
    let s1 = "ab".to_string();
    let s2 = "eidboaoo".to_string();
    assert_eq!(Solution::check_inclusion(s1, s2), false);
}

#[test]
fn test_s1_longer_than_s2() {
    let s1 = "hello".to_string();
    let s2 = "hi".to_string();
    assert_eq!(Solution::check_inclusion(s1, s2), false);
}

#[test]
fn test_exact_match() {
    let s1 = "abc".to_string();
    let s2 = "abc".to_string();
    assert_eq!(Solution::check_inclusion(s1, s2), true);
}

#[test]
fn test_permutation_at_beginning() {
    let s1 = "abc".to_string();
    let s2 = "cbaefd".to_string();
    assert_eq!(Solution::check_inclusion(s1, s2), true);
}

#[test]
fn test_permutation_at_end() {
    let s1 = "abc".to_string();
    let s2 = "defcba".to_string();
    assert_eq!(Solution::check_inclusion(s1, s2), true);
}

#[test]
fn test_permutation_in_middle() {
    let s1 = "abc".to_string();
    let s2 = "defcabghi".to_string();
    assert_eq!(Solution::check_inclusion(s1, s2), true);
}

#[test]
fn test_repeated_characters() {
    let s1 = "aab".to_string();
    let s2 = "eidbaaooo".to_string();
    assert_eq!(Solution::check_inclusion(s1, s2), true);
}

#[test]
fn test_no_permutation() {
    let s1 = "xyz".to_string();
    let s2 = "abcdefg".to_string();
    assert_eq!(Solution::check_inclusion(s1, s2), false);
}
