use number_of_connected_components_in_an_undirected_graph::Solution;

#[test]
fn test_example_1() {
    // Input: n = 5, edges = [[0,1],[1,2],[3,4]]
    // Output: 2
    let n = 5;
    let edges = vec![vec![0, 1], vec![1, 2], vec![3, 4]];
    
    assert_eq!(Solution::count_components(n, edges), 2);
}

#[test]
fn test_example_2() {
    // Input: n = 5, edges = [[0,1],[1,2],[2,3],[3,4]]
    // Output: 1
    let n = 5;
    let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
    
    assert_eq!(Solution::count_components(n, edges), 1);
}

#[test]
fn test_no_edges() {
    // If there are no edges, each node is its own component
    let n = 5;
    let edges: Vec<Vec<i32>> = vec![];
    
    assert_eq!(Solution::count_components(n, edges), 5);
}

#[test]
fn test_single_node() {
    // A single node is one component
    let n = 1;
    let edges: Vec<Vec<i32>> = vec![];
    
    assert_eq!(Solution::count_components(n, edges), 1);
}

#[test]
fn test_multiple_components() {
    // Multiple disconnected components
    // Component 1: 0, 1
    // Component 2: 2, 3, 4
    // Component 3: 5
    let n = 6;
    let edges = vec![vec![0, 1], vec![2, 3], vec![3, 4]];
    
    assert_eq!(Solution::count_components(n, edges), 3);
}
