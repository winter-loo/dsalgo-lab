# Approach

The "Target Sum" problem asks us to find the number of different ways to assign `+` and `-` signs to the numbers in an array such that the sum equals a target value.

## Strategy

This problem can be solved using:
1. **Dynamic Programming**: Transform it into a subset sum problem
2. **Recursion with Memoization**: Top-down approach with caching
3. **Depth-First Search (DFS)**: Explore all possible combinations

## Dynamic Programming Approach
1. Observe that the problem can be transformed into a subset sum problem:
   - Let P be the subset of numbers with a + sign, and N be the subset with a - sign
   - We know that P + N = sum(nums) and P - N = target
   - From these equations: P = (sum(nums) + target) / 2
2. So the problem reduces to: find the number of subsets of nums that sum to (sum(nums) + target) / 2
3. Create a DP array where `dp[j]` represents the number of ways to make sum `j`
4. Initialize `dp[0] = 1` (there's 1 way to make sum 0 - by not selecting any number)
5. For each number and each possible sum:
   - Update `dp[j] += dp[j-num]` if j >= num
6. Return `dp[P]`

## Recursion with Memoization
1. Define a recursive function that tries both + and - for each number
2. Use memoization to avoid recalculating the same subproblems
3. Base case: if we've processed all numbers, check if the current sum equals the target

## Time and Space Complexity
- **Dynamic Programming**:
  - Time Complexity: O(n * sum) where n is the length of the array and sum is the sum of all numbers
  - Space Complexity: O(sum)
- **Recursion with Memoization**:
  - Time Complexity: O(n * sum) where sum is the sum of all numbers
  - Space Complexity: O(n * sum) for the memoization table

## Pseudocode for Dynamic Programming Approach
```
function findTargetSumWays(nums, target):
    total = sum(nums)
    
    // Check if it's possible to achieve the target
    if total < abs(target) or (total + target) % 2 != 0:
        return 0
    
    // Calculate the subset sum we need to find
    subsetSum = (total + target) / 2
    
    dp = array of size subsetSum+1, initialized to 0
    dp[0] = 1
    
    for num in nums:
        for j from subsetSum down to num:
            dp[j] += dp[j-num]
    
    return dp[subsetSum]
```

## Pseudocode for Recursion with Memoization
```
function findTargetSumWays(nums, target):
    memo = empty map
    return dfs(nums, target, 0, 0, memo)

function dfs(nums, target, index, currentSum, memo):
    // Create a unique key for the current state
    key = (index, currentSum)
    
    if key in memo:
        return memo[key]
    
    if index == length(nums):
        if currentSum == target:
            return 1
        else:
            return 0
    
    // Try adding the current number
    positive = dfs(nums, target, index+1, currentSum + nums[index], memo)
    
    // Try subtracting the current number
    negative = dfs(nums, target, index+1, currentSum - nums[index], memo)
    
    memo[key] = positive + negative
    return memo[key]
```
