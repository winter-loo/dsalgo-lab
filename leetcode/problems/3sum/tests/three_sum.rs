use three_sum::Solution;

#[test]
fn test_example_1() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    // [-4, -1, -1, 0, 1, 2]
    // -1 -1 2
    // -1 0 1
    let mut result = Solution::three_sum(nums);
    result.sort_unstable();

    // Expected result: [[-1,-1,2],[-1,0,1]]
    let mut expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
    expected.sort_unstable();

    // Convert to HashSet for unordered comparison
    assert_eq!(result, expected);
}

#[test]
fn test_example_2() {
    let nums = vec![0, 1, 1];
    let result = Solution::three_sum(nums);

    // Expected result: []
    let expected: Vec<Vec<i32>> = vec![];

    assert_eq!(result, expected);
}

#[test]
fn test_example_3() {
    let nums = vec![0, 0, 0];
    let result = Solution::three_sum(nums);

    // Expected result: [[0,0,0]]
    let expected = vec![vec![0, 0, 0]];

    assert_eq!(result, expected);
}

#[test]
fn test_empty_array() {
    let nums = vec![];
    let result = Solution::three_sum(nums);

    // Expected result: []
    let expected: Vec<Vec<i32>> = vec![];

    assert_eq!(result, expected);
}

#[test]
fn test_array_with_duplicates() {
    let nums = vec![-2, 0, 0, 2, 2];
    let result = Solution::three_sum(nums);

    // Expected result: [[-2,0,2]]
    let expected = vec![vec![-2, 0, 2]];

    // Convert to HashSet for unordered comparison
    assert_eq!(result, expected);
}

#[test]
fn test_larger_array() {
    let nums = vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4];
    let mut result = Solution::three_sum(nums);
    result.sort_unstable();

    // Expected triplets: [[-4,0,4],[-4,1,3],[-3,0,3],[-3,-1,4],[-2,-1,3],[-2,0,2],[-1,-1,2],[-1,0,1]]
    let mut expected = vec![
        vec![-4, 0, 4],
        vec![-4, 1, 3],
        vec![-3, 0, 3],
        vec![-3, -1, 4],
        vec![-3, 1, 2],
        vec![-2, -1, 3],
        vec![-2, 0, 2],
        vec![-1, -1, 2],
        vec![-1, 0, 1],
    ];

    expected.sort_unstable();

    // Convert to HashSet for unordered comparison
    assert_eq!(result, expected);
}
