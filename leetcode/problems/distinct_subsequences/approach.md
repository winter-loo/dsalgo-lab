# Approach

The "Distinct Subsequences" problem asks us to find the number of distinct subsequences of string s that equal string t.

## Strategy

This problem can be solved using:
1. **Dynamic Programming**: Build a solution using a 2D DP table
2. **Recursive Approach with Memoization**: Top-down approach with caching

## Dynamic Programming Approach
1. Create a 2D DP table where dp[i][j] represents the number of distinct subsequences of s[0...i-1] that equal t[0...j-1]
2. Initialize the base cases:
   - dp[i][0] = 1 for all i (empty string t is a subsequence of any string s, and there's exactly one way to form it)
   - dp[0][j] = 0 for j > 0 (non-empty string t cannot be a subsequence of empty string s)
3. For each cell dp[i][j]:
   - If s[i-1] == t[j-1], then dp[i][j] = dp[i-1][j-1] + dp[i-1][j]
     - dp[i-1][j-1]: use s[i-1] to match t[j-1]
     - dp[i-1][j]: skip s[i-1] and try to match t[j] with earlier characters in s
   - Otherwise, dp[i][j] = dp[i-1][j] (skip s[i-1])
4. Return dp[m][n] where m and n are the lengths of s and t

## Time and Space Complexity
- **Time Complexity**: O(m * n) where m and n are the lengths of strings s and t
  - We need to fill in each cell of the DP table once
- **Space Complexity**: O(m * n) for the DP table

## Pseudocode for Dynamic Programming
```
function numDistinct(s, t):
    m = length of s
    n = length of t
    
    // Create DP table
    dp = 2D array of size (m+1) x (n+1)
    
    // Base cases
    for i from 0 to m:
        dp[i][0] = 1
    for j from 1 to n:
        dp[0][j] = 0
    
    // Fill the DP table
    for i from 1 to m:
        for j from 1 to n:
            if s[i-1] == t[j-1]:
                dp[i][j] = dp[i-1][j-1] + dp[i-1][j]
            else:
                dp[i][j] = dp[i-1][j]
    
    return dp[m][n]
```

## Alternative Approach: Recursive with Memoization
1. Use a recursive function that computes the number of distinct subsequences of s[i...] that equal t[j...]
2. Use memoization to avoid recalculating the same subproblems
3. The base cases are:
   - If j == n, return 1 (we've matched all characters in t)
   - If i == m, return 0 (we've exhausted s but not t)

This approach is more intuitive but may have higher overhead due to the recursion stack.
