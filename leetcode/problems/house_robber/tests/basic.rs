use house_robber::Solution;

#[test]
fn test_example_1() {
    // Input: nums = [1,2,3,1]
    // Output: 4
    let nums = vec![1, 2, 3, 1];
    assert_eq!(Solution::rob(nums), 4);
}

#[test]
fn test_example_2() {
    // Input: nums = [2,7,9,3,1]
    // Output: 12
    let nums = vec![2, 7, 9, 3, 1];
    assert_eq!(Solution::rob(nums), 12);
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
fn test_all_same_value() {
    // All houses have the same value, should rob alternate houses
    let nums = vec![5, 5, 5, 5, 5];
    assert_eq!(Solution::rob(nums), 15);
}

#[test]
fn test_alternating_values() {
    // Alternating high and low values
    let nums = vec![2, 1, 2, 1, 2];
    assert_eq!(Solution::rob(nums), 6);
}
