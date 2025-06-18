use meeting_rooms::Solution;

#[test]
fn test_example_1() {
    // Input: intervals = [[0,30],[5,10],[15,20]]
    // Output: false
    let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
    assert_eq!(Solution::can_attend_meetings(intervals), false);
}

#[test]
fn test_example_2() {
    // Input: intervals = [[7,10],[2,4]]
    // Output: true
    let intervals = vec![vec![7, 10], vec![2, 4]];
    assert_eq!(Solution::can_attend_meetings(intervals), true);
}

#[test]
fn test_empty_intervals() {
    // Test with empty intervals
    let intervals = Vec::<Vec<i32>>::new();
    assert_eq!(Solution::can_attend_meetings(intervals), true);
}

#[test]
fn test_single_interval() {
    // Test with a single interval
    let intervals = vec![vec![1, 5]];
    assert_eq!(Solution::can_attend_meetings(intervals), true);
}

#[test]
fn test_adjacent_intervals() {
    // Test with adjacent intervals
    let intervals = vec![vec![1, 3], vec![3, 5], vec![5, 7]];
    assert_eq!(Solution::can_attend_meetings(intervals), true);
}

#[test]
fn test_unsorted_intervals() {
    // Test with unsorted intervals
    let intervals = vec![vec![5, 10], vec![0, 3], vec![15, 20]];
    assert_eq!(Solution::can_attend_meetings(intervals), true);
}

#[test]
fn test_multiple_overlaps() {
    // Test with multiple overlapping intervals
    let intervals = vec![vec![0, 10], vec![5, 15], vec![10, 20]];
    assert_eq!(Solution::can_attend_meetings(intervals), false);
}

#[test]
fn test_nested_intervals() {
    // Test with nested intervals
    let intervals = vec![vec![1, 10], vec![2, 5], vec![3, 4]];
    assert_eq!(Solution::can_attend_meetings(intervals), false);
}
