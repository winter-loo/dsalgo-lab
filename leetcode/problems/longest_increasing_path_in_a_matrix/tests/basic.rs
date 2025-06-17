use longest_increasing_path_in_a_matrix::Solution;

#[test]
fn test_example_1() {
    // Input: matrix = [[9,9,4],[6,6,8],[2,1,1]]
    // Output: 4
    // Explanation: The longest increasing path is [1, 2, 6, 9].
    // [9, 9, 4]
    // [6, 6, 8]
    // [2, 1, 1]
    let matrix = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
    assert_eq!(Solution::longest_increasing_path(matrix), 4);
}

#[test]
fn test_example_2() {
    // Input: matrix = [[3,4,5],[3,2,6],[2,2,1]]
    // Output: 4
    // Explanation: The longest increasing path is [3, 4, 5, 6]. Moving diagonally is not allowed.
    let matrix = vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]];
    assert_eq!(Solution::longest_increasing_path(matrix), 4);
}

#[test]
fn test_example_3() {
    // Input: matrix = [[1]]
    // Output: 1
    let matrix = vec![vec![1]];
    assert_eq!(Solution::longest_increasing_path(matrix), 1);
}

#[test]
fn test_decreasing_matrix() {
    // Decreasing matrix
    // [9, 8, 7]
    // [6, 5, 4]
    // [3, 2, 1]
    let matrix = vec![vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1]];
    assert_eq!(Solution::longest_increasing_path(matrix), 5);
}

#[test]
fn test_increasing_matrix() {
    // Increasing matrix
    // [1,2,3]
    // [4,5,6]
    // [7,8,9]
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    assert_eq!(Solution::longest_increasing_path(matrix), 5);
}

#[test]
fn test_spiral_matrix() {
    // Spiral matrix
    let matrix = vec![
        vec![1, 2, 3, 4],
        vec![12, 13, 14, 5],
        vec![11, 16, 15, 6],
        vec![10, 9, 8, 7],
    ];
    assert_eq!(Solution::longest_increasing_path(matrix), 16);
}

#[test]
fn test_example_4() {
    let matrix = vec![vec![1, 2, 3], vec![6, 5, 4], vec![7, 8, 9]];
    assert_eq!(Solution::longest_increasing_path(matrix), 9);
}

#[test]
fn test_example_5() {
    // [1,4,7,9]
    // [0,3,8,5]
    // [3,6,0,6]
    // [1,4,5,6]
    let matrix = vec![vec![1,4,7,9], vec![0,3,8,5], vec![3,6,0,6], vec![1,4,5,6]];
    assert_eq!(Solution::longest_increasing_path(matrix), 5);
}
