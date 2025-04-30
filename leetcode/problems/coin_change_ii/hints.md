# Hints for Coin Change II

1. This problem is asking for the number of different combinations that can make up a given amount, not the minimum number of coins.

2. Think of it as a counting problem rather than an optimization problem.

3. Consider using dynamic programming where dp[i] represents the number of ways to make amount i.

4. The base case is dp[0] = 1, as there is exactly one way to make amount 0 (by not using any coin).

5. For each coin and each amount, you have a choice: either use the coin or don't use it.

6. Be careful about the order of the loops: to avoid counting the same combination multiple times, iterate through the coins first, then through the amounts.

7. For each coin and each amount j, if j >= coin, then dp[j] += dp[j - coin].

8. This approach ensures that we're counting combinations (order doesn't matter) rather than permutations (order matters).

9. If you're using a 2D DP approach, dp[i][j] would represent the number of ways to make amount j using the first i types of coins.

10. The final answer is dp[amount], which gives the total number of ways to make up the target amount.
