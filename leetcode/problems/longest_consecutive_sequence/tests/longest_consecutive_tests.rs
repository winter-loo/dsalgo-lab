use longest_consecutive_sequence::longest_consecutive;

#[test]
fn test_basic_cases() {
    // Example 1: Basic test with small sequence
    let nums1 = vec![100, 4, 200, 1, 3, 2];
    assert_eq!(longest_consecutive(nums1), 4);
    
    // Example 2: Longer sequence with duplicates
    let nums2 = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
    assert_eq!(longest_consecutive(nums2), 9);
}

#[test]
fn test_empty_array() {
    // Empty array should return 0
    let nums = vec![];
    assert_eq!(longest_consecutive(nums), 0);
}

#[test]
fn test_single_element() {
    // Array with a single element should return 1
    let nums = vec![42];
    assert_eq!(longest_consecutive(nums), 1);
}

#[test]
fn test_duplicates() {
    // Array with duplicates
    let nums = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    assert_eq!(longest_consecutive(nums), 4);
    
    // All duplicates
    let nums_all_same = vec![5, 5, 5, 5, 5];
    assert_eq!(longest_consecutive(nums_all_same), 1);
}

#[test]
fn test_negative_numbers() {
    // Array with negative numbers
    let nums = vec![-3, -2, -1, 0, 1];
    assert_eq!(longest_consecutive(nums), 5);
    
    // Only negative numbers in sequence
    let nums_negative = vec![-5, -4, -3, -2, -1, -10, -20];
    assert_eq!(longest_consecutive(nums_negative), 5);
}

#[test]
fn test_multiple_sequences() {
    // Multiple sequences of different lengths
    let nums = vec![1, 2, 3, 10, 11, 12, 13, 20, 21, 22, 23, 24];
    assert_eq!(longest_consecutive(nums), 5); // Sequence: 20-24
    
    // Multiple sequences of same length
    let nums_same_length = vec![1, 2, 3, 4, 10, 11, 12, 13, 20, 21, 22, 23];
    assert_eq!(longest_consecutive(nums_same_length), 4); // Any of the sequences is valid
}

#[test]
fn test_large_range() {
    // Test with numbers at the extremes of the constraints
    let nums = vec![i32::MIN, 0, i32::MAX];
    assert_eq!(longest_consecutive(nums), 1); // No consecutive numbers
    
    // Large gap between consecutive numbers
    let nums_large_gap = vec![i32::MIN, i32::MIN + 1, i32::MAX - 1, i32::MAX];
    assert_eq!(longest_consecutive(nums_large_gap), 2); // Two separate sequences of length 2
}

#[test]
fn test_random_order() {
    // Numbers in random order
    let nums = vec![9, 1, 4, 7, 3, -1, 0, 5, 8, -3, 6, 2];
    assert_eq!(longest_consecutive(nums), 11); // Sequence: -3 to 9 (excluding -2)
}

#[test]
fn test_consecutive_at_edges() {
    // Sequence at the beginning
    let nums1 = vec![1, 2, 3, 4, 100, 200];
    assert_eq!(longest_consecutive(nums1), 4);
    
    // Sequence at the end
    let nums2 = vec![100, 200, 3, 4, 5, 6];
    assert_eq!(longest_consecutive(nums2), 4);
} 