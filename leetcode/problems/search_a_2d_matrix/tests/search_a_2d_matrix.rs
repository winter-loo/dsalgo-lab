use search_a_2d_matrix::Solution;

#[test]
fn test_example_1() {
    let matrix = vec![
        vec![1, 3, 5, 7],
        vec![10, 11, 16, 20],
        vec![23, 30, 34, 60],
    ];
    let target = 3;
    assert_eq!(Solution::search_matrix(matrix, target), true);
}

#[test]
fn test_example_2() {
    let matrix = vec![
        vec![1, 3, 5, 7],
        vec![10, 11, 16, 20],
        vec![23, 30, 34, 60],
    ];
    let target = 13;
    assert_eq!(Solution::search_matrix(matrix, target), false);
}

#[test]
fn test_target_in_first_row() {
    let matrix = vec![
        vec![1, 3, 5, 7],
        vec![10, 11, 16, 20],
        vec![23, 30, 34, 60],
    ];
    let target = 1;
    assert_eq!(Solution::search_matrix(matrix, target), true);
}

#[test]
fn test_target_in_last_row() {
    let matrix = vec![
        vec![1, 3, 5, 7],
        vec![10, 11, 16, 20],
        vec![23, 30, 34, 60],
    ];
    let target = 60;
    assert_eq!(Solution::search_matrix(matrix, target), true);
}

#[test]
fn test_target_smaller_than_all() {
    let matrix = vec![
        vec![1, 3, 5, 7],
        vec![10, 11, 16, 20],
        vec![23, 30, 34, 60],
    ];
    let target = 0;
    assert_eq!(Solution::search_matrix(matrix, target), false);
}

#[test]
fn test_target_larger_than_all() {
    let matrix = vec![
        vec![1, 3, 5, 7],
        vec![10, 11, 16, 20],
        vec![23, 30, 34, 60],
    ];
    let target = 61;
    assert_eq!(Solution::search_matrix(matrix, target), false);
}

#[test]
fn test_single_element_matrix() {
    let matrix = vec![vec![1]];
    let target = 1;
    assert_eq!(Solution::search_matrix(matrix, target), true);
}

#[test]
fn test_single_row_matrix() {
    let matrix = vec![vec![1, 3, 5, 7]];
    let target = 5;
    assert_eq!(Solution::search_matrix(matrix, target), true);
}

#[test]
fn test_single_column_matrix() {
    let matrix = vec![vec![1], vec![3], vec![5], vec![7]];
    let target = 3;
    assert_eq!(Solution::search_matrix(matrix, target), true);
}
