use walls_and_gates::Solution;

const INF: i32 = 2147483647;

#[test]
fn test_example_1() {
    // Input: rooms = [[2147483647,-1,0,2147483647],[2147483647,2147483647,2147483647,-1],[2147483647,-1,2147483647,-1],[0,-1,2147483647,2147483647]]
    // Output: [[3,-1,0,1],[2,2,1,-1],[1,-1,2,-1],[0,-1,3,4]]
    let mut rooms = vec![
        vec![INF, -1, 0, INF],
        vec![INF, INF, INF, -1],
        vec![INF, -1, INF, -1],
        vec![0, -1, INF, INF]
    ];
    
    let expected = vec![
        vec![3, -1, 0, 1],
        vec![2, 2, 1, -1],
        vec![1, -1, 2, -1],
        vec![0, -1, 3, 4]
    ];
    
    Solution::walls_and_gates(&mut rooms);
    assert_eq!(rooms, expected);
}

#[test]
fn test_example_2() {
    // Input: rooms = [[-1]]
    // Output: [[-1]]
    let mut rooms = vec![vec![-1]];
    
    let expected = vec![vec![-1]];
    
    Solution::walls_and_gates(&mut rooms);
    assert_eq!(rooms, expected);
}

#[test]
fn test_empty_rooms() {
    // Input: rooms = [[2147483647,2147483647],[2147483647,2147483647]]
    // Output: [[2147483647,2147483647],[2147483647,2147483647]]
    // No gates, so all rooms remain INF
    let mut rooms = vec![
        vec![INF, INF],
        vec![INF, INF]
    ];
    
    let expected = vec![
        vec![INF, INF],
        vec![INF, INF]
    ];
    
    Solution::walls_and_gates(&mut rooms);
    assert_eq!(rooms, expected);
}

#[test]
fn test_all_gates() {
    // Input: rooms = [[0,0],[0,0]]
    // Output: [[0,0],[0,0]]
    // All cells are gates
    let mut rooms = vec![
        vec![0, 0],
        vec![0, 0]
    ];
    
    let expected = vec![
        vec![0, 0],
        vec![0, 0]
    ];
    
    Solution::walls_and_gates(&mut rooms);
    assert_eq!(rooms, expected);
}

#[test]
fn test_all_walls() {
    // Input: rooms = [[-1,-1],[-1,-1]]
    // Output: [[-1,-1],[-1,-1]]
    // All cells are walls
    let mut rooms = vec![
        vec![-1, -1],
        vec![-1, -1]
    ];
    
    let expected = vec![
        vec![-1, -1],
        vec![-1, -1]
    ];
    
    Solution::walls_and_gates(&mut rooms);
    assert_eq!(rooms, expected);
}

#[test]
fn test_surrounded_by_walls() {
    // Input: rooms = [[-1,-1,-1],[-1,2147483647,-1],[-1,-1,-1]]
    // Output: [[-1,-1,-1],[-1,2147483647,-1],[-1,-1,-1]]
    // Room surrounded by walls, cannot reach any gate
    let mut rooms = vec![
        vec![-1, -1, -1],
        vec![-1, INF, -1],
        vec![-1, -1, -1]
    ];
    
    let expected = vec![
        vec![-1, -1, -1],
        vec![-1, INF, -1],
        vec![-1, -1, -1]
    ];
    
    Solution::walls_and_gates(&mut rooms);
    assert_eq!(rooms, expected);
}
