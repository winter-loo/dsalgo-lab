use last_stone_weight::Solution;

#[test]
fn test_example_1() {
    // Input: stones = [2,7,4,1,8,1]
    // Output: 1
    let stones = vec![2, 7, 4, 1, 8, 1];
    assert_eq!(Solution::last_stone_weight(stones), 1);
}

#[test]
fn test_example_2() {
    // Input: stones = [1]
    // Output: 1
    let stones = vec![1];
    assert_eq!(Solution::last_stone_weight(stones), 1);
}

#[test]
fn test_all_same_weights() {
    // Input: stones = [2,2,2,2]
    // Output: 0
    let stones = vec![2, 2, 2, 2];
    assert_eq!(Solution::last_stone_weight(stones), 0);
}

#[test]
fn test_descending_weights() {
    // Input: stones = [5,4,3,2,1]
    // Output: 3
    let stones = vec![5, 4, 3, 2, 1];
    // 5, 4 => 1 => [3, 2, 1, 1]
    // 3, 2 => 1 => [1, 1, 1]
    assert_eq!(Solution::last_stone_weight(stones), 1);
}

#[test]
fn test_two_stones() {
    // Input: stones = [10, 5]
    // Output: 5
    let stones = vec![10, 5];
    assert_eq!(Solution::last_stone_weight(stones), 5);
}
