# Hints for Longest Increasing Subsequence

1. A subsequence is a sequence that can be derived from an array by deleting some or no elements without changing the order of the remaining elements.

2. The problem asks for the length of the longest strictly increasing subsequence, not the subsequence itself.

3. Think about using dynamic programming. For each position, consider the longest increasing subsequence that ends at that position.

4. Define dp[i] as the length of the longest increasing subsequence ending at index i.

5. The base case is that dp[i] = 1 for all i, since each element by itself forms a subsequence of length 1.

6. For each position i, look at all previous positions j < i. If nums[i] > nums[j], then we can extend the subsequence ending at j to include the element at i.

7. The recurrence relation is: dp[i] = max(dp[i], dp[j] + 1) for all j < i where nums[i] > nums[j].

8. The final answer is the maximum value in the dp array.

9. For the O(n log n) solution, think about using binary search. Maintain an array where tails[i] represents the smallest ending value of all increasing subsequences of length i+1.

10. When processing a new element, use binary search to find where it fits in the tails array, and update accordingly.
