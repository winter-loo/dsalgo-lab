use counting_bits::Solution;

#[test]
fn test_example_1() {
    // Input: n = 2
    // Output: [0,1,1]
    let n = 2;
    assert_eq!(Solution::count_bits(n), vec![0, 1, 1]);
}

#[test]
fn test_example_2() {
    // Input: n = 5
    // Output: [0,1,1,2,1,2]
    let n = 5;
    assert_eq!(Solution::count_bits(n), vec![0, 1, 1, 2, 1, 2]);
}

#[test]
fn test_zero() {
    // Test with n = 0
    let n = 0;
    assert_eq!(Solution::count_bits(n), vec![0]);
}

#[test]
fn test_one() {
    // Test with n = 1
    let n = 1;
    assert_eq!(Solution::count_bits(n), vec![0, 1]);
}

#[test]
fn test_power_of_two() {
    // Test with n = 8
    // 0 -> 0, 1 -> 1, 2 -> 1, 3 -> 2, 4 -> 1, 5 -> 2, 6 -> 2, 7 -> 3, 8 -> 1
    let n = 8;
    assert_eq!(Solution::count_bits(n), vec![0, 1, 1, 2, 1, 2, 2, 3, 1]);
}

#[test]
fn test_larger_number() {
    // Test with n = 15
    // Binary: 0, 1, 10, 11, 100, 101, 110, 111, 1000, 1001, 1010, 1011, 1100, 1101, 1110, 1111
    let n = 15;
    assert_eq!(Solution::count_bits(n), vec![0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4]);
}
