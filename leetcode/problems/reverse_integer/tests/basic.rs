use reverse_integer::Solution;

#[test]
fn test_example_1() {
    // Input: x = 123
    // Output: 321
    assert_eq!(Solution::reverse(123), 321);
}

#[test]
fn test_example_2() {
    // Input: x = -123
    // Output: -321
    assert_eq!(Solution::reverse(-123), -321);
}

#[test]
fn test_example_3() {
    // Input: x = 120
    // Output: 21
    assert_eq!(Solution::reverse(120), 21);
}

#[test]
fn test_zero() {
    // Test with zero
    assert_eq!(Solution::reverse(0), 0);
}

#[test]
fn test_single_digit() {
    // Test with single digit
    assert_eq!(Solution::reverse(5), 5);
    assert_eq!(Solution::reverse(-8), -8);
}

#[test]
fn test_overflow_positive() {
    // Test with number that will overflow when reversed
    // 2147483647 (INT_MAX) reversed is 7463847412, which overflows
    assert_eq!(Solution::reverse(1463847412), 2147483641);
    assert_eq!(Solution::reverse(2147483647), 0); // Should return 0 due to overflow
}

#[test]
fn test_overflow_negative() {
    // Test with negative number that will overflow when reversed
    // -2147483648 (INT_MIN) reversed is -8463847412, which overflows
    assert_eq!(Solution::reverse(-1463847412), -2147483641);
    assert_eq!(Solution::reverse(-2147483648), 0); // Should return 0 due to overflow
}

#[test]
fn test_palindrome() {
    // Test with palindrome numbers
    assert_eq!(Solution::reverse(121), 121);
    assert_eq!(Solution::reverse(-121), -121);
}

#[test]
fn test_leading_zeros() {
    // Test with numbers that have trailing zeros (which become leading zeros when reversed)
    assert_eq!(Solution::reverse(1000), 1);
    assert_eq!(Solution::reverse(1010), 101);
}
