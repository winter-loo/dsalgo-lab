# Approach

The "Longest Common Subsequence" (LCS) problem asks us to find the length of the longest subsequence that is common to two given strings.

## Strategy

This problem is a classic dynamic programming problem that can be solved using:
1. **Dynamic Programming**: Build a solution using a 2D DP table
2. **Recursion with Memoization**: Top-down approach with caching

## Dynamic Programming Approach
1. Create a 2D DP table where `dp[i][j]` represents the length of the LCS of the first `i` characters of `text1` and the first `j` characters of `text2`
2. Initialize the first row and column to 0 (empty string has no common subsequence with any string)
3. For each character in both strings:
   - If the characters match: `dp[i][j] = dp[i-1][j-1] + 1` (extend the LCS by 1)
   - If the characters don't match: `dp[i][j] = max(dp[i-1][j], dp[i][j-1])` (take the maximum of excluding either character)
4. Return `dp[m][n]` where `m` and `n` are the lengths of the two strings

## Recursion with Memoization
1. Define a recursive function that computes the LCS of the substrings starting at indices `i` and `j`
2. Use memoization to avoid recalculating the same subproblems
3. Base case: if either index reaches the end of its string, return 0
4. Recursive case:
   - If the characters match, return 1 + LCS of the next characters in both strings
   - If the characters don't match, return the maximum of LCS excluding either character

## Time and Space Complexity
- **Time Complexity**: O(m * n) where m and n are the lengths of the two strings
- **Space Complexity**: O(m * n) for the DP table or memoization array

## Pseudocode for Dynamic Programming Approach
```
function longestCommonSubsequence(text1, text2):
    m = length(text1)
    n = length(text2)
    dp = 2D array of size (m+1) x (n+1), initialized to 0
    
    for i from 1 to m:
        for j from 1 to n:
            if text1[i-1] == text2[j-1]:
                dp[i][j] = dp[i-1][j-1] + 1
            else:
                dp[i][j] = max(dp[i-1][j], dp[i][j-1])
    
    return dp[m][n]
```

## Pseudocode for Recursion with Memoization
```
function longestCommonSubsequence(text1, text2):
    m = length(text1)
    n = length(text2)
    memo = 2D array of size m x n, initialized to -1
    
    return lcs(text1, text2, 0, 0, memo)

function lcs(text1, text2, i, j, memo):
    if i == length(text1) or j == length(text2):
        return 0
    
    if memo[i][j] != -1:
        return memo[i][j]
    
    if text1[i] == text2[j]:
        memo[i][j] = 1 + lcs(text1, text2, i+1, j+1, memo)
    else:
        memo[i][j] = max(lcs(text1, text2, i+1, j, memo), lcs(text1, text2, i, j+1, memo))
    
    return memo[i][j]
```
