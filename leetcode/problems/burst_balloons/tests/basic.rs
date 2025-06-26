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
    assert_eq!(Solution::max_coins(nums), 66);
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
    assert_eq!(Solution::max_coins(nums), 152);
}

#[test]
fn test_large_values() {
    // Test with large values
    let nums = vec![100, 50, 75, 25];
    assert_eq!(Solution::max_coins(nums), 565100);
}

#[test]
fn test_example_3() {
    // Test with an increasing sequence
    let nums = vec![1, 3, 4, 5];
    assert_eq!(Solution::max_coins(nums), 85);

    // 1 3 4 5
    // 1 * 1 * 3, 3 4 5 => 3 + 80 = 83
    // 1 * 3 * 4, 1 4 5 => 12 + 30 = 42
    // 3 * 4 * 5, 1 3 5 => 60 + 25 = 85 ✓
    // 4 * 5 * 1, 1 3 4 => 20 + 20 = 40

    // 3 4 5
    // 1 * 3 * 4, 4 5 => 12 + 25 = 37
    // 3 * 4 * 5, 3 5 => 60 + 20 = 80 ✓
    // 4 * 5 * 1, 3 4 => 20 + 16 = 36

    // 1 4 5
    // 1 * 1 * 4, 4 5 => 4 + 25 = 29
    // 1 * 4 * 5, 1 5 => 20 + 10 = 30 ✓
    // 4 * 5 * 1, 1 4 => 20 + 8 = 28

    // 1 3 5
    // 1 * 1 * 3, 3 5 => 3 + 20 = 23
    // 1 * 3 * 5, 1 5 => 15 + 10 = 25 ✓
    // 3 * 5 * 1, 1 3 => 15 + 6 = 21

    // 1 3 4
    // 1 * 1 * 3, 3 4 => 3 + 14 = 17
    // 1 * 3 * 4, 1 4 => 12 + 8 = 20 ✓
    // 3 * 4 * 1, 1 3 => 12 + 6 = 18
}

#[test]
// #[ignore]
fn test_many_values_1() {
    let nums = vec![7, 9, 8, 0, 7, 1, 3, 5, 5, 2, 3];
    assert_eq!(Solution::max_coins(nums), 1654);
}

#[test]
// #[ignore]
fn test_many_values_2() {
    let nums = vec![8, 2, 6, 8, 9, 8, 1, 4, 1, 5, 3, 0, 7, 7, 0, 4, 2, 2, 5, 5];
    assert_eq!(Solution::max_coins(nums), 3830);
}
