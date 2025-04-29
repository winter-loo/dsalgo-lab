use min_cost_to_connect_all_points::Solution;

#[test]
fn test_example_1() {
    // Input: points = [[0,0],[2,2],[3,10],[5,2],[7,0]]
    // Output: 20
    let points = vec![
        vec![0, 0],
        vec![2, 2],
        vec![3, 10],
        vec![5, 2],
        vec![7, 0],
    ];
    
    assert_eq!(Solution::min_cost_connect_points(points), 20);
}

#[test]
fn test_example_2() {
    // Input: points = [[3,12],[-2,5],[-4,1]]
    // Output: 18
    let points = vec![
        vec![3, 12],
        vec![-2, 5],
        vec![-4, 1],
    ];
    
    assert_eq!(Solution::min_cost_connect_points(points), 18);
}

#[test]
fn test_single_point() {
    // Input: points = [[0,0]]
    // Output: 0
    let points = vec![vec![0, 0]];
    
    assert_eq!(Solution::min_cost_connect_points(points), 0);
}

#[test]
fn test_two_points() {
    // Input: points = [[0,0],[1,1]]
    // Output: 2
    let points = vec![vec![0, 0], vec![1, 1]];
    
    assert_eq!(Solution::min_cost_connect_points(points), 2);
}

#[test]
fn test_points_on_line() {
    // Input: points = [[0,0],[1,0],[2,0],[3,0],[4,0]]
    // Output: 4
    let points = vec![
        vec![0, 0],
        vec![1, 0],
        vec![2, 0],
        vec![3, 0],
        vec![4, 0],
    ];
    
    assert_eq!(Solution::min_cost_connect_points(points), 4);
}
