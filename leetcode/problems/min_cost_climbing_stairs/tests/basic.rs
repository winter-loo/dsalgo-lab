use min_cost_climbing_stairs::Solution;

#[test]
fn test_example_1() {
    // Input: cost = [10,15,20]
    // Output: 15
    let cost = vec![10, 15, 20];
    // f(4) -> 0
    // f(3) -> 20
    // f(2) -> 15 + min(f(3), f(4)) -> 15
    // f(1) -> 10 + min(f(2), f(3)) -> 10 + 15 -> 25
    // f(0) -> 0 + min(f(1), f(2)) -> 15
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
    assert_eq!(Solution::min_cost_climbing_stairs(cost), 2);
}

#[test]
fn test_alternating_costs() {
    // Alternating high and low costs
    let cost = vec![10, 1, 10, 1, 10];
    assert_eq!(Solution::min_cost_climbing_stairs(cost), 2);
}
