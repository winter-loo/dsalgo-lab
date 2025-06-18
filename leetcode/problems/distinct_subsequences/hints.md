# Hints for Distinct Subsequences

1. Think of the problem as finding the number of ways to form string t by selecting characters from string s in order.

2. Consider using dynamic programming where dp[i][j] represents the number of distinct subsequences of s[0...i-1] that equal t[0...j-1].

3. For the base cases, an empty string t is a subsequence of any string s, and there's exactly one way to form it (by selecting no characters).

4. When processing each character, you have two choices:
   - If the current characters match (s[i-1] == t[j-1]), you can either use this character to match or skip it
   - If they don't match, you must skip the current character in s

5. Be careful with integer overflow, as the number of distinct subsequences can be very large.

6. The final answer is dp[m][n], where m and n are the lengths of s and t.

7. You can optimize the space complexity to O(n) by using a 1D DP array, as each row only depends on the previous row.

8. Consider using a recursive approach with memoization if you find it more intuitive.

9. The time complexity is O(m * n) where m and n are the lengths of strings s and t.

10. Test your solution with edge cases like empty strings, strings with repeated characters, and strings where t is not a subsequence of s.
