# Approach

The "Min Cost Climbing Stairs" problem is a dynamic programming problem that builds on the classic climbing stairs problem.

## Strategy

This problem can be solved using:
1. **Dynamic Programming**: Build a solution from the bottom up
2. **Recursion with Memoization**: Top-down approach with caching

## Dynamic Programming Approach
1. Create an array `dp` where `dp[i]` represents the minimum cost to reach the top from position `i`
2. Initialize the base cases: 
   - `dp[n] = 0` (already at the top)
   - `dp[n-1] = cost[n-1]` (one step from the top)
3. For each step `i` from `n-2` down to `0`, calculate:
   - `dp[i] = cost[i] + min(dp[i+1], dp[i+2])`
4. Return the minimum of `dp[0]` and `dp[1]` (since we can start at either step)

## Recursion with Memoization
1. Use a recursive function that calculates the minimum cost to reach the top from any step
2. Use memoization to avoid recalculating the same subproblems
3. For each step, decide whether to take one step or two steps based on which path costs less

## Time and Space Complexity
- **Time Complexity**: O(n) where n is the length of the cost array
- **Space Complexity**: O(n) for the standard implementation, but can be optimized to O(1) by only keeping track of the last two values

## Pseudocode for Dynamic Programming Approach
```
function minCostClimbingStairs(cost):
    n = cost.length
    dp = array of size n+1
    dp[n] = 0
    dp[n-1] = cost[n-1]
    
    for i from n-2 down to 0:
        dp[i] = cost[i] + min(dp[i+1], dp[i+2])
    
    return min(dp[0], dp[1])
```

## Pseudocode for Optimized Space Approach
```
function minCostClimbingStairs(cost):
    n = cost.length
    next1 = 0  // Cost to reach top from the top
    next2 = cost[n-1]  // Cost to reach top from one step below
    
    for i from n-2 down to 0:
        current = cost[i] + min(next1, next2)
        next1 = next2
        next2 = current
    
    return min(next2, next1)  // Min cost starting from either step 0 or step 1
```
