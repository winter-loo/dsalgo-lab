# Hints for Combination Sum II

1. This problem is similar to Combination Sum, but with two key differences:
   - Each number can only be used once in a combination
   - The input may contain duplicate numbers

2. Sort the candidates array first. This helps in handling duplicates and makes the backtracking process more efficient.

3. Use a backtracking approach to explore all possible combinations systematically.

4. To avoid using the same number multiple times, increment the index in the recursive call.

5. To avoid duplicate combinations, skip duplicates at the same level of recursion. After sorting, check if the current candidate is the same as the previous one at the same level.

6. The base cases for your recursion are:
   - When the current sum equals the target (add the combination to the result)
   - When the current sum exceeds the target (backtrack)
   - When you've considered all candidates (backtrack)

7. Keep track of the current combination and its sum during the backtracking process.

8. Consider using early pruning: if the current sum plus the smallest remaining candidate exceeds the target, you can stop exploring that path.
