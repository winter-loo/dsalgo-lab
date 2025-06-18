use reverse_bits::Solution;

#[test]
fn test_example_1() {
    // Input: n = 00000010100101000001111010011100 (43261596)
    // Output: 00111001011110000010100101000000 (964176192)
    let n = 0b00000010100101000001111010011100;
    assert_eq!(Solution::reverse_bits(n), 0b00111001011110000010100101000000);
}

#[test]
fn test_example_2() {
    // Input: n = 11111111111111111111111111111101 (4294967293)
    // Output: 10111111111111111111111111111111 (3221225471)
    let n = 0b11111111111111111111111111111101;
    assert_eq!(Solution::reverse_bits(n), 0b10111111111111111111111111111111);
}

#[test]
fn test_zero() {
    // Test with zero
    let n = 0;
    assert_eq!(Solution::reverse_bits(n), 0);
}

#[test]
fn test_all_ones() {
    // Test with all bits set to 1
    let n = 0xFFFFFFFF;
    assert_eq!(Solution::reverse_bits(n), 0xFFFFFFFF);
}

#[test]
fn test_powers_of_two() {
    // Test with powers of two
    let n = 1; // 2^0
    assert_eq!(Solution::reverse_bits(n), 0x80000000);
    
    let n = 2; // 2^1
    assert_eq!(Solution::reverse_bits(n), 0x40000000);
    
    let n = 0x80000000; // 2^31
    assert_eq!(Solution::reverse_bits(n), 1);
}

#[test]
fn test_alternating_bits() {
    // Test with alternating bits
    let n = 0x55555555; // 01010101...
    assert_eq!(Solution::reverse_bits(n), 0xAAAAAAAA); // 10101010...
    
    let n = 0xAAAAAAAA; // 10101010...
    assert_eq!(Solution::reverse_bits(n), 0x55555555); // 01010101...
}
