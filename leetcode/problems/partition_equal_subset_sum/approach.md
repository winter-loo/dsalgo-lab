# Approach

The "Partition Equal Subset Sum" problem asks us to determine if an array can be partitioned into two subsets with equal sums.

## Strategy

This problem can be reduced to a "Subset Sum" problem:
1. Calculate the total sum of the array
2. If the total sum is odd, return false (we can't partition into two equal subsets)
3. If the total sum is even, check if there exists a subset with sum equal to totalSum/2

The subset sum problem can be solved using:
1. **Dynamic Programming**: Build a solution using a 2D or 1D DP table
2. **Recursion with Memoization**: Top-down approach with caching

## Dynamic Programming Approach
1. Calculate the total sum of the array
2. If the total sum is odd, return false
3. Create a boolean DP table where `dp[i][j]` represents whether a subset of the first `i` elements can sum up to `j`
4. Initialize the first column to true (empty subset sums to 0)
5. For each element and each possible sum:
   - If we don't include the current element: `dp[i][j] = dp[i-1][j]`
   - If we include the current element: `dp[i][j] = dp[i-1][j-nums[i-1]]` (if j >= nums[i-1])
6. Return `dp[n][totalSum/2]`

## Space-Optimized Dynamic Programming
1. Since we only need the previous row to calculate the current row, we can optimize space to O(target)
2. Use a 1D DP array where `dp[j]` represents whether a subset can sum up to `j`
3. Iterate through the array backwards to avoid using the same element multiple times

## Time and Space Complexity
- **Time Complexity**: O(n * target) where n is the length of the array and target is totalSum/2
- **Space Complexity**: 
  - 2D DP: O(n * target)
  - 1D DP: O(target)

## Pseudocode for Dynamic Programming Approach (2D)
```
function canPartition(nums):
    totalSum = sum(nums)
    if totalSum % 2 != 0:
        return false
    
    target = totalSum / 2
    n = length(nums)
    dp = boolean table of size (n+1) x (target+1)
    
    // Empty subset sums to 0
    for i from 0 to n:
        dp[i][0] = true
    
    for i from 1 to n:
        for j from 1 to target:
            // Don't include current element
            dp[i][j] = dp[i-1][j]
            
            // Include current element if possible
            if j >= nums[i-1]:
                dp[i][j] = dp[i][j] || dp[i-1][j-nums[i-1]]
    
    return dp[n][target]
```

## Pseudocode for Space-Optimized Dynamic Programming (1D)
```
function canPartition(nums):
    totalSum = sum(nums)
    if totalSum % 2 != 0:
        return false
    
    target = totalSum / 2
    dp = boolean array of size target+1, initialized to false
    dp[0] = true
    
    for num in nums:
        for j from target down to num:
            dp[j] = dp[j] || dp[j-num]
    
    return dp[target]
```
