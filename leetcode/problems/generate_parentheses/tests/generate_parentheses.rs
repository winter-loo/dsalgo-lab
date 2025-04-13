use generate_parentheses::Solution;
use std::collections::HashSet;

#[test]
fn test_example_1() {
    let n = 3;
    let result = Solution::generate_parenthesis(n);
    let expected = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];

    // Convert to HashSet for unordered comparison
    let result_set: HashSet<String> = result.into_iter().collect();
    let expected_set: HashSet<String> = expected.into_iter().map(String::from).collect();

    assert_eq!(result_set, expected_set);
}

#[test]
fn test_example_2() {
    let n = 1;
    let result = Solution::generate_parenthesis(n);
    let expected = vec!["()"];

    // Convert to HashSet for unordered comparison
    let result_set: HashSet<String> = result.into_iter().collect();
    let expected_set: HashSet<String> = expected.into_iter().map(String::from).collect();

    assert_eq!(result_set, expected_set);
}

#[test]
fn test_n_2() {
    let n = 2;
    let result = Solution::generate_parenthesis(n);
    let expected = vec!["(())", "()()"];

    // Convert to HashSet for unordered comparison
    let result_set: HashSet<String> = result.into_iter().collect();
    let expected_set: HashSet<String> = expected.into_iter().map(String::from).collect();

    assert_eq!(result_set, expected_set);
}

#[test]
fn test_example_3() {
    let n = 4;
    let result = Solution::generate_parenthesis(n);
    let expected = vec![
        "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
        "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()",
    ];

    // Convert to HashSet for unordered comparison
    let result_set: HashSet<String> = result.into_iter().collect();
    let expected_set: HashSet<String> = expected.into_iter().map(String::from).collect();

    assert_eq!(result_set, expected_set);
}
