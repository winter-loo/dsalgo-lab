# Approach

The "Longest Increasing Subsequence" (LIS) problem asks us to find the length of the longest subsequence in an array such that all elements of the subsequence are in strictly increasing order.

## Strategy

This problem can be solved using:
1. **Dynamic Programming**: O(n²) approach
2. **Binary Search with Patience Sort**: O(n log n) approach

## Dynamic Programming Approach (O(n²))
1. Create an array `dp` where `dp[i]` represents the length of the longest increasing subsequence ending at index `i`
2. Initialize all elements of `dp` to 1 (each element by itself is a subsequence of length 1)
3. For each position `i` from 1 to n-1:
   - For each position `j` from 0 to i-1:
     - If `nums[i] > nums[j]`, then `dp[i] = max(dp[i], dp[j] + 1)`
4. Return the maximum value in the `dp` array

## Binary Search Approach (O(n log n))
1. Create an array `tails` where `tails[i]` represents the smallest ending value of all increasing subsequences of length i+1
2. Initialize an empty `tails` array and a variable `size = 0`
3. For each element `num` in the array:
   - Use binary search to find the position `i` in `tails` such that `tails[i-1] < num <= tails[i]`
   - If `i == size` (no such position found), append `num` to `tails` and increment `size`
   - Otherwise, update `tails[i] = num`
4. Return `size` (the length of the `tails` array)

## Time and Space Complexity
- **Dynamic Programming**:
  - Time Complexity: O(n²)
  - Space Complexity: O(n)
- **Binary Search**:
  - Time Complexity: O(n log n)
  - Space Complexity: O(n)

## Pseudocode for Dynamic Programming Approach
```
function lengthOfLIS(nums):
    n = length(nums)
    dp = array of size n, initialized to 1
    
    for i from 1 to n-1:
        for j from 0 to i-1:
            if nums[i] > nums[j]:
                dp[i] = max(dp[i], dp[j] + 1)
    
    return maximum value in dp
```

## Pseudocode for Binary Search Approach
```
function lengthOfLIS(nums):
    n = length(nums)
    tails = empty array
    size = 0
    
    for num in nums:
        // Binary search to find the position to insert/update
        left = 0
        right = size
        
        while left < right:
            mid = (left + right) / 2
            if tails[mid] < num:
                left = mid + 1
            else:
                right = mid
        
        // Update tails array
        if left == size:
            tails.append(num)
            size++
        else:
            tails[left] = num
    
    return size
```
