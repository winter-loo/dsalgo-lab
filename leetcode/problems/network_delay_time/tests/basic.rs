use network_delay_time::Solution;

#[test]
fn test_example_1() {
    // Input: times = [[2,1,1],[2,3,1],[3,4,1]], n = 4, k = 2
    // Output: 2
    let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
    let n = 4;
    let k = 2;

    assert_eq!(Solution::network_delay_time(times, n, k), 2);
}

#[test]
fn test_example_2() {
    // Input: times = [[1,2,1]], n = 2, k = 1
    // Output: 1
    let times = vec![vec![1, 2, 1]];
    let n = 2;
    let k = 1;

    assert_eq!(Solution::network_delay_time(times, n, k), 1);
}

#[test]
fn test_example_3() {
    // Input: times = [[1,2,1]], n = 2, k = 2
    // Output: -1
    let times = vec![vec![1, 2, 1]];
    let n = 2;
    let k = 2;

    assert_eq!(Solution::network_delay_time(times, n, k), -1);
}

#[test]
fn test_complex_graph() {
    // A more complex graph with multiple paths
    // Input: times = [[1,2,1],[1,3,4],[2,3,2],[3,4,3]], n = 4, k = 1
    // Output: 7
    let times = vec![vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 2], vec![3, 4, 3]];
    let n = 4;
    let k = 1;

    assert_eq!(Solution::network_delay_time(times, n, k), 6);
}

#[test]
fn test_single_node() {
    // A graph with a single node
    // Input: times = [], n = 1, k = 1
    // Output: 0
    let times: Vec<Vec<i32>> = vec![];
    let n = 1;
    let k = 1;

    assert_eq!(Solution::network_delay_time(times, n, k), 0);
}
