use climbing_stairs::Solution;

#[test]
fn test_example_1() {
    // Input: n = 2
    // Output: 2
    assert_eq!(Solution::climb_stairs(2), 2);
}

#[test]
fn test_example_2() {
    // Input: n = 3
    // Output: 3
    assert_eq!(Solution::climb_stairs(3), 3);
}

#[test]
fn test_one_step() {
    // Only one way to climb 1 step
    assert_eq!(Solution::climb_stairs(1), 1);
}

#[test]
fn test_four_steps() {
    // There are 5 ways to climb 4 steps:
    // 1. 1 + 1 + 1 + 1
    // 2. 1 + 1 + 2
    // 3. 1 + 2 + 1
    // 4. 2 + 1 + 1
    // 5. 2 + 2
    assert_eq!(Solution::climb_stairs(4), 5);
}

#[test]
fn test_five_steps() {
    // There are 8 ways to climb 5 steps
    assert_eq!(Solution::climb_stairs(5), 8);
}

#[test]
fn test_larger_input() {
    // Testing with a larger input
    // For n = 10, the answer is 89
    assert_eq!(Solution::climb_stairs(10), 89);
}
