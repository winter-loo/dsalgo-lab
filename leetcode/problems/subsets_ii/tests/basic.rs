use subsets_ii::Solution;
use std::collections::HashSet;

// Helper function to compare results regardless of order
fn compare_results(result: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) -> bool {
    assert_eq!(result.len(), expected.len());
    let result_set: HashSet<Vec<i32>> = result
        .into_iter()
        .map(|mut v| {
            v.sort(); // Sort each subset
            v
        })
        .collect();
    
    let expected_set: HashSet<Vec<i32>> = expected
        .into_iter()
        .map(|mut v| {
            v.sort(); // Sort each subset
            v
        })
        .collect();
    
    result_set == expected_set
}

#[test]
fn test_example_1() {
    // Input: nums = [1,2,2]
    // Output: [[],[1],[1,2],[1,2,2],[2],[2,2]]
    let nums = vec![1, 2, 2];
    let result = Solution::subsets_with_dup(nums);
    let expected = vec![
        vec![],
        vec![1],
        vec![2],
        vec![1, 2],
        vec![2, 2],
        vec![1, 2, 2]
    ];
    
    println!("result={result:?}");
    println!("expected={expected:?}");
    assert!(compare_results(result, expected));
}

#[test]
fn test_example_2() {
    // Input: nums = [0]
    // Output: [[],[0]]
    let nums = vec![0];
    let result = Solution::subsets_with_dup(nums);
    let expected = vec![vec![], vec![0]];
    
    assert!(compare_results(result, expected));
}

#[test]
fn test_multiple_duplicates() {
    // Input: nums = [1,2,2,2]
    // Output: [[],[1],[1,2],[1,2,2],[1,2,2,2],[2],[2,2],[2,2,2]]
    let nums = vec![1, 2, 2, 2];
    let result = Solution::subsets_with_dup(nums);
    let expected = vec![
        vec![],
        vec![1],
        vec![2],
        vec![1, 2],
        vec![2, 2],
        vec![1, 2, 2],
        vec![2, 2, 2],
        vec![1, 2, 2, 2]
    ];
    
    assert!(compare_results(result, expected));
}

#[test]
fn test_all_duplicates() {
    // Input: nums = [1,1,1]
    // Output: [[],[1],[1,1],[1,1,1]]
    let nums = vec![1, 1, 1];
    let result = Solution::subsets_with_dup(nums);
    let expected = vec![
        vec![],
        vec![1],
        vec![1, 1],
        vec![1, 1, 1]
    ];
    
    assert!(compare_results(result, expected));
}

#[test]
fn test_mixed_duplicates() {
    // Input: nums = [4,4,1,4]
    // Output: [[],[1],[1,4],[1,4,4],[1,4,4,4],[4],[4,4],[4,4,4]]
    let nums = vec![4, 4, 1, 4];
    let result = Solution::subsets_with_dup(nums);
    let expected = vec![
        vec![],
        vec![1],
        vec![4],
        vec![1, 4],
        vec![4, 4],
        vec![1, 4, 4],
        vec![4, 4, 4],
        vec![1, 4, 4, 4]
    ];
    
    assert!(compare_results(result, expected));
}
