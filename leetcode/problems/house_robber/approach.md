# Approach

The "House Robber" problem is a classic dynamic programming problem where we need to find the maximum amount of money that can be robbed without alerting the police (i.e., without robbing adjacent houses).

## Strategy

This problem can be solved using:
1. **Dynamic Programming**: Build a solution from the bottom up
2. **Recursion with Memoization**: Top-down approach with caching

## Dynamic Programming Approach
1. Create an array `dp` where `dp[i]` represents the maximum amount of money that can be robbed up to house `i`
2. For each house, we have two options:
   - Rob the current house and add its value to the maximum amount robbed up to two houses ago
   - Skip the current house and take the maximum amount robbed up to the previous house
3. The recurrence relation is: `dp[i] = max(dp[i-2] + nums[i], dp[i-1])`
4. Initialize the base cases: 
   - `dp[0] = nums[0]` (only one house to rob)
   - `dp[1] = max(nums[0], nums[1])` (maximum of first two houses)

## Recursion with Memoization
1. Use a recursive function that calculates the maximum amount that can be robbed starting from any house
2. Use memoization to avoid recalculating the same subproblems
3. For each house, decide whether to rob it or skip it based on which choice maximizes the total amount

## Time and Space Complexity
- **Time Complexity**: O(n) where n is the number of houses
- **Space Complexity**: O(n) for the standard implementation, but can be optimized to O(1) by only keeping track of the last two values

## Pseudocode for Dynamic Programming Approach
```
function rob(nums):
    n = nums.length
    if n == 0:
        return 0
    if n == 1:
        return nums[0]
    
    dp = array of size n
    dp[0] = nums[0]
    dp[1] = max(nums[0], nums[1])
    
    for i from 2 to n-1:
        dp[i] = max(dp[i-2] + nums[i], dp[i-1])
    
    return dp[n-1]
```

## Pseudocode for Optimized Space Approach
```
function rob(nums):
    n = nums.length
    if n == 0:
        return 0
    if n == 1:
        return nums[0]
    
    prev2 = nums[0]  // Max amount if we rob only the first house
    prev1 = max(nums[0], nums[1])  // Max amount if we consider the first two houses
    
    for i from 2 to n-1:
        current = max(prev2 + nums[i], prev1)
        prev2 = prev1
        prev1 = current
    
    return prev1
```
