# Approach: Binary Search

## Intuition
The key insight for this problem is to leverage the sorted nature of the array to achieve a logarithmic time complexity. Instead of checking each element sequentially, we can eliminate half of the remaining elements with each comparison by comparing the middle element with our target value.

## Multiple Approaches

### Approach 1: Iterative Binary Search
1. Initialize two pointers, `left` and `right`, to the first and last indices of the array.
2. While `left <= right`, calculate the middle index `mid = left + (right - left) / 2`.
3. If `nums[mid] == target`, return `mid`.
4. If `nums[mid] < target`, set `left = mid + 1` to search the right half.
5. If `nums[mid] > target`, set `right = mid - 1` to search the left half.
6. If the loop terminates without finding the target, return `-1`.

**Time Complexity**: O(log n) - With each comparison, we eliminate half of the remaining elements.
**Space Complexity**: O(1) - We only use a constant amount of extra space.

### Approach 2: Recursive Binary Search
1. Define a recursive function that takes the array, target, left, and right indices.
2. Base case: If `left > right`, return `-1` (target not found).
3. Calculate the middle index `mid = left + (right - left) / 2`.
4. If `nums[mid] == target`, return `mid`.
5. If `nums[mid] < target`, recursively search the right half.
6. If `nums[mid] > target`, recursively search the left half.

**Time Complexity**: O(log n) - Same as the iterative approach.
**Space Complexity**: O(log n) - Due to the recursive call stack.

### Approach 3: Built-in Binary Search Functions
Many programming languages provide built-in binary search functions that can be used directly.

**Time Complexity**: O(log n) - These functions typically implement binary search.
**Space Complexity**: O(1) - Typically constant extra space.

## Implementation Notes
- Be careful with integer overflow when calculating the middle index. Use `mid = left + (right - left) / 2` instead of `mid = (left + right) / 2`.
- For the iterative approach, the condition should be `left <= right` to ensure we check all elements.
- For the recursive approach, make sure to handle the base case correctly.
- Binary search only works on sorted arrays. If the array is not sorted, you need to sort it first (which would change the time complexity to O(n log n)).
- Be mindful of edge cases like an empty array or when the target is not present in the array.

## Topics
- Array
- Binary Search
