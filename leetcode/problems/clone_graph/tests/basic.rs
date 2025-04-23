use clone_graph::{Node, Solution};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};

// Helper function to create a graph from an adjacency list
fn create_graph(adj_list: Vec<Vec<i32>>) -> Option<Rc<RefCell<Node>>> {
    if adj_list.is_empty() {
        return None;
    }
    
    let n = adj_list.len();
    let mut nodes = vec![];
    
    // Create all nodes
    for i in 0..n {
        nodes.push(Rc::new(RefCell::new(Node::new((i + 1) as i32))));
    }
    
    // Add neighbors
    for (i, neighbors) in adj_list.iter().enumerate() {
        let node = nodes[i].clone();
        for &neighbor in neighbors {
            node.borrow_mut().neighbors.push(nodes[(neighbor - 1) as usize].clone());
        }
    }
    
    Some(nodes[0].clone())
}

// Helper function to convert a graph to an adjacency list
fn graph_to_adj_list(node: Option<Rc<RefCell<Node>>>) -> Vec<Vec<i32>> {
    if node.is_none() {
        return vec![];
    }
    
    let mut adj_list = vec![];
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut node_map = HashMap::new();
    
    queue.push_back(node.unwrap());
    
    // BFS to traverse the graph
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        let val = curr.borrow().val;
        
        if visited.contains(&val) {
            continue;
        }
        
        visited.insert(val);
        node_map.insert(val, adj_list.len());
        
        let mut neighbors = vec![];
        for neighbor in curr.borrow().neighbors.iter() {
            neighbors.push(neighbor.borrow().val);
            if !visited.contains(&neighbor.borrow().val) {
                queue.push_back(neighbor.clone());
            }
        }
        
        adj_list.push(neighbors);
    }
    
    // Sort the adjacency list by node value
    let mut sorted_adj_list = vec![vec![]; adj_list.len()];
    for (val, idx) in node_map {
        sorted_adj_list[(val - 1) as usize] = adj_list[idx].clone();
    }
    
    sorted_adj_list
}

// Helper function to compare two graphs
fn compare_graphs(a: Option<Rc<RefCell<Node>>>, b: Option<Rc<RefCell<Node>>>) -> bool {
    let adj_list_a = graph_to_adj_list(a);
    let adj_list_b = graph_to_adj_list(b);
    
    if adj_list_a.len() != adj_list_b.len() {
        return false;
    }
    
    for i in 0..adj_list_a.len() {
        let mut neighbors_a = adj_list_a[i].clone();
        let mut neighbors_b = adj_list_b[i].clone();
        
        neighbors_a.sort();
        neighbors_b.sort();
        
        if neighbors_a != neighbors_b {
            return false;
        }
    }
    
    true
}

#[test]
fn test_example_1() {
    // Input: adjList = [[2,4],[1,3],[2,4],[1,3]]
    // Output: [[2,4],[1,3],[2,4],[1,3]]
    let adj_list = vec![
        vec![2, 4],
        vec![1, 3],
        vec![2, 4],
        vec![1, 3]
    ];
    
    let graph = create_graph(adj_list);
    let cloned_graph = Solution::clone_graph(graph.clone());
    
    assert!(compare_graphs(graph, cloned_graph));
}

#[test]
fn test_example_2() {
    // Input: adjList = [[]]
    // Output: [[]]
    let adj_list = vec![vec![]];
    
    let graph = create_graph(adj_list);
    let cloned_graph = Solution::clone_graph(graph.clone());
    
    assert!(compare_graphs(graph, cloned_graph));
}

#[test]
fn test_example_3() {
    // Input: adjList = []
    // Output: []
    let adj_list: Vec<Vec<i32>> = vec![];
    
    let graph = create_graph(adj_list);
    let cloned_graph = Solution::clone_graph(graph.clone());
    
    assert!(compare_graphs(graph, cloned_graph));
}

#[test]
fn test_single_node_with_self_loop() {
    // Input: adjList = [[1]]
    // Output: [[1]]
    let adj_list = vec![vec![1]];
    
    let graph = create_graph(adj_list);
    let cloned_graph = Solution::clone_graph(graph.clone());
    
    assert!(compare_graphs(graph, cloned_graph));
}

#[test]
fn test_linear_graph() {
    // Input: adjList = [[2],[1,3],[2]]
    // Output: [[2],[1,3],[2]]
    let adj_list = vec![
        vec![2],
        vec![1, 3],
        vec![2]
    ];
    
    let graph = create_graph(adj_list);
    let cloned_graph = Solution::clone_graph(graph.clone());
    
    assert!(compare_graphs(graph, cloned_graph));
}
