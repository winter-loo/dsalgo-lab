use insert_interval::Solution;

#[test]
fn test_example_1() {
    // Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
    // Output: [[1,5],[6,9]]
    let intervals = vec![vec![1, 3], vec![6, 9]];
    let new_interval = vec![2, 5];
    assert_eq!(Solution::insert(intervals, new_interval), vec![vec![1, 5], vec![6, 9]]);
}

#[test]
fn test_example_2() {
    // Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
    // Output: [[1,2],[3,10],[12,16]]
    let intervals = vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]];
    let new_interval = vec![4, 8];
    assert_eq!(Solution::insert(intervals, new_interval), vec![vec![1, 2], vec![3, 10], vec![12, 16]]);
}

#[test]
fn test_empty_intervals() {
    // Test with empty intervals
    let intervals = Vec::<Vec<i32>>::new();
    let new_interval = vec![5, 7];
    assert_eq!(Solution::insert(intervals, new_interval), vec![vec![5, 7]]);
}

#[test]
fn test_insert_at_beginning() {
    // Test inserting at the beginning
    let intervals = vec![vec![3, 5], vec![6, 9]];
    let new_interval = vec![1, 2];
    assert_eq!(Solution::insert(intervals, new_interval), vec![vec![1, 2], vec![3, 5], vec![6, 9]]);
}

#[test]
fn test_insert_at_end() {
    // Test inserting at the end
    let intervals = vec![vec![1, 2], vec![3, 5]];
    let new_interval = vec![6, 8];
    assert_eq!(Solution::insert(intervals, new_interval), vec![vec![1, 2], vec![3, 5], vec![6, 8]]);
}

#[test]
fn test_merge_all() {
    // Test merging all intervals
    let intervals = vec![vec![1, 3], vec![4, 6], vec![8, 10]];
    let new_interval = vec![2, 9];
    assert_eq!(Solution::insert(intervals, new_interval), vec![vec![1, 10]]);
}

#[test]
fn test_no_overlap() {
    // Test with no overlap
    let intervals = vec![vec![1, 3], vec![6, 9]];
    let new_interval = vec![4, 5];
    assert_eq!(Solution::insert(intervals, new_interval), vec![vec![1, 3], vec![4, 5], vec![6, 9]]);
}

#[test]
fn test_exact_overlap() {
    // Test with exact overlap
    let intervals = vec![vec![1, 3], vec![6, 9]];
    let new_interval = vec![3, 6];
    assert_eq!(Solution::insert(intervals, new_interval), vec![vec![1, 9]]);
}
