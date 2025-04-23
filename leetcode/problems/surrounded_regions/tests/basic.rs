use surrounded_regions::Solution;

#[test]
fn test_example_1() {
    // Input: board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
    // Output: [["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]
    let mut board = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X']
    ];
    
    let expected = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'X', 'X']
    ];
    
    Solution::solve(&mut board);
    assert_eq!(board, expected);
}

#[test]
fn test_example_2() {
    // Input: board = [["X"]]
    // Output: [["X"]]
    let mut board = vec![vec!['X']];
    
    let expected = vec![vec!['X']];
    
    Solution::solve(&mut board);
    assert_eq!(board, expected);
}

#[test]
fn test_all_x() {
    // Input: board = [["X","X"],["X","X"]]
    // Output: [["X","X"],["X","X"]]
    let mut board = vec![
        vec!['X', 'X'],
        vec!['X', 'X']
    ];
    
    let expected = vec![
        vec!['X', 'X'],
        vec!['X', 'X']
    ];
    
    Solution::solve(&mut board);
    assert_eq!(board, expected);
}

#[test]
fn test_all_o() {
    // Input: board = [["O","O"],["O","O"]]
    // Output: [["O","O"],["O","O"]]
    // All 'O's are on the border, so none are flipped
    let mut board = vec![
        vec!['O', 'O'],
        vec!['O', 'O']
    ];
    
    let expected = vec![
        vec!['O', 'O'],
        vec!['O', 'O']
    ];
    
    Solution::solve(&mut board);
    assert_eq!(board, expected);
}

#[test]
fn test_o_in_center() {
    // Input: board = [["X","X","X"],["X","O","X"],["X","X","X"]]
    // Output: [["X","X","X"],["X","X","X"],["X","X","X"]]
    // The 'O' in the center is surrounded, so it is flipped
    let mut board = vec![
        vec!['X', 'X', 'X'],
        vec!['X', 'O', 'X'],
        vec!['X', 'X', 'X']
    ];
    
    let expected = vec![
        vec!['X', 'X', 'X'],
        vec!['X', 'X', 'X'],
        vec!['X', 'X', 'X']
    ];
    
    Solution::solve(&mut board);
    assert_eq!(board, expected);
}

#[test]
fn test_o_connected_to_border() {
    // Input: board = [["X","X","X"],["X","O","X"],["X","O","X"]]
    // Output: [["X","X","X"],["X","O","X"],["X","O","X"]]
    // The 'O's are connected to the border, so they are not flipped
    let mut board = vec![
        vec!['X', 'X', 'X'],
        vec!['X', 'O', 'X'],
        vec!['X', 'O', 'X']
    ];
    
    let expected = vec![
        vec!['X', 'X', 'X'],
        vec!['X', 'O', 'X'],
        vec!['X', 'O', 'X']
    ];
    
    Solution::solve(&mut board);
    assert_eq!(board, expected);
}
