use rotting_oranges::Solution;

#[test]
fn test_example_1() {
    // Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
    // Output: 4
    let grid = vec![
        vec![2, 1, 1],
        vec![1, 1, 0],
        vec![0, 1, 1]
    ];
    
    assert_eq!(Solution::oranges_rotting(grid), 4);
}

#[test]
fn test_example_2() {
    // Input: grid = [[2,1,1],[0,1,1],[1,0,1]]
    // Output: -1
    // Explanation: The orange in the bottom left corner (row 2, column 0) is never rotten, because rotting only happens 4-directionally.
    let grid = vec![
        vec![2, 1, 1],
        vec![0, 1, 1],
        vec![1, 0, 1]
    ];
    
    assert_eq!(Solution::oranges_rotting(grid), -1);
}

#[test]
fn test_example_3() {
    // Input: grid = [[0,2]]
    // Output: 0
    // Explanation: Since there are already no fresh oranges at minute 0, the answer is just 0.
    let grid = vec![vec![0, 2]];
    
    assert_eq!(Solution::oranges_rotting(grid), 0);
}

#[test]
fn test_no_oranges() {
    // Input: grid = [[0,0],[0,0]]
    // Output: 0
    // No fresh oranges, so the answer is 0
    let grid = vec![
        vec![0, 0],
        vec![0, 0]
    ];
    
    assert_eq!(Solution::oranges_rotting(grid), 0);
}

#[test]
fn test_all_fresh_oranges() {
    // Input: grid = [[1,1],[1,1]]
    // Output: -1
    // All oranges are fresh, but there are no rotten oranges to start the process
    let grid = vec![
        vec![1, 1],
        vec![1, 1]
    ];
    
    assert_eq!(Solution::oranges_rotting(grid), -1);
}

#[test]
fn test_multiple_rotten_oranges() {
    // Input: grid = [[2,1,1],[1,1,1],[1,1,2]]
    // Output: 2
    // Two rotten oranges at opposite corners
    let grid = vec![
        vec![2, 1, 1],
        vec![1, 1, 1],
        vec![1, 1, 2]
    ];
    
    assert_eq!(Solution::oranges_rotting(grid), 2);
}

#[test]
fn test_all_rotten_oranges() {
    // Input: grid = [[2,2],[2,2]]
    // Output: 0
    // All oranges are already rotten
    let grid = vec![
        vec![2, 2],
        vec![2, 2]
    ];
    
    assert_eq!(Solution::oranges_rotting(grid), 0);
}
