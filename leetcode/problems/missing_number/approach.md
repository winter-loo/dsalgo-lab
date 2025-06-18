# Approach

The "Missing Number" problem asks us to find the missing number in an array containing n distinct numbers in the range [0, n].

## Understanding the Problem

We have an array of n distinct numbers in the range [0, n], which means there should be n+1 numbers in total (0 to n inclusive). However, the array only contains n numbers, so one number is missing. We need to find that missing number.

## Approaches

### 1. Sum Approach

One approach is to use the formula for the sum of the first n natural numbers: `n * (n + 1) / 2`. This gives us the expected sum of all numbers from 0 to n. We can then subtract the actual sum of the numbers in the array to find the missing number.

**Time Complexity**: O(n)
**Space Complexity**: O(1)

### 2. XOR Approach

Another approach is to use the XOR operation. If we XOR all numbers from 0 to n and all numbers in the array, the missing number will be the result. This works because:
- XOR of a number with itself is 0.
- XOR of a number with 0 is the number itself.
- XOR is commutative and associative.

**Time Complexity**: O(n)
**Space Complexity**: O(1)

### 3. Binary Search Approach

If the array is sorted, we can use binary search to find the missing number. The key insight is that if all numbers are present, then nums[i] should equal i. The first index i where nums[i] != i indicates that i is missing.

**Time Complexity**: O(n log n) due to sorting
**Space Complexity**: O(1) or O(n) depending on the sorting algorithm

## Pseudocode for Sum Approach

```
function missingNumber(nums):
    n = length(nums)
    expectedSum = n * (n + 1) / 2
    actualSum = sum(nums)
    return expectedSum - actualSum
```

## Pseudocode for XOR Approach

```
function missingNumber(nums):
    n = length(nums)
    result = n  // XOR with n first
    
    for i from 0 to n-1:
        result = result XOR i XOR nums[i]
    
    return result
```

## Step-by-Step Example

Let's trace through the sum approach with the example: nums = [3,0,1]

1. n = 3
2. expectedSum = 3 * (3 + 1) / 2 = 6
3. actualSum = 3 + 0 + 1 = 4
4. missingNumber = expectedSum - actualSum = 6 - 4 = 2

Let's trace through the XOR approach with the same example:

1. n = 3
2. Initialize result = 3
3. Iterate through the array:
   - i = 0: result = 3 XOR 0 XOR 3 = 0
   - i = 1: result = 0 XOR 1 XOR 0 = 1
   - i = 2: result = 1 XOR 2 XOR 1 = 2
4. Return result = 2

## Alternative Approaches

### Hash Set Approach

We can also use a hash set to solve this problem:
1. Add all numbers in the array to a hash set.
2. Iterate from 0 to n and check if each number is in the hash set.
3. The first number that is not in the hash set is the missing number.

**Time Complexity**: O(n)
**Space Complexity**: O(n)

### In-place Marking Approach

If we're allowed to modify the input array, we can mark the presence of each number by negating the value at the corresponding index:
1. For each number x in the array, negate the value at index |x| if it's within bounds.
2. Iterate through the array again and find the first index with a positive value.
3. If all values are negative, then n is the missing number.

**Time Complexity**: O(n)
**Space Complexity**: O(1)

However, this approach modifies the input array, which may not be allowed.
