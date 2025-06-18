use valid_parenthesis_string::Solution;

#[test]
fn test_example_1() {
    // Input: s = "()"
    // Output: true
    let s = String::from("()");
    assert_eq!(Solution::check_valid_string(s), true);
}

#[test]
fn test_example_2() {
    // Input: s = "(*)"
    // Output: true
    let s = String::from("(*)");
    assert_eq!(Solution::check_valid_string(s), true);
}

#[test]
fn test_example_3() {
    // Input: s = "(*))"
    // Output: true
    let s = String::from("(*))");
    assert_eq!(Solution::check_valid_string(s), true);
}

#[test]
fn test_invalid_string() {
    // Test an invalid string
    let s = String::from("(()");
    assert_eq!(Solution::check_valid_string(s), false);
}

#[test]
fn test_only_asterisks() {
    // Test string with only asterisks
    let s = String::from("***");
    assert_eq!(Solution::check_valid_string(s), true);
}

#[test]
fn test_empty_string() {
    // Test empty string
    let s = String::from("");
    assert_eq!(Solution::check_valid_string(s), true);
}

#[test]
fn test_complex_valid_string() {
    // Test a more complex valid string
    let s = String::from("(*)(*)");
    assert_eq!(Solution::check_valid_string(s), true);
}

#[test]
fn test_complex_invalid_string() {
    // Test a more complex invalid string
    let s = String::from("((*)(*))(");
    assert_eq!(Solution::check_valid_string(s), false);
}

#[test]
fn test_asterisk_as_empty() {
    // Test case where asterisks need to be treated as empty strings
    let s = String::from("()()()");
    assert_eq!(Solution::check_valid_string(s), true);
}

#[test]
fn test_asterisk_as_parentheses() {
    // Test case where asterisks need to be treated as parentheses
    let s = String::from("(*))*(");
    assert_eq!(Solution::check_valid_string(s), true);
}
