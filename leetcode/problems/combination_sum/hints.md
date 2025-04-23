# Hints for Combination Sum

1. This problem is best solved using a backtracking approach, where you explore all possible combinations systematically.

2. Remember that the same number can be used multiple times. This means when you choose a number, you can consider it again for the next position.

3. To avoid duplicate combinations, consider candidates in a specific order. For example, only consider candidates at or after the current index in each recursive call.

4. You can optimize your solution by stopping exploration when the current sum exceeds the target.

5. Consider sorting the candidates array first, which can help with early pruning of paths that would exceed the target.

6. Keep track of the current combination and its sum during the backtracking process.

7. The base cases for your recursion are:
   - When the current sum equals the target (add the combination to the result)
   - When the current sum exceeds the target (backtrack)
   - When you've considered all candidates (backtrack)
