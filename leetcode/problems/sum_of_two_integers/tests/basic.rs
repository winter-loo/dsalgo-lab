use sum_of_two_integers::Solution;

#[test]
fn test_example_1() {
    // Input: a = 1, b = 2
    // Output: 3
    assert_eq!(Solution::get_sum(1, 2), 3);
}

#[test]
fn test_example_2() {
    // Input: a = 2, b = 3
    // Output: 5
    assert_eq!(Solution::get_sum(2, 3), 5);
}

#[test]
fn test_negative_numbers() {
    // Test with negative numbers
    assert_eq!(Solution::get_sum(-1, 1), 0);
    assert_eq!(Solution::get_sum(-2, -3), -5);
    assert_eq!(Solution::get_sum(-10, 5), -5);
}

#[test]
fn test_zero() {
    // Test with zero
    assert_eq!(Solution::get_sum(0, 0), 0);
    assert_eq!(Solution::get_sum(0, 5), 5);
    assert_eq!(Solution::get_sum(10, 0), 10);
}

#[test]
fn test_large_numbers() {
    // Test with larger numbers
    assert_eq!(Solution::get_sum(100, 200), 300);
    assert_eq!(Solution::get_sum(999, 1), 1000);
}

#[test]
fn test_same_number() {
    // Test adding a number to itself
    assert_eq!(Solution::get_sum(5, 5), 10);
    assert_eq!(Solution::get_sum(-7, -7), -14);
}

#[test]
fn test_boundary_values() {
    // Test with boundary values within the constraints
    assert_eq!(Solution::get_sum(1000, 1000), 2000);
    assert_eq!(Solution::get_sum(-1000, -1000), -2000);
    assert_eq!(Solution::get_sum(-1000, 1000), 0);
}
