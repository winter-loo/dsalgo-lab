# Approach

The "Interleaving String" problem asks us to determine if a string `s3` can be formed by interleaving the characters from strings `s1` and `s2` while maintaining their relative order.

## Strategy

This problem can be solved using:
1. **Dynamic Programming**: Build a solution using a 2D DP table
2. **Recursion with Memoization**: Top-down approach with caching

## Dynamic Programming Approach
1. First, check if the lengths match: if `len(s1) + len(s2) != len(s3)`, return false
2. Create a 2D DP table where `dp[i][j]` represents whether the first `i` characters of `s1` and the first `j` characters of `s2` can form the first `i+j` characters of `s3`
3. Initialize `dp[0][0] = true` (empty strings can form an empty string)
4. Fill the first row: `dp[0][j] = dp[0][j-1] && s2[j-1] == s3[j-1]` for j from 1 to len(s2)
5. Fill the first column: `dp[i][0] = dp[i-1][0] && s1[i-1] == s3[i-1]` for i from 1 to len(s1)
6. Fill the rest of the table:
   - `dp[i][j] = (dp[i-1][j] && s1[i-1] == s3[i+j-1]) || (dp[i][j-1] && s2[j-1] == s3[i+j-1])`
7. Return `dp[len(s1)][len(s2)]`

## Space-Optimized Dynamic Programming
1. Since each cell in the DP table only depends on the cell to its left and the cell above it, we can optimize space to O(min(m, n))
2. Use a 1D DP array and update it in place

## Time and Space Complexity
- **Time Complexity**: O(m * n) where m and n are the lengths of s1 and s2
- **Space Complexity**: 
  - 2D DP: O(m * n)
  - 1D DP: O(min(m, n))

## Pseudocode for Dynamic Programming Approach
```
function isInterleave(s1, s2, s3):
    m = length(s1)
    n = length(s2)
    
    if m + n != length(s3):
        return false
    
    dp = 2D boolean array of size (m+1) x (n+1), initialized to false
    dp[0][0] = true
    
    // Fill the first row
    for j from 1 to n:
        dp[0][j] = dp[0][j-1] && s2[j-1] == s3[j-1]
    
    // Fill the first column
    for i from 1 to m:
        dp[i][0] = dp[i-1][0] && s1[i-1] == s3[i-1]
    
    // Fill the rest of the table
    for i from 1 to m:
        for j from 1 to n:
            dp[i][j] = (dp[i-1][j] && s1[i-1] == s3[i+j-1]) || 
                       (dp[i][j-1] && s2[j-1] == s3[i+j-1])
    
    return dp[m][n]
```

## Pseudocode for Space-Optimized Approach
```
function isInterleave(s1, s2, s3):
    m = length(s1)
    n = length(s2)
    
    if m + n != length(s3):
        return false
    
    // Ensure s1 is the shorter string for space optimization
    if m > n:
        swap(s1, s2)
        swap(m, n)
    
    dp = boolean array of size n+1, initialized to false
    dp[0] = true
    
    // Initialize the first row
    for j from 1 to n:
        dp[j] = dp[j-1] && s2[j-1] == s3[j-1]
    
    // Fill the rest of the table
    for i from 1 to m:
        dp[0] = dp[0] && s1[i-1] == s3[i-1]
        
        for j from 1 to n:
            dp[j] = (dp[j] && s1[i-1] == s3[i+j-1]) || 
                    (dp[j-1] && s2[j-1] == s3[i+j-1])
    
    return dp[n]
```
