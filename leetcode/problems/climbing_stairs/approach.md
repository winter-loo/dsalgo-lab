# Approach

The "Climbing Stairs" problem is a classic dynamic programming problem that can be approached in multiple ways.

## Strategy

This problem can be solved using:
1. **Dynamic Programming**: Build a solution from the bottom up
2. **Recursion with Memoization**: Top-down approach with caching
3. **Mathematical Formula**: Using the Fibonacci sequence

## Dynamic Programming Approach
1. Create an array `dp` where `dp[i]` represents the number of ways to climb to the `i`th step
2. Initialize base cases: `dp[1] = 1` (one way to climb 1 step) and `dp[2] = 2` (two ways to climb 2 steps)
3. For each step `i` from 3 to n, calculate `dp[i] = dp[i-1] + dp[i-2]` (we can reach the current step either from one step below or two steps below)
4. Return `dp[n]`

## Recursion with Memoization
1. Use a recursive function that calculates the number of ways to climb from any step
2. Use memoization to avoid recalculating the same subproblems
3. Base cases: 1 way to climb 0 steps, 1 way to climb 1 step

## Mathematical Approach
1. Observe that the solution follows the Fibonacci sequence
2. The number of ways to climb n stairs is the (n+1)th Fibonacci number
3. This can be calculated directly using the Fibonacci formula

## Time and Space Complexity
- **Time Complexity**: O(n) for the dynamic programming and memoization approaches
- **Space Complexity**: O(n) for the standard implementations, but can be optimized to O(1) by only keeping track of the last two values

## Pseudocode for Dynamic Programming Approach
```
function climbStairs(n):
    if n <= 2:
        return n
    
    dp = array of size n+1
    dp[1] = 1
    dp[2] = 2
    
    for i from 3 to n:
        dp[i] = dp[i-1] + dp[i-2]
    
    return dp[n]
```

## Pseudocode for Optimized Space Approach
```
function climbStairs(n):
    if n <= 2:
        return n
    
    prev1 = 1  // Ways to climb 1 step
    prev2 = 2  // Ways to climb 2 steps
    
    for i from 3 to n:
        current = prev1 + prev2
        prev1 = prev2
        prev2 = current
    
    return prev2
```
