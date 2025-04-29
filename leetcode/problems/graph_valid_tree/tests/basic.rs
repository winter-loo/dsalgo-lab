use graph_valid_tree::Solution;

#[test]
fn test_example_1() {
    // Input: n = 5, edges = [[0,1],[0,2],[0,3],[1,4]]
    // Output: true
    let n = 5;
    let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]];
    
    assert_eq!(Solution::valid_tree(n, edges), true);
}

#[test]
fn test_example_2() {
    // Input: n = 5, edges = [[0,1],[1,2],[2,3],[1,3],[1,4]]
    // Output: false
    let n = 5;
    let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![1, 3], vec![1, 4]];
    
    assert_eq!(Solution::valid_tree(n, edges), false);
}

#[test]
fn test_disconnected_graph() {
    // A disconnected graph is not a valid tree
    let n = 5;
    let edges = vec![vec![0, 1], vec![2, 3]];
    
    assert_eq!(Solution::valid_tree(n, edges), false);
}

#[test]
fn test_single_node() {
    // A single node with no edges is a valid tree
    let n = 1;
    let edges: Vec<Vec<i32>> = vec![];
    
    assert_eq!(Solution::valid_tree(n, edges), true);
}

#[test]
fn test_too_many_edges() {
    // A graph with n nodes and n or more edges cannot be a tree
    let n = 3;
    let edges = vec![vec![0, 1], vec![1, 2], vec![0, 2]];
    
    assert_eq!(Solution::valid_tree(n, edges), false);
}
