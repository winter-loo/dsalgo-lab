# Approach

The "Number of 1 Bits" problem asks us to count the number of '1' bits in a 32-bit unsigned integer.

## Understanding the Problem

We need to count how many bits in the binary representation of the given integer are set to 1.

## Bit Manipulation Approaches

There are several approaches to solve this problem using bit manipulation:

### 1. Loop and Bit Shift

1. Initialize a counter to 0.
2. Check the least significant bit (LSB) of the number.
3. If the LSB is 1, increment the counter.
4. Right shift the number by 1 bit.
5. Repeat steps 2-4 until the number becomes 0.

### 2. Loop and Mask

1. Initialize a counter to 0.
2. Check if the number AND 1 equals 1 (which means the LSB is 1).
3. If yes, increment the counter.
4. Right shift the number by 1 bit.
5. Repeat steps 2-4 for all 32 bits or until the number becomes 0.

### 3. Brian Kernighan's Algorithm

This algorithm is more efficient when the number of set bits is small:
1. Initialize a counter to 0.
2. While the number is not 0:
   - Perform n = n & (n-1) to clear the least significant set bit.
   - Increment the counter.
3. Return the counter.

## Time and Space Complexity

- **Time Complexity**: O(k), where k is the number of set bits in the number for Brian Kernighan's algorithm, or O(32) = O(1) for the loop approaches since we're dealing with a fixed-size integer.
- **Space Complexity**: O(1) for all approaches, as we only use a single variable to store the count.

## Pseudocode for Brian Kernighan's Algorithm

```
function hammingWeight(n):
    count = 0
    
    while n != 0:
        n = n & (n - 1)  // Clear the least significant set bit
        count++
    
    return count
```

## Step-by-Step Example

Let's trace through Brian Kernighan's algorithm with the example: n = 00000000000000000000000000001011 (decimal 11)

1. Initialize count = 0.
2. n = 11 (binary 1011), n != 0, so enter the loop.
   - n = n & (n - 1) = 1011 & 1010 = 1010 (decimal 10)
   - count = 1
3. n = 10 (binary 1010), n != 0, so enter the loop.
   - n = n & (n - 1) = 1010 & 1001 = 1000 (decimal 8)
   - count = 2
4. n = 8 (binary 1000), n != 0, so enter the loop.
   - n = n & (n - 1) = 1000 & 0111 = 0000 (decimal 0)
   - count = 3
5. n = 0, so exit the loop.
6. Return count = 3.

## Alternative Approaches

### Built-in Functions

Many programming languages provide built-in functions to count the number of set bits:
- In C++: `__builtin_popcount(n)`
- In Java: `Integer.bitCount(n)`
- In Python: `bin(n).count('1')`

However, the purpose of this problem is to understand bit manipulation, so it's better to implement the solution manually.

### Lookup Table

For a fixed-size integer (like 32 bits), we can use a lookup table to count the number of set bits in chunks (e.g., 8 bits at a time). This approach can be faster but requires more space.

Brian Kernighan's algorithm is generally the most efficient for this problem, especially when the number of set bits is small.
