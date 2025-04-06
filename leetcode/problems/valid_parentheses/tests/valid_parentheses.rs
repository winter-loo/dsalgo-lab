use valid_parentheses::Solution;

#[test]
fn test_example_1() {
    let s = String::from("()");
    assert_eq!(Solution::is_valid(s), true);
}

#[test]
fn test_example_2() {
    let s = String::from("()[]{}");
    assert_eq!(Solution::is_valid(s), true);
}

#[test]
fn test_example_3() {
    let s = String::from("(]");
    assert_eq!(Solution::is_valid(s), false);
}

#[test]
fn test_example_4() {
    let s = String::from("([)]");
    assert_eq!(Solution::is_valid(s), false);
}

#[test]
fn test_example_5() {
    let s = String::from("{[]}");
    assert_eq!(Solution::is_valid(s), true);
}
