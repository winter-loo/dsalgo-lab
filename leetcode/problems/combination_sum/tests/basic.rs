use combination_sum::Solution;
use std::collections::HashSet;

// Helper function to compare results regardless of order
fn compare_results(result: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) -> bool {
    assert_eq!(result.len(), expected.len());
    let result_set: HashSet<Vec<i32>> = result
        .into_iter()
        .map(|mut v| {
            v.sort(); // Sort each combination
            v
        })
        .collect();

    let expected_set: HashSet<Vec<i32>> = expected
        .into_iter()
        .map(|mut v| {
            v.sort(); // Sort each combination
            v
        })
        .collect();

    result_set == expected_set
}

#[test]
fn test_example_1() {
    // Input: candidates = [2,3,6,7], target = 7
    // Output: [[2,2,3],[7]]
    let candidates = vec![2, 3, 6, 7];
    let target = 7;
    let result = Solution::combination_sum(candidates, target);
    let expected = vec![vec![2, 2, 3], vec![7]];

    assert!(compare_results(result, expected));
}

#[test]
fn test_example_2() {
    // Input: candidates = [2,3,5], target = 8
    // Output: [[2,2,2,2],[2,3,3],[3,5]]
    let candidates = vec![2, 3, 5];
    let target = 8;
    let result = Solution::combination_sum(candidates, target);
    let expected = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];

    assert!(compare_results(result, expected));
}

#[test]
fn test_example_3() {
    // Input: candidates = [2], target = 1
    // Output: []
    let candidates = vec![2];
    let target = 1;
    let result = Solution::combination_sum(candidates, target);

    assert!(result.is_empty());
}

#[test]
fn test_single_candidate_exact_match() {
    // Input: candidates = [5], target = 5
    // Output: [[5]]
    let candidates = vec![5];
    let target = 5;
    let result = Solution::combination_sum(candidates, target);
    let expected = vec![vec![5]];

    assert!(compare_results(result, expected));
}
