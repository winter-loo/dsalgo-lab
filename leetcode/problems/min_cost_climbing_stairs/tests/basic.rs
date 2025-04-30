use min_cost_climbing_stairs::Solution;

#[test]
fn test_example_1() {
    // Input: cost = [10,15,20]
    // Output: 15
    let cost = vec![10, 15, 20];
    assert_eq!(Solution::min_cost_climbing_stairs(cost), 15);
}

#[test]
fn test_example_2() {
    // Input: cost = [1,100,1,1,1,100,1,1,100,1]
    // Output: 6
    let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    assert_eq!(Solution::min_cost_climbing_stairs(cost), 6);
}

#[test]
fn test_minimal_case() {
    // Just two steps
    let cost = vec![5, 10];
    assert_eq!(Solution::min_cost_climbing_stairs(cost), 5);
}

#[test]
fn test_equal_cost() {
    // All steps have the same cost
    let cost = vec![1, 1, 1, 1];
    assert_eq!(Solution::min_cost_climbing_stairs(cost), 1);
}

#[test]
fn test_alternating_costs() {
    // Alternating high and low costs
    let cost = vec![10, 1, 10, 1, 10];
    assert_eq!(Solution::min_cost_climbing_stairs(cost), 3);
}
