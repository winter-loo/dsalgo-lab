use house_robber_ii::Solution;

#[test]
fn test_example_1() {
    // Input: nums = [2,3,2]
    // Output: 3
    let nums = vec![2, 3, 2];
    assert_eq!(Solution::rob(nums), 3);
}

#[test]
fn test_example_2() {
    // Input: nums = [1,2,3,1]
    // Output: 4
    let nums = vec![1, 2, 3, 1];
    assert_eq!(Solution::rob(nums), 4);
}

#[test]
fn test_example_3() {
    // Input: nums = [1,2,3]
    // Output: 3
    let nums = vec![1, 2, 3];
    assert_eq!(Solution::rob(nums), 3);
}

#[test]
fn test_single_house() {
    // Only one house to rob
    let nums = vec![5];
    assert_eq!(Solution::rob(nums), 5);
}

#[test]
fn test_two_houses() {
    // Two houses, should rob the one with more money
    let nums = vec![3, 7];
    assert_eq!(Solution::rob(nums), 7);
}

#[test]
fn test_circular_arrangement() {
    // Test the circular arrangement constraint
    let nums = vec![1, 3, 1, 3, 100];
    assert_eq!(Solution::rob(nums), 103);
}
