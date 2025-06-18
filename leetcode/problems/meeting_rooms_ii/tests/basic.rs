use meeting_rooms_ii::Solution;

#[test]
fn test_example_1() {
    // Input: intervals = [[0,30],[5,10],[15,20]]
    // Output: 2
    let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
    assert_eq!(Solution::min_meeting_rooms(intervals), 2);
}

#[test]
fn test_example_2() {
    // Input: intervals = [[7,10],[2,4]]
    // Output: 1
    let intervals = vec![vec![7, 10], vec![2, 4]];
    assert_eq!(Solution::min_meeting_rooms(intervals), 1);
}

#[test]
fn test_single_interval() {
    // Test with a single interval
    let intervals = vec![vec![1, 5]];
    assert_eq!(Solution::min_meeting_rooms(intervals), 1);
}

#[test]
fn test_adjacent_intervals() {
    // Test with adjacent intervals
    let intervals = vec![vec![1, 3], vec![3, 5], vec![5, 7]];
    assert_eq!(Solution::min_meeting_rooms(intervals), 1);
}

#[test]
fn test_unsorted_intervals() {
    // Test with unsorted intervals
    let intervals = vec![vec![5, 10], vec![0, 3], vec![15, 20]];
    assert_eq!(Solution::min_meeting_rooms(intervals), 1);
}

#[test]
fn test_multiple_overlaps() {
    // Test with multiple overlapping intervals
    let intervals = vec![vec![0, 10], vec![5, 15], vec![10, 20], vec![15, 25]];
    assert_eq!(Solution::min_meeting_rooms(intervals), 2);
}

#[test]
fn test_all_overlapping() {
    // Test with all overlapping intervals
    let intervals = vec![vec![1, 10], vec![2, 11], vec![3, 12], vec![4, 13]];
    assert_eq!(Solution::min_meeting_rooms(intervals), 4);
}

#[test]
fn test_complex_case() {
    // Test with a more complex case
    let intervals = vec![vec![0, 5], vec![1, 2], vec![1, 10], vec![2, 8], vec![3, 4], vec![6, 9]];
    assert_eq!(Solution::min_meeting_rooms(intervals), 3);
}
