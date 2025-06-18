use burst_balloons::Solution;

#[test]
fn test_example_1() {
    // Input: nums = [3,1,5,8]
    // Output: 167
    // Explanation:
    // nums = [3,1,5,8] --> [3,5,8] --> [3,8] --> [8] --> []
    // coins =  3*1*5    +   3*5*8   +  1*3*8  + 1*8*1 = 167
    let nums = vec![3, 1, 5, 8];
    assert_eq!(Solution::max_coins(nums), 167);
}

#[test]
fn test_example_2() {
    // Input: nums = [1,5]
    // Output: 10
    let nums = vec![1, 5];
    assert_eq!(Solution::max_coins(nums), 10);
}

#[test]
fn test_single_balloon() {
    // Test with a single balloon
    let nums = vec![7];
    assert_eq!(Solution::max_coins(nums), 7);
}

#[test]
fn test_all_same_values() {
    // Test with all balloons having the same value
    let nums = vec![3, 3, 3, 3];
    assert_eq!(Solution::max_coins(nums), 39);
}

#[test]
fn test_increasing_sequence() {
    // Test with an increasing sequence
    let nums = vec![1, 2, 3, 4, 5];
    assert_eq!(Solution::max_coins(nums), 110);
}

#[test]
fn test_decreasing_sequence() {
    // Test with a decreasing sequence
    let nums = vec![5, 4, 3, 2, 1];
    assert_eq!(Solution::max_coins(nums), 110);
}

#[test]
fn test_zeros_in_array() {
    // Test with zeros in the array
    let nums = vec![3, 0, 5, 0, 8];
    assert_eq!(Solution::max_coins(nums), 40);
}

#[test]
fn test_large_values() {
    // Test with large values
    let nums = vec![100, 50, 75, 25];
    assert_eq!(Solution::max_coins(nums), 137500);
}
