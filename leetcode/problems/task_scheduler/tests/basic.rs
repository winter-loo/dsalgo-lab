use task_scheduler::Solution;

#[test]
fn test_example_1() {
    // Input: tasks = ["A","A","A","B","B","B"], n = 2
    // Output: 8
    let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
    let n = 2;
    assert_eq!(Solution::least_interval(tasks, n), 8);
}

#[test]
fn test_example_2() {
    // Input: tasks = ["A","A","A","B","B","B"], n = 0
    // Output: 6
    let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
    let n = 0;
    assert_eq!(Solution::least_interval(tasks, n), 6);
}

#[test]
fn test_example_3() {
    // Input: tasks = ["A","A","A","A","A","A","B","C","D","E","F","G"], n = 2
    // Output: 16
    let tasks = vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'];
    let n = 2;
    assert_eq!(Solution::least_interval(tasks, n), 16);
}

#[test]
fn test_single_task() {
    // Input: tasks = ["A"], n = 2
    // Output: 1
    let tasks = vec!['A'];
    let n = 2;
    assert_eq!(Solution::least_interval(tasks, n), 1);
}

#[test]
fn test_all_different_tasks() {
    // Input: tasks = ["A","B","C","D","E"], n = 3
    // Output: 5
    let tasks = vec!['A', 'B', 'C', 'D', 'E'];
    let n = 3;
    assert_eq!(Solution::least_interval(tasks, n), 5);
}
