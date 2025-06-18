# Hints for Regular Expression Matching

1. Think of the problem as determining whether the first i characters of string s match the first j characters of pattern p.

2. Consider using dynamic programming where dp[i][j] represents whether the first i characters of s match the first j characters of p.

3. Handle the base cases carefully: empty string matches empty pattern, and pattern with '*' can match empty string by ignoring the preceding element.

4. When processing a '*' character, there are two possibilities:
   - Ignore the '*' and its preceding element (use 0 occurrences)
   - Use the '*' to match one or more characters (if the current character in s matches the preceding element in p)

5. For regular characters or '.', check if the current characters match and if the previous substrings match.

6. Be careful with the indices when implementing the solution, especially when handling the '*' character.

7. Remember that '.' matches any single character, so s[i-1] matches p[j-1] if p[j-1] is '.' or if they are the same character.

8. The time complexity is O(m * n) where m and n are the lengths of the string and pattern.

9. Consider using recursion with memoization as an alternative approach if you find it more intuitive.

10. Test your solution with edge cases like empty strings, patterns with consecutive '*' characters, and patterns that should match the entire string.
