use course_schedule::Solution;

#[test]
fn test_example_1() {
    // Input: numCourses = 2, prerequisites = [[1,0]]
    // Output: true
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0]];
    
    assert_eq!(Solution::can_finish(num_courses, prerequisites), true);
}

#[test]
fn test_example_2() {
    // Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
    // Output: false
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0], vec![0, 1]];
    
    assert_eq!(Solution::can_finish(num_courses, prerequisites), false);
}

#[test]
fn test_no_prerequisites() {
    // If there are no prerequisites, all courses can be taken
    let num_courses = 5;
    let prerequisites: Vec<Vec<i32>> = vec![];
    
    assert_eq!(Solution::can_finish(num_courses, prerequisites), true);
}

#[test]
fn test_complex_graph_no_cycle() {
    // A more complex graph with no cycle
    // 0 -> 1 -> 3
    //      |
    //      v
    //      2 -> 4
    let num_courses = 5;
    let prerequisites = vec![
        vec![1, 0], // 1 depends on 0
        vec![2, 1], // 2 depends on 1
        vec![3, 1], // 3 depends on 1
        vec![4, 2], // 4 depends on 2
    ];
    
    assert_eq!(Solution::can_finish(num_courses, prerequisites), true);
}

#[test]
fn test_complex_graph_with_cycle() {
    // A more complex graph with a cycle
    // 0 -> 1 -> 3
    //      |    |
    //      v    v
    //      2 <- 4
    let num_courses = 5;
    let prerequisites = vec![
        vec![1, 0], // 1 depends on 0
        vec![2, 1], // 2 depends on 1
        vec![3, 1], // 3 depends on 1
        vec![4, 3], // 4 depends on 3
        vec![2, 4], // 2 depends on 4 (creates a cycle: 1 -> 3 -> 4 -> 2 -> 1)
    ];
    
    assert_eq!(Solution::can_finish(num_courses, prerequisites), false);
}

#[test]
fn test_self_loop() {
    // A course depends on itself, creating a self-loop
    let num_courses = 3;
    let prerequisites = vec![
        vec![0, 0], // 0 depends on itself
        vec![1, 2], // 1 depends on 2
    ];
    
    assert_eq!(Solution::can_finish(num_courses, prerequisites), false);
}

#[test]
fn test_disconnected_components() {
    // Multiple disconnected components, one with a cycle
    // Component 1: 0 -> 1
    // Component 2: 2 -> 3 -> 4 -> 2 (cycle)
    let num_courses = 5;
    let prerequisites = vec![
        vec![1, 0], // 1 depends on 0
        vec![3, 2], // 3 depends on 2
        vec![4, 3], // 4 depends on 3
        vec![2, 4], // 2 depends on 4 (creates a cycle)
    ];
    
    assert_eq!(Solution::can_finish(num_courses, prerequisites), false);
}

#[test]
fn test_bfs_implementation() {
    // Test the BFS implementation with a simple case
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0]];
    
    assert_eq!(Solution::can_finish_bfs(num_courses, prerequisites), true);
    
    // Test with a cycle
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0], vec![0, 1]];
    
    assert_eq!(Solution::can_finish_bfs(num_courses, prerequisites), false);
}