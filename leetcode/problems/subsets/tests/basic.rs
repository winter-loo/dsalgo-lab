use std::collections::HashSet;
use subsets::Solution;

#[test]
fn test_example_1() {
    // Input: nums = [1,2,3]
    // Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
    let nums = vec![1, 2, 3];
    let result = Solution::subsets(nums);

    // Convert result to HashSet for easy comparison regardless of order
    let result_set: HashSet<Vec<i32>> = result.into_iter().collect();

    // Expected subsets
    let expected = vec![
        vec![],
        vec![1],
        vec![2],
        vec![3],
        vec![1, 2],
        vec![1, 3],
        vec![2, 3],
        vec![1, 2, 3],
    ];
    let expected_set: HashSet<Vec<i32>> = expected.into_iter().collect();

    assert_eq!(result_set, expected_set);
}

#[test]
fn test_example_2() {
    // Input: nums = [0]
    // Output: [[],[0]]
    let nums = vec![0];
    let result = Solution::subsets(nums);

    // Convert result to HashSet for easy comparison regardless of order
    let result_set: HashSet<Vec<i32>> = result.into_iter().collect();

    // Expected subsets
    let expected = vec![vec![], vec![0]];
    let expected_set: HashSet<Vec<i32>> = expected.into_iter().collect();

    assert_eq!(result_set, expected_set);
}

#[test]
fn test_empty_array() {
    // Input: nums = []
    // Output: [[]]
    let nums = vec![];
    let result = Solution::subsets(nums);

    assert_eq!(result, vec![vec![]]);
}

#[test]
fn test_larger_array() {
    // Input: nums = [1,2,3,4]
    let nums = vec![1, 2, 3, 4];
    let result = Solution::subsets(nums);

    // There should be 2^4 = 16 subsets
    assert_eq!(result.len(), 16);

    // Check that empty set is included
    assert!(result.contains(&vec![]));

    // Check that full set is included
    assert!(result.contains(&vec![1, 2, 3, 4]));
}
