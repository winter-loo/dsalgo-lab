use word_search::Solution;

#[test]
fn test_example_1() {
    // Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
    // Output: true
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = "ABCCED".to_string();
    assert!(Solution::exist(board, word));
}

#[test]
fn test_example_2() {
    // Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
    // Output: true
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = "SEE".to_string();
    assert!(Solution::exist(board, word));
}

#[test]
fn test_example_3() {
    // Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
    // Output: false
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = "ABCB".to_string();
    assert!(!Solution::exist(board, word));
}

#[test]
fn test_single_cell() {
    // Input: board = [["A"]], word = "A"
    // Output: true
    let board = vec![vec!['A']];
    let word = "A".to_string();
    assert!(Solution::exist(board, word));
}

#[test]
fn test_word_too_long() {
    // Input: board = [["A","B"],["C","D"]], word = "ABCDE"
    // Output: false
    let board = vec![vec!['A', 'B'], vec!['C', 'D']];
    let word = "ABCDE".to_string();
    assert!(!Solution::exist(board, word));
}

#[test]
fn test_zigzag_path() {
    // Input: board = [["A","B","C"],["D","E","F"],["G","H","I"]], word = "ABEHI"
    // Output: true
    let board = vec![
        vec!['A', 'B', 'C'],
        vec!['D', 'E', 'F'],
        vec!['G', 'H', 'I'],
    ];
    let word = "ABEHI".to_string();
    assert!(Solution::exist(board, word));
}

#[test]
fn test_example_4() {
    // Input: board = [["A"]], word = "A"
    // Output: true
    let board = vec![vec!['A']];
    let word = "AB".to_string();
    assert!(!Solution::exist(board, word));
}

#[test]
fn test_example_5() {
    let board = vec![
        vec!['a', 'a', 'b', 'a', 'a', 'b'],
        vec!['a', 'a', 'b', 'b', 'b', 'a'],
        vec!['a', 'a', 'a', 'a', 'b', 'a'],
        vec!['b', 'a', 'b', 'b', 'a', 'b'],
        vec!['a', 'b', 'b', 'a', 'b', 'a'],
        vec!['b', 'a', 'a', 'a', 'a', 'b'],
    ];
    let word = "bbbaabbbbbab".to_string();
    assert!(!Solution::exist(board, word));
}

#[test]
fn test_example_6() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'E', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = "ABCESEEEFS".to_string();
    assert!(Solution::exist(board, word));
}

#[test]
fn test_example_7() {
    let board = vec![
        vec!['A', 'A', 'A', 'A', 'A', 'A'],
        vec!['A', 'A', 'A', 'A', 'A', 'A'],
        vec!['A', 'A', 'A', 'A', 'A', 'A'],
        vec!['A', 'A', 'A', 'A', 'A', 'A'],
        vec!['A', 'A', 'A', 'A', 'A', 'A'],
        vec!['A', 'A', 'A', 'A', 'A', 'A'],
    ];
    let word = "AAAAAAAAAAAAAAa".to_string();
    assert!(!Solution::exist(board, word));
}
