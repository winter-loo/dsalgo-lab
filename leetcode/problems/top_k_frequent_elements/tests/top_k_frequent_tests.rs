use top_k_frequent_elements::top_k_frequent;

#[test]
fn test_top_k_frequent_basic() {
    // Test case 1: Basic functionality
    let nums1 = vec![1, 2, 2, 3, 3, 3];
    let k1 = 2;
    let mut result1 = top_k_frequent(nums1.clone(), k1);
    result1.sort(); // Sort for consistent comparison
    assert_eq!(result1, vec![2, 3]);

    // Test case 2: Single element
    let nums2 = vec![1];
    let k2 = 1;
    let result2 = top_k_frequent(nums2.clone(), k2);
    assert_eq!(result2, vec![1]);
}

#[test]
fn test_top_k_frequent_equal_frequency() {
    // Elements with equal frequency
    let nums = vec![1, 1, 2, 2, 3, 3];
    let k = 2;
    let mut result = top_k_frequent(nums, k);
    result.sort();

    // Any two of the three numbers should be returned
    assert_eq!(result.len(), 2);
    assert!(result[0] == 1 || result[0] == 2 || result[0] == 3);
    assert!(result[1] == 1 || result[1] == 2 || result[1] == 3);
    assert_ne!(result[0], result[1]);
}

#[test]
fn test_top_k_frequent_all_elements() {
    // k equals the number of unique elements
    let nums = vec![1, 1, 2, 3, 4, 4];
    let k = 4; // All unique elements
    let mut result = top_k_frequent(nums, k);
    result.sort();

    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn test_top_k_frequent_negative_numbers() {
    // Test with negative numbers
    let nums = vec![-1, -1, -2, -2, -2, 3, 3, 4];
    let k = 3;
    let mut result = top_k_frequent(nums, k);
    result.sort();

    assert_eq!(result, vec![-2, -1, 3]);
}

#[test]
fn test_top_k_frequent_large_range() {
    // Test with a larger range of numbers and frequencies
    let nums = vec![5, 5, 5, 5, 2, 2, 2, 1, 1, 4, 4, 4, 4, 4, 3, 3, 3, 3];
    let k = 3;
    let mut result = top_k_frequent(nums, k);
    result.sort();

    assert_eq!(result, vec![3, 4, 5]);
}
