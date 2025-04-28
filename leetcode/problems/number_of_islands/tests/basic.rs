use number_of_islands::Solution;

#[test]
fn test_example_1() {
    // Input: grid = [
    //   ["1","1","1","1","0"],
    //   ["1","1","0","1","0"],
    //   ["1","1","0","0","0"],
    //   ["0","0","0","0","0"]
    // ]
    // Output: 1
    let grid = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0']
    ];
    
    assert_eq!(Solution::num_islands(grid), 1);
}

#[test]
fn test_example_2() {
    // Input: grid = [
    //   ["1","1","0","0","0"],
    //   ["1","1","0","0","0"],
    //   ["0","0","1","0","0"],
    //   ["0","0","0","1","1"]
    // ]
    // Output: 3
    let grid = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1']
    ];
    
    assert_eq!(Solution::num_islands(grid), 3);
}

#[test]
fn test_all_water() {
    // Input: grid = [
    //   ["0","0","0"],
    //   ["0","0","0"],
    //   ["0","0","0"]
    // ]
    // Output: 0
    let grid = vec![
        vec!['0', '0', '0'],
        vec!['0', '0', '0'],
        vec!['0', '0', '0']
    ];
    
    assert_eq!(Solution::num_islands(grid), 0);
}

#[test]
fn test_all_land() {
    // Input: grid = [
    //   ["1","1","1"],
    //   ["1","1","1"],
    //   ["1","1","1"]
    // ]
    // Output: 1
    let grid = vec![
        vec!['1', '1', '1'],
        vec!['1', '1', '1'],
        vec!['1', '1', '1']
    ];
    
    assert_eq!(Solution::num_islands(grid), 1);
}

#[test]
fn test_diagonal_islands() {
    // Input: grid = [
    //   ["1","0","1"],
    //   ["0","1","0"],
    //   ["1","0","1"]
    // ]
    // Output: 5 (diagonals don't count as connected)
    let grid = vec![
        vec!['1', '0', '1'],
        vec!['0', '1', '0'],
        vec!['1', '0', '1']
    ];
    
    assert_eq!(Solution::num_islands(grid), 5);
}
