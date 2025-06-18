use merge_triplets_to_form_target_triplet::Solution;

#[test]
fn test_example_1() {
    // Input: triplets = [[2,5,3],[1,8,4],[1,7,5]], target = [2,7,5]
    // Output: true
    let triplets = vec![vec![2, 5, 3], vec![1, 8, 4], vec![1, 7, 5]];
    let target = vec![2, 7, 5];
    assert_eq!(Solution::merge_triplets(triplets, target), true);
}

#[test]
fn test_example_2() {
    // Input: triplets = [[3,4,5],[4,5,6]], target = [3,2,5]
    // Output: false
    let triplets = vec![vec![3, 4, 5], vec![4, 5, 6]];
    let target = vec![3, 2, 5];
    assert_eq!(Solution::merge_triplets(triplets, target), false);
}

#[test]
fn test_example_3() {
    // Input: triplets = [[2,5,3],[2,3,4],[1,2,5],[5,2,3]], target = [5,5,5]
    // Output: true
    let triplets = vec![vec![2, 5, 3], vec![2, 3, 4], vec![1, 2, 5], vec![5, 2, 3]];
    let target = vec![5, 5, 5];
    assert_eq!(Solution::merge_triplets(triplets, target), true);
}

#[test]
fn test_single_triplet_match() {
    // Test when a single triplet matches the target
    let triplets = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let target = vec![4, 5, 6];
    assert_eq!(Solution::merge_triplets(triplets, target), true);
}

#[test]
fn test_no_match_possible() {
    // Test when no combination can form the target
    let triplets = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let target = vec![9, 9, 1];
    assert_eq!(Solution::merge_triplets(triplets, target), false);
}

#[test]
fn test_partial_match() {
    // Test when we can match some but not all target elements
    let triplets = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let target = vec![7, 5, 3];
    assert_eq!(Solution::merge_triplets(triplets, target), true);
}

#[test]
fn test_all_triplets_needed() {
    // Test when all triplets are needed to form the target
    let triplets = vec![vec![1, 5, 3], vec![4, 2, 6], vec![7, 8, 1]];
    let target = vec![7, 8, 6];
    assert_eq!(Solution::merge_triplets(triplets, target), true);
}

#[test]
fn test_large_values() {
    // Test with large values within constraints
    let triplets = vec![vec![1000, 500, 300], vec![400, 1000, 600], vec![700, 800, 1000]];
    let target = vec![1000, 1000, 1000];
    assert_eq!(Solution::merge_triplets(triplets, target), true);
}
