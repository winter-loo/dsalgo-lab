use pacific_atlantic_water_flow::Solution;

#[test]
fn test_example_1() {
    // Input: heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
    // Output: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
    let heights = vec![
        vec![1, 2, 2, 3, 5],
        vec![3, 2, 3, 4, 4],
        vec![2, 4, 5, 3, 1],
        vec![6, 7, 1, 4, 5],
        vec![5, 1, 1, 2, 4],
    ];

    let mut result = Solution::pacific_atlantic(heights);
    result.sort(); // Sort for consistent comparison

    let mut expected = vec![
        vec![0, 4],
        vec![1, 3],
        vec![1, 4],
        vec![2, 2],
        vec![3, 0],
        vec![3, 1],
        vec![4, 0],
    ];
    expected.sort(); // Sort for consistent comparison

    assert_eq!(result, expected);
}

#[test]
fn test_example_2() {
    // Input: heights = [[2,1],[1,2]]
    // Output: [[0,0],[0,1],[1,0],[1,1]]
    let heights = vec![vec![2, 1], vec![1, 2]];

    let mut result = Solution::pacific_atlantic(heights);
    result.sort(); // Sort for consistent comparison

    let mut expected = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]];
    expected.sort(); // Sort for consistent comparison

    assert_eq!(result, expected);
}

#[test]
fn test_single_cell() {
    // Input: heights = [[1]]
    // Output: [[0,0]]
    let heights = vec![vec![1]];

    let result = Solution::pacific_atlantic(heights);

    let expected = vec![vec![0, 0]];

    assert_eq!(result, expected);
}

#[test]
fn test_increasing_heights() {
    // Input: heights = [[1,2,3],[4,5,6],[7,8,9]]
    // Output: [[0,0],[0,1],[0,2],[1,0],[1,1],[1,2],[2,0],[2,1],[2,2]]
    // All cells can reach both oceans
    #[rustfmt::skip]
    let heights = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];

    let mut result = Solution::pacific_atlantic(heights);
    result.sort(); // Sort for consistent comparison

    let mut expected = vec![
        vec![0, 2],
        vec![1, 2],
        vec![2, 0],
        vec![2, 1],
        vec![2, 2],
    ];
    expected.sort(); // Sort for consistent comparison

    assert_eq!(result, expected);
}

#[test]
fn test_decreasing_heights() {
    // Input: heights = [[9,8,7],[6,5,4],[3,2,1]]
    // Output: [[0,0],[0,1],[0,2],[1,0],[1,1],[1,2],[2,0],[2,1],[2,2]]
    // All cells can reach both oceans
    #[rustfmt::skip]
    let heights = vec![
        vec![9, 8, 7],
        vec![6, 5, 4],
        vec![3, 2, 1]
    ];

    let mut result = Solution::pacific_atlantic(heights);
    result.sort(); // Sort for consistent comparison

    let mut expected = vec![
        vec![0, 0],
        vec![0, 1],
        vec![0, 2],
        vec![1, 0],
        vec![2, 0],
    ];
    expected.sort(); // Sort for consistent comparison

    assert_eq!(result, expected);
}

#[test]
fn test_peak_in_middle() {
    // Input: heights = [[1,2,1],[2,3,2],[1,2,1]]
    #[rustfmt::skip]
    let heights = vec![
        vec![1, 2, 1],
        vec![2, 3, 2],
        vec![1, 2, 1]
    ];

    let result = Solution::pacific_atlantic(heights);

    let expected = vec![
        vec![0, 1],
        vec![0, 2],
        vec![1, 0],
        vec![1, 1],
        vec![1, 2],
        vec![2, 0],
        vec![2, 1],
    ];

    assert_eq!(result, expected);
}

#[test]
fn test_example_3() {
    #[rustfmt::skip]
    let heights = vec![
        vec![1, 1],
        vec![1, 1],
        vec![1, 1]
    ];

    let result = Solution::pacific_atlantic(heights);

    let expected = vec![
        vec![0, 0],
        vec![0, 1],
        vec![1, 0],
        vec![1, 1],
        vec![2, 0],
        vec![2, 1],
    ];

    assert_eq!(result, expected);
}
