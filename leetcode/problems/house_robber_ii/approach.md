# Approach

The "House Robber II" problem builds on the original House Robber problem, with the additional constraint that the houses are arranged in a circle, meaning the first and last houses are adjacent.

## Strategy

This problem can be solved by breaking it down into two subproblems:
1. Rob houses from 0 to n-2 (excluding the last house)
2. Rob houses from 1 to n-1 (excluding the first house)

Then take the maximum of these two scenarios. This approach works because:
- If we rob the first house, we cannot rob the last house
- If we rob the last house, we cannot rob the first house
- If we don't rob either the first or last house, then the result will be less than or equal to one of the above scenarios

## Dynamic Programming Approach
1. Create a helper function that solves the original House Robber problem (linear arrangement)
2. Call this helper function twice:
   - Once for the subarray excluding the last house
   - Once for the subarray excluding the first house
3. Return the maximum of these two results
4. Handle edge cases: if there's only one house, return its value

## Time and Space Complexity
- **Time Complexity**: O(n) where n is the number of houses
- **Space Complexity**: O(1) if we use the space-optimized version of the original House Robber solution

## Pseudocode
```
function rob(nums):
    n = nums.length
    if n == 0:
        return 0
    if n == 1:
        return nums[0]
    
    // Rob houses 0 to n-2 (excluding the last house)
    max1 = robLinear(nums, 0, n-2)
    
    // Rob houses 1 to n-1 (excluding the first house)
    max2 = robLinear(nums, 1, n-1)
    
    return max(max1, max2)

function robLinear(nums, start, end):
    // This is the solution to the original House Robber problem
    prev2 = 0  // Max amount if we consider no houses
    prev1 = 0  // Max amount if we consider one house
    
    for i from start to end:
        current = max(prev2 + nums[i], prev1)
        prev2 = prev1
        prev1 = current
    
    return prev1
```
