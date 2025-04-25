use n_queens::Solution;
use std::collections::HashSet;

// Helper function to convert a solution to a canonical form for comparison
fn canonicalize(board: Vec<String>) -> Vec<String> {
    let mut canonical = board.clone();
    canonical.sort();
    canonical
}

// Helper function to compare results regardless of order
fn compare_results(result: Vec<Vec<String>>, expected: Vec<Vec<String>>) -> bool {
    if result.len() != expected.len() {
        return false;
    }

    let result_set: HashSet<Vec<String>> = result.into_iter().map(canonicalize).collect();

    let expected_set: HashSet<Vec<String>> = expected.into_iter().map(canonicalize).collect();

    result_set == expected_set
}

#[test]
fn test_example_1() {
    // Input: n = 4
    // Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
    let n = 4;
    let result = Solution::solve_n_queens(n);

    let expected = vec![
        vec![
            ".Q..".to_string(),
            "...Q".to_string(),
            "Q...".to_string(),
            "..Q.".to_string(),
        ],
        vec![
            "..Q.".to_string(),
            "Q...".to_string(),
            "...Q".to_string(),
            ".Q..".to_string(),
        ],
    ];

    assert!(compare_results(result, expected));
}

#[test]
fn test_example_2() {
    // Input: n = 1
    // Output: [["Q"]]
    let n = 1;
    let result = Solution::solve_n_queens(n);

    let expected = vec![vec!["Q".to_string()]];

    assert!(compare_results(result, expected));
}

#[test]
fn test_n_equals_2() {
    // Input: n = 2
    // Output: [] (no solution exists)
    let n = 2;
    let result = Solution::solve_n_queens(n);

    assert!(result.is_empty());
}

#[test]
fn test_n_equals_3() {
    // Input: n = 3
    // Output: [] (no solution exists)
    let n = 3;
    let result = Solution::solve_n_queens(n);

    assert!(result.is_empty());
}

#[test]
fn test_solution_count() {
    // For n = 5, there should be 10 solutions
    let n = 5;
    let result = Solution::solve_n_queens(n);

    assert_eq!(result.len(), 10);

    // For n = 6, there should be 4 solutions
    let n = 6;
    let result = Solution::solve_n_queens(n);

    assert_eq!(result.len(), 4);
}
