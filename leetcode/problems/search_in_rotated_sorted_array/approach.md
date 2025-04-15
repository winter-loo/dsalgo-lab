# Approach: Search in Rotated Sorted Array

## Intuition
The key insight for this problem is to adapt the binary search algorithm to work with a rotated sorted array. In a standard binary search, we compare the middle element with the target and decide which half to search next. In a rotated sorted array, we need to first determine which half is sorted and then check if the target lies within that sorted half.

## Multiple Approaches

### Approach 1: Modified Binary Search
1. Initialize two pointers, `left` and `right`, to the first and last indices of the array.
2. While `left <= right`:
   - Calculate the middle index `mid = left + (right - left) / 2`.
   - If `nums[mid] == target`, return `mid`.
   - Determine which half is sorted:
     - If `nums[left] <= nums[mid]`, the left half is sorted.
       - If `target >= nums[left]` and `target < nums[mid]`, search in the left half by setting `right = mid - 1`.
       - Otherwise, search in the right half by setting `left = mid + 1`.
     - If `nums[mid] <= nums[right]`, the right half is sorted.
       - If `target > nums[mid]` and `target <= nums[right]`, search in the right half by setting `left = mid + 1`.
       - Otherwise, search in the left half by setting `right = mid - 1`.
3. If the loop terminates without finding the target, return `-1`.

**Time Complexity**: O(log n) - Binary search reduces the search space by half in each iteration.
**Space Complexity**: O(1) - Constant extra space.

### Approach 2: Find Pivot, Then Binary Search
1. Find the pivot index (the index where the rotation occurs) using binary search.
2. Once the pivot is found, determine which half of the array contains the target.
3. Perform a standard binary search on that half.

**Time Complexity**: O(log n) - Two binary searches, each taking O(log n) time.
**Space Complexity**: O(1) - Constant extra space.

## Implementation Notes
- The key to solving this problem efficiently is to recognize which half of the array is sorted and then check if the target lies within that sorted half.
- Be careful with the boundary conditions when checking if the target lies within a range.
- Edge cases to consider:
  - If the array is not rotated (i.e., already sorted), the standard binary search will work.
  - If the array has only one element, check if it matches the target.
- The problem guarantees that all elements are unique, which simplifies our approach.
- When implementing the binary search, be careful with the termination condition and the updates to `left` and `right`.

## Topics
- Array
- Binary Search
