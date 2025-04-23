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

#[test]
fn test_example_4() {
    let tasks = vec!['A', 'A', 'A', 'C', 'D', 'E', 'E'];
    // A,_,A,_,A
    // A,C,A,D,A,E,_,E => 8
    // A,E,A,E,A,C,D => 7
    let n = 1;
    assert_eq!(Solution::least_interval(tasks, n), 7);
}

#[test]
fn test_example_5() {
    let tasks = vec!['A', 'A', 'A', 'C', 'C', 'D', 'D', 'E', 'E'];
    // A,_,A,_,A
    // A,C,A,C,A,D,E,D,E
    let n = 1;
    assert_eq!(Solution::least_interval(tasks, n), 9);
}

#[test]
fn test_example_6() {
    let tasks = vec!['A', 'A', 'A', 'C', 'C', 'D', 'D', 'E', 'E'];
    // A,_,_,A,_,_,A
    // A,C,D,A,C,D,A
    // E,_,E
    let n = 2;
    assert_eq!(Solution::least_interval(tasks, n), 10);
}
