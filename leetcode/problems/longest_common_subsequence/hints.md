# Hints for Longest Common Subsequence

1. A subsequence is a sequence that can be derived from another sequence by deleting some or no elements without changing the order of the remaining elements.

2. The longest common subsequence (LCS) is the longest sequence that appears in the same relative order in both strings, but not necessarily consecutively.

3. This is a classic dynamic programming problem. Consider using a 2D DP table where dp[i][j] represents the length of the LCS of the first i characters of text1 and the first j characters of text2.

4. The base case is that dp[i][0] = dp[0][j] = 0 for all i and j, since the LCS of any string with an empty string is 0.

5. For the recursive case, consider two scenarios:
   - If the current characters match (text1[i-1] == text2[j-1]), then dp[i][j] = dp[i-1][j-1] + 1
   - If they don't match, then dp[i][j] = max(dp[i-1][j], dp[i][j-1])

6. The final answer is dp[m][n], where m and n are the lengths of text1 and text2.

7. You can also solve this problem using recursion with memoization, which is the top-down approach to dynamic programming.

8. Be careful with the indices when implementing the solution, especially when mapping between the DP table indices and the string indices.
