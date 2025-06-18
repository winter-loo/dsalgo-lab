use number_of_1_bits::Solution;

#[test]
fn test_example_1() {
    // Input: n = 00000000000000000000000000001011
    // Output: 3
    let n = 0b00000000000000000000000000001011;
    assert_eq!(Solution::hammingWeight(n), 3);
}

#[test]
fn test_example_2() {
    // Input: n = 00000000000000000000000010000000
    // Output: 1
    let n = 0b00000000000000000000000010000000;
    assert_eq!(Solution::hammingWeight(n), 1);
}

#[test]
fn test_example_3() {
    // Input: n = 11111111111111111111111111111101
    // Output: 31
    let n = 0b11111111111111111111111111111101;
    assert_eq!(Solution::hammingWeight(n), 31);
}

#[test]
fn test_zero() {
    // Test with zero
    let n = 0;
    assert_eq!(Solution::hammingWeight(n), 0);
}

#[test]
fn test_all_ones() {
    // Test with all bits set to 1
    let n = 0xFFFFFFFF;
    assert_eq!(Solution::hammingWeight(n), 32);
}

#[test]
fn test_powers_of_two() {
    // Test with powers of two (only one bit set)
    let n = 0x00000001; // 2^0
    assert_eq!(Solution::hammingWeight(n), 1);
    
    let n = 0x00000002; // 2^1
    assert_eq!(Solution::hammingWeight(n), 1);
    
    let n = 0x00000004; // 2^2
    assert_eq!(Solution::hammingWeight(n), 1);
    
    let n = 0x80000000; // 2^31
    assert_eq!(Solution::hammingWeight(n), 1);
}

#[test]
fn test_alternating_bits() {
    // Test with alternating bits
    let n = 0x55555555; // 01010101...
    assert_eq!(Solution::hammingWeight(n), 16);
    
    let n = 0xAAAAAAAA; // 10101010...
    assert_eq!(Solution::hammingWeight(n), 16);
}
