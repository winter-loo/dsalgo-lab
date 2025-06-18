# Hints for Counting Bits

1. You can solve this problem by counting the number of 1 bits for each number from 0 to n, but that would be O(n log n) time complexity.

2. Think about how to reuse the results of previous calculations to achieve O(n) time complexity.

3. Consider the relationship between the number of 1 bits in a number and the number of 1 bits in related numbers.

4. For any number i, what is the relationship between the number of 1 bits in i and the number of 1 bits in i/2 (or i >> 1)?

5. If i is even, then i and i/2 have the same number of 1 bits because shifting right by 1 just removes the least significant bit, which is 0.

6. If i is odd, then i has one more 1 bit than i/2 because the least significant bit is 1.

7. This leads to the recurrence relation: dp[i] = dp[i >> 1] + (i & 1).

8. Alternatively, you can use the fact that i & (i-1) removes the least significant set bit of i. The number of 1 bits in i is 1 + the number of 1 bits in i & (i-1).

9. This leads to another recurrence relation: dp[i] = dp[i & (i-1)] + 1.

10. Both approaches allow you to compute the result in O(n) time and O(n) space.
