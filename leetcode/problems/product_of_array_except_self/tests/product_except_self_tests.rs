use product_of_array_except_self::product_except_self;

#[test]
fn test_product_except_self_basic() {
    // Test case 1: Basic functionality
    let nums1 = vec![1, 2, 3, 4];
    let result1 = product_except_self(nums1);
    assert_eq!(result1, vec![24, 12, 8, 6]);
}

#[test]
fn test_product_except_self_with_zeros() {
    // Test case 2: Array with a zero
    let nums2 = vec![-1, 1, 0, -3, 3];
    let result2 = product_except_self(nums2);
    assert_eq!(result2, vec![0, 0, 9, 0, 0]);

    // Test case: Array with two zeros
    let nums_two_zeros = vec![0, 1, 2, 0, 3];
    let result_two_zeros = product_except_self(nums_two_zeros);
    // When there are two or more zeros, all elements in the result should be 0
    assert_eq!(result_two_zeros, vec![0, 0, 0, 0, 0]);
}

#[test]
fn test_product_except_self_minimum_size() {
    // Test case 3: Edge case with only two elements
    let nums3 = vec![2, 3];
    let result3 = product_except_self(nums3);
    assert_eq!(result3, vec![3, 2]);

    // Test case: Minimum size with larger values
    let nums_min_large = vec![10, 20];
    let result_min_large = product_except_self(nums_min_large);
    assert_eq!(result_min_large, vec![20, 10]);
}

#[test]
fn test_product_except_self_negative_numbers() {
    // Test case: Negative numbers
    let nums_neg = vec![-1, -2, -3, -4];
    let result_neg = product_except_self(nums_neg);
    assert_eq!(result_neg, vec![-24, -12, -8, -6]);

    // Test case: Mix of positive and negative numbers
    let nums_mix = vec![-1, 2, -3, 4];
    let result_mix = product_except_self(nums_mix);
    assert_eq!(result_mix, vec![-24, 12, -8, 6]);
}

#[test]
fn test_product_except_self_ones() {
    // Test case: All ones
    let nums_ones = vec![1, 1, 1, 1];
    let result_ones = product_except_self(nums_ones);
    assert_eq!(result_ones, vec![1, 1, 1, 1]);

    // Test case: All elements are the same non-one value
    let nums_same = vec![5, 5, 5, 5];
    let result_same = product_except_self(nums_same);
    assert_eq!(result_same, vec![125, 125, 125, 125]); // 5^3 = 125
}

#[test]
fn test_product_except_self_integer_overflow() {
    // Test with values that might cause overflow in naive implementations
    // Using smaller values to avoid actual overflow
    let nums_potential_overflow = vec![20, 30, 40, 50];
    let result_overflow = product_except_self(nums_potential_overflow);

    // 30*40*50 = 60000, 20*40*50 = 40000, 20*30*50 = 30000, 20*30*40 = 24000
    assert_eq!(result_overflow, vec![60000, 40000, 30000, 24000]);
}
