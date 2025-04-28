use max_area_of_island::Solution;

#[test]
fn test_example_1() {
    // Input: grid = [
    //   [0,0,1,0,0,0,0,1,0,0,0,0,0],
    //   [0,0,0,0,0,0,0,1,1,1,0,0,0],
    //   [0,1,1,0,1,0,0,0,0,0,0,0,0],
    //   [0,1,0,0,1,1,0,0,1,0,1,0,0],
    //   [0,1,0,0,1,1,0,0,1,1,1,0,0],
    //   [0,0,0,0,0,0,0,0,0,0,1,0,0],
    //   [0,0,0,0,0,0,0,1,1,1,0,0,0],
    //   [0,0,0,0,0,0,0,1,1,0,0,0,0]
    // ]
    // Output: 6
    let grid = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
    ];
    
    assert_eq!(Solution::max_area_of_island(grid), 6);
}

#[test]
fn test_example_2() {
    // Input: grid = [[0,0,0,0,0,0,0,0]]
    // Output: 0
    let grid = vec![vec![0, 0, 0, 0, 0, 0, 0, 0]];
    
    assert_eq!(Solution::max_area_of_island(grid), 0);
}

#[test]
fn test_all_water() {
    // Input: grid = [
    //   [0,0,0],
    //   [0,0,0],
    //   [0,0,0]
    // ]
    // Output: 0
    let grid = vec![
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![0, 0, 0]
    ];
    
    assert_eq!(Solution::max_area_of_island(grid), 0);
}

#[test]
fn test_all_land() {
    // Input: grid = [
    //   [1,1,1],
    //   [1,1,1],
    //   [1,1,1]
    // ]
    // Output: 9
    let grid = vec![
        vec![1, 1, 1],
        vec![1, 1, 1],
        vec![1, 1, 1]
    ];
    
    assert_eq!(Solution::max_area_of_island(grid), 9);
}

#[test]
fn test_multiple_islands() {
    // Input: grid = [
    //   [1,1,0,0,0],
    //   [1,1,0,0,0],
    //   [0,0,0,1,1],
    //   [0,0,0,1,1]
    // ]
    // Output: 4
    let grid = vec![
        vec![1, 1, 0, 0, 0],
        vec![1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 1],
        vec![0, 0, 0, 1, 1]
    ];
    
    assert_eq!(Solution::max_area_of_island(grid), 4);
}
