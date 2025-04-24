use combination_sum_ii::Solution;
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
    // Input: candidates = [10,1,2,7,6,1,5], target = 8
    // Output: [[1,1,6],[1,2,5],[1,7],[2,6]]
    let candidates = vec![10, 1, 2, 7, 6, 1, 5];
    let target = 8;
    let result = Solution::combination_sum2(candidates, target);
    let expected = vec![
        vec![1, 1, 6],
        vec![1, 2, 5],
        vec![1, 7],
        vec![2, 6]
    ];
    println!("result:{result:?}");
    println!("expected:{expected:?}");
    
    assert!(compare_results(result, expected));
}

#[test]
fn test_example_2() {
    // Input: candidates = [2,5,2,1,2], target = 5
    // Output: [[1,2,2],[5]]
    let candidates = vec![2, 5, 2, 1, 2];
    let target = 5;
    let result = Solution::combination_sum2(candidates, target);
    let expected = vec![
        vec![1, 2, 2],
        vec![5]
    ];
    
    assert!(compare_results(result, expected));
}

#[test]
fn test_no_solution() {
    // Input: candidates = [1,2,3], target = 10
    // Output: []
    let candidates = vec![1, 2, 3];
    let target = 10;
    let result = Solution::combination_sum2(candidates, target);
    
    assert!(result.is_empty());
}

#[test]
fn test_single_candidate_exact_match() {
    // Input: candidates = [5], target = 5
    // Output: [[5]]
    let candidates = vec![5];
    let target = 5;
    let result = Solution::combination_sum2(candidates, target);
    let expected = vec![vec![5]];
    
    assert!(compare_results(result, expected));
}

#[test]
fn test_many_duplicates() {
    // Input: candidates = [1,1,1,1,1,1,1], target = 3
    // Output: [[1,1,1]]
    let candidates = vec![1, 1, 1, 1, 1, 1, 1];
    let target = 3;
    let result = Solution::combination_sum2(candidates, target);
    let expected = vec![vec![1, 1, 1]];
    
    assert!(compare_results(result, expected));
}

#[test]
fn test_example_3() {
    let candidates = vec![1; 100];
    let target = 30;
    let result = Solution::combination_sum2_iterate_all(candidates, target);
    println!("result:{result:?}");
    
    // assert!(compare_results(result, expected));
}