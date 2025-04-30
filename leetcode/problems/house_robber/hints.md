# Hints for House Robber

1. Think of the problem as a series of decisions: for each house, you need to decide whether to rob it or skip it.

2. If you rob a house, you cannot rob the adjacent houses (the ones immediately before and after it).

3. This is a perfect fit for dynamic programming, where each state depends on previous decisions.

4. Consider using an array where dp[i] represents the maximum amount of money you can rob up to house i.

5. For each house, you have two options:
   - Rob the current house and add its value to the maximum amount robbed up to two houses ago
   - Skip the current house and take the maximum amount robbed up to the previous house

6. The recurrence relation is: dp[i] = max(dp[i-2] + nums[i], dp[i-1])

7. Be careful with the base cases, especially when handling arrays with only one or two elements.

8. For optimization, notice that you only need to keep track of the maximum amounts for the previous two houses at each step, not the entire array.
