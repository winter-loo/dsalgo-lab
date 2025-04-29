use redundant_connection::Solution;

#[test]
fn test_example_1() {
    // Input: edges = [[1,2],[1,3],[2,3]]
    // Output: [2,3]
    let edges = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
    
    assert_eq!(Solution::find_redundant_connection(edges), vec![2, 3]);
}

#[test]
fn test_example_2() {
    // Input: edges = [[1,2],[2,3],[3,4],[1,4],[1,5]]
    // Output: [1,4]
    let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]];
    
    assert_eq!(Solution::find_redundant_connection(edges), vec![1, 4]);
}

#[test]
fn test_multiple_redundant_edges() {
    // If there are multiple redundant edges, return the one that occurs last
    // Input: edges = [[1,2],[1,3],[2,3],[1,4],[1,5]]
    // Output: [2,3]
    let edges = vec![vec![1, 2], vec![1, 3], vec![2, 3], vec![1, 4], vec![1, 5]];
    
    assert_eq!(Solution::find_redundant_connection(edges), vec![2, 3]);
}

#[test]
fn test_complex_graph() {
    // A more complex graph with a cycle
    // Input: edges = [[1,2],[2,3],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9],[9,1]]
    // Output: [9,1]
    let edges = vec![
        vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5],
        vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 1]
    ];
    
    assert_eq!(Solution::find_redundant_connection(edges), vec![9, 1]);
}
