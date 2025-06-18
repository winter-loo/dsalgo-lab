# Hints for Valid Parenthesis String

1. Remember that the '*' character can be treated as '(', ')', or an empty string.

2. Consider tracking a range of possible values for the balance of open parentheses.

3. The minimum balance occurs when all '*' are treated as ')' or empty strings.

4. The maximum balance occurs when all '*' are treated as '(' or empty strings.

5. If at any point the maximum balance becomes negative, the string is invalid (too many closing parentheses).

6. If the minimum balance goes below zero, we can reset it to zero (by treating some '*' as '(' to balance).

7. At the end, the string is valid if the minimum balance can be zero (all opening parentheses have matching closing ones).

8. Consider edge cases like empty strings, strings with only '*' characters, or strings with unbalanced parentheses.

9. Another approach is to use two separate passes: one from left to right and another from right to left.

10. You can also solve this using dynamic programming, but the greedy approach with min-max balance is more efficient.
