use evaluate_reverse_polish_notation::Solution;

#[test]
fn test_example_1() {
    let tokens = vec![
        "2".to_string(),
        "1".to_string(),
        "+".to_string(),
        "3".to_string(),
        "*".to_string(),
    ];
    assert_eq!(Solution::eval_rpn(tokens), 9);
}

#[test]
fn test_example_2() {
    let tokens = vec![
        "4".to_string(),
        "13".to_string(),
        "5".to_string(),
        "/".to_string(),
        "+".to_string(),
    ];
    assert_eq!(Solution::eval_rpn(tokens), 6);
}

#[test]
fn test_example_3() {
    let tokens = vec![
        "10".to_string(),
        "6".to_string(),
        "9".to_string(),
        "3".to_string(),
        "+".to_string(),
        "-11".to_string(),
        "*".to_string(),
        "/".to_string(),
        "*".to_string(),
        "17".to_string(),
        "+".to_string(),
        "5".to_string(),
        "+".to_string(),
    ];
    assert_eq!(Solution::eval_rpn(tokens), 22);
}

#[test]
fn test_single_number() {
    let tokens = vec!["42".to_string()];
    assert_eq!(Solution::eval_rpn(tokens), 42);
}

#[test]
fn test_simple_addition() {
    let tokens = vec!["3".to_string(), "4".to_string(), "+".to_string()];
    assert_eq!(Solution::eval_rpn(tokens), 7);
}

#[test]
fn test_division_truncation() {
    let tokens = vec!["7".to_string(), "2".to_string(), "/".to_string()];
    assert_eq!(Solution::eval_rpn(tokens), 3);

    let tokens = vec!["-7".to_string(), "2".to_string(), "/".to_string()];
    assert_eq!(Solution::eval_rpn(tokens), -3);
}
