# Hints for Number of 1 Bits

1. Think about how to check if the least significant bit (LSB) of a number is 1.

2. You can use the bitwise AND operation with 1 to check if the LSB is 1: `n & 1`.

3. After checking the LSB, you can right shift the number by 1 bit to check the next bit: `n >>= 1`.

4. You can iterate through all 32 bits of the number, or stop early if the number becomes 0.

5. Consider using Brian Kernighan's algorithm, which is more efficient when the number of set bits is small.

6. Brian Kernighan's algorithm uses the fact that `n & (n-1)` clears the least significant set bit of n.

7. By repeatedly applying `n & (n-1)` and counting how many times we need to apply it until n becomes 0, we get the number of set bits.

8. Be careful with negative numbers in languages that use signed integers. In such cases, treat the input as an unsigned integer.

9. Remember that the problem is asking for the Hamming weight, which is the number of 1 bits in the binary representation of the number.

10. The time complexity of your solution should be O(k), where k is the number of set bits, or O(1) since we're dealing with a fixed-size integer.
