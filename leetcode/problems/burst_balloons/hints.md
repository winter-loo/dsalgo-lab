# Hints for Burst Balloons

1. Think about the problem in reverse: instead of deciding which balloon to burst first, consider which balloon to burst last.

2. Use dynamic programming where `dp[i][j]` represents the maximum coins you can get by bursting all balloons between indices i and j (inclusive).

3. For each subproblem `dp[i][j]`, consider each balloon k between i and j as the last one to burst.

4. When balloon `k` is the last to burst, the coins you get are: `nums[i-1] * nums[k] * nums[j+1] + dp[i][k-1] + dp[k+1][j]`.

5. Handle the boundary cases by adding 1 at the beginning and end of the array (representing the implicit balloons outside the array).

6. Build the DP table bottom-up, starting with smaller subarrays and gradually increasing the size.

7. The final answer is `dp[1][n]` where n is the number of balloons.

8. Be careful with the indices when implementing the solution, especially when adding the boundary balloons.

9. The time complexity is O(n³) where n is the number of balloons, as we have O(n²) subproblems and for each subproblem, we consider O(n) possibilities.

10. Consider using recursion with memoization as an alternative approach if you find it more intuitive.
