use minimum_window_substring::Solution;

#[test]
fn test_example_1() {
    let s = String::from("ADOBECODEBANC");
    let t = String::from("ABC");
    assert_eq!(Solution::min_window(s, t), "BANC");
}

#[test]
fn test_example_2() {
    let s = String::from("a");
    let t = String::from("a");
    assert_eq!(Solution::min_window(s, t), "a");
}

#[test]
fn test_example_3() {
    let s = String::from("a");
    let t = String::from("aa");
    assert_eq!(Solution::min_window(s, t), "");
}

#[test]
fn test_empty_strings() {
    let s = String::from("");
    let t = String::from("");
    assert_eq!(Solution::min_window(s, t), "");
}

#[test]
fn test_no_match() {
    let s = String::from("XYZ");
    let t = String::from("ABC");
    assert_eq!(Solution::min_window(s, t), "");
}

#[test]
fn test_duplicate_characters() {
    let s = String::from("ADOBECODEBANCBB");
    let t = String::from("ABBC");
    assert_eq!(Solution::min_window(s, t), "BANCB");
}
