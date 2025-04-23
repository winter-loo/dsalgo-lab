use permutations::Solution;
use std::collections::HashSet;

#[test]
fn test_example_1() {
    // Input: nums = [1,2,3]
    // Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
    let nums = vec![1, 2, 3];
    let result = Solution::permute(nums.clone());
    
    // Check that all permutations are present
    assert_eq!(result.len(), 6); // 3! = 6 permutations
    
    // Convert result to HashSet for easy comparison
    let result_set: HashSet<Vec<i32>> = result.into_iter().collect();
    
    // Check that all permutations are unique
    assert_eq!(result_set.len(), 6);
    
    // Check that all expected permutations are present
    assert!(result_set.contains(&vec![1, 2, 3]));
    assert!(result_set.contains(&vec![1, 3, 2]));
    assert!(result_set.contains(&vec![2, 1, 3]));
    assert!(result_set.contains(&vec![2, 3, 1]));
    assert!(result_set.contains(&vec![3, 1, 2]));
    assert!(result_set.contains(&vec![3, 2, 1]));
}

#[test]
fn test_example_2() {
    // Input: nums = [0,1]
    // Output: [[0,1],[1,0]]
    let nums = vec![0, 1];
    let result = Solution::permute(nums.clone());
    
    // Check that all permutations are present
    assert_eq!(result.len(), 2); // 2! = 2 permutations
    
    // Convert result to HashSet for easy comparison
    let result_set: HashSet<Vec<i32>> = result.into_iter().collect();
    
    // Check that all permutations are unique
    assert_eq!(result_set.len(), 2);
    
    // Check that all expected permutations are present
    assert!(result_set.contains(&vec![0, 1]));
    assert!(result_set.contains(&vec![1, 0]));
}

#[test]
fn test_example_3() {
    // Input: nums = [1]
    // Output: [[1]]
    let nums = vec![1];
    let result = Solution::permute(nums.clone());
    
    // Check that all permutations are present
    assert_eq!(result.len(), 1); // 1! = 1 permutation
    
    // Check that the expected permutation is present
    assert_eq!(result[0], vec![1]);
}

#[test]
fn test_empty_array() {
    // Input: nums = []
    // Output: [[]]
    let nums = vec![];
    let result = Solution::permute(nums.clone());
    
    // Check that there is exactly one permutation (the empty permutation)
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], vec![]);
}

#[test]
fn test_larger_array() {
    // Input: nums = [1,2,3,4]
    // Output: 24 different permutations (4!)
    let nums = vec![1, 2, 3, 4];
    let result = Solution::permute(nums.clone());
    
    // Check that all permutations are present
    assert_eq!(result.len(), 24); // 4! = 24 permutations
    
    // Convert result to HashSet for easy comparison
    let result_set: HashSet<Vec<i32>> = result.into_iter().collect();
    
    // Check that all permutations are unique
    assert_eq!(result_set.len(), 24);
}
