use k_closest_points_to_origin::Solution;

#[test]
fn test_example_1() {
    // Input: points = [[1,3],[-2,2]], k = 1
    // Output: [[-2,2]]
    let points = vec![vec![1, 3], vec![-2, 2]];
    let k = 1;
    let result = Solution::k_closest(points, k);
    assert_eq!(result.len(), 1);
    assert!(result.contains(&vec![-2, 2]));
}

#[test]
fn test_example_2() {
    // Input: points = [[3,3],[5,-1],[-2,4]], k = 2
    // Output: [[3,3],[-2,4]]
    let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
    let k = 2;
    let result = Solution::k_closest(points, k);
    assert_eq!(result.len(), 2);
    assert!(result.contains(&vec![3, 3]));
    assert!(result.contains(&vec![-2, 4]));
}

#[test]
fn test_single_point() {
    // Input: points = [[1,1]], k = 1
    // Output: [[1,1]]
    let points = vec![vec![1, 1]];
    let k = 1;
    let result = Solution::k_closest(points, k);
    assert_eq!(result, vec![vec![1, 1]]);
}

#[test]
fn test_all_points() {
    // Input: points = [[1,1],[2,2],[3,3]], k = 3
    // Output: [[1,1],[2,2],[3,3]]
    let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    let k = 3;
    let result = Solution::k_closest(points, k);
    assert_eq!(result.len(), 3);
    assert!(result.contains(&vec![1, 1]));
    assert!(result.contains(&vec![2, 2]));
    assert!(result.contains(&vec![3, 3]));
}

#[test]
fn test_negative_coordinates() {
    // Input: points = [[-5,-5],[-4,-4],[-3,-3],[-2,-2],[-1,-1]], k = 3
    // Output: [[-1,-1],[-2,-2],[-3,-3]]
    let points = vec![
        vec![-5, -5],
        vec![-4, -4],
        vec![-3, -3],
        vec![-2, -2],
        vec![-1, -1],
    ];
    let k = 3;
    let result = Solution::k_closest(points, k);
    assert_eq!(result.len(), 3);
    assert!(result.contains(&vec![-1, -1]));
    assert!(result.contains(&vec![-2, -2]));
    assert!(result.contains(&vec![-3, -3]));
}
