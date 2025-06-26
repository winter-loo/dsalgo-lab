# Approach

The "Burst Balloons" problem asks us to find the maximum coins we can
collect by bursting balloons, where bursting the ith balloon gives us
`nums[i-1] * nums[i] * nums[i+1]` coins.

## Strategy

This problem can be solved using:
1. **Dynamic Programming**: Build a solution using a 2D DP table
2. **Divide and Conquer with Memoization**: Consider the last balloon to burst in each subproblem

## Dynamic Programming Approach
1. The key insight is to think about the problem in reverse: instead of
   considering which balloon to burst first, consider which balloon to burst
   last
2. Create a 2D DP table where `dp[i][j]` represents the maximum coins we can get
   by bursting all balloons between indices i and j (inclusive)
3. For each subproblem `dp[i][j]`:
   - Consider each balloon k between i and j as the last one to burst
   - The coins we get are: `nums[i-1] * nums[k] * nums[j+1] + dp[i][k-1] + dp[k+1][j]`
4. Return `dp[1][n]` where n is the number of balloons

## Time and Space Complexity
- **Time Complexity**: O(n³) where n is the number of balloons
  - We have O(n²) subproblems, and for each subproblem, we consider O(n) possibilities
- **Space Complexity**: O(n²) for the DP table

## Pseudocode for Dynamic Programming
```
function maxCoins(nums):
    n = length of nums
    
    // Add 1 at the beginning and end of the array
    newNums = new array of size n+2
    newNums[0] = newNums[n+1] = 1
    for i from 1 to n:
        newNums[i] = nums[i-1]
    
    // Create DP table
    dp = 2D array of size (n+2) x (n+2), initialized to 0
    
    // Fill the DP table
    for len from 1 to n:
        for i from 1 to n-len+1:
            j = i + len - 1
            for k from i to j:
                dp[i][j] = max(dp[i][j], 
                              dp[i][k-1] + dp[k+1][j] + newNums[i-1] * newNums[k] * newNums[j+1])
    
    return dp[1][n]
```

## Alternative Approach: Recursive with Memoization
1. Use a recursive function that computes the maximum coins for the subarray `nums[left...right]`
2. For each position `k` between left and right, consider it as the last balloon to burst
3. Use memoization to avoid recalculating the same subproblems

This approach is more intuitive but may have higher overhead due to the recursion stack.
