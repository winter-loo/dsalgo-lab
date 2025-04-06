use daily_temperatures::Solution;

#[test]
fn test_example_1() {
    let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let expected = vec![1, 1, 4, 2, 1, 1, 0, 0];
    assert_eq!(Solution::daily_temperatures(temperatures), expected);
}

#[test]
fn test_example_2() {
    let temperatures = vec![30, 40, 50, 60];
    let expected = vec![1, 1, 1, 0];
    assert_eq!(Solution::daily_temperatures(temperatures), expected);
}

#[test]
fn test_example_3() {
    let temperatures = vec![30, 60, 90];
    let expected = vec![1, 1, 0];
    assert_eq!(Solution::daily_temperatures(temperatures), expected);
}

#[test]
fn test_decreasing_temperatures() {
    let temperatures = vec![90, 80, 70, 60, 50];
    let expected = vec![0, 0, 0, 0, 0];
    assert_eq!(Solution::daily_temperatures(temperatures), expected);
}

#[test]
fn test_same_temperatures() {
    let temperatures = vec![70, 70, 70, 70];
    let expected = vec![0, 0, 0, 0];
    assert_eq!(Solution::daily_temperatures(temperatures), expected);
}

#[test]
fn test_fluctuating_temperatures() {
    let temperatures = vec![73, 72, 75, 71, 69, 72, 76, 73];
    let expected = vec![2, 1, 4, 2, 1, 1, 0, 0];
    assert_eq!(Solution::daily_temperatures(temperatures), expected);
}
