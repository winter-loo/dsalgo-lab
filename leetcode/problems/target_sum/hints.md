# Hints for Target Sum

1. The problem asks for the number of ways to assign + and - signs to numbers in an array to achieve a target sum.

2. One approach is to use recursion with memoization, where for each number, you explore both possibilities: adding it and subtracting it.

3. Another approach is to transform this problem into a subset sum problem:
   - Let P be the subset of numbers with a + sign, and N be the subset with a - sign
   - We know that P + N = sum(nums) and P - N = target
   - From these equations: P = (sum(nums) + target) / 2

4. So the problem reduces to: find the number of subsets of nums that sum to (sum(nums) + target) / 2.

5. Check if it's possible to achieve the target: if total < abs(target) or (total + target) % 2 != 0, return 0.

6. Use dynamic programming with a 1D array where dp[j] represents the number of ways to make sum j.

7. Initialize dp[0] = 1 (there's 1 way to make sum 0 - by not selecting any number).

8. For each number and each possible sum, update dp[j] += dp[j-num] if j >= num.

9. Be careful with the order of the loops to avoid counting the same combination multiple times.

10. The final answer is dp[subsetSum], where subsetSum = (total + target) / 2.
