# Approach: Find Minimum in Rotated Sorted Array

## Intuition
The key insight for this problem is to leverage the properties of a rotated sorted array. In a rotated sorted array, there exists a "pivot" point where the array is rotated. The minimum element is at this pivot point. We can use binary search to efficiently find this pivot point.

## Approaches

1. Initialize two pointers, `left` and `right`, to the first and last indices of the array.
2. While `left < right`:
   - Calculate the middle index `mid = left + (right - left) / 2`.
   - If `nums[mid] > nums[right]`, then the minimum element must be in the right half, so set `left = mid + 1`.
   - Otherwise, the minimum element must be in the left half (including `mid`), so set `right = mid`.
3. Return `nums[left]` (or `nums[right]`, they are the same at this point).

**Time Complexity**: O(log n) - Binary search reduces the search space by half in each iteration.
**Space Complexity**: O(1) - Constant extra space.


## Implementation Notes
- The key to solving this problem efficiently is to recognize that we can use binary search even though the array is not completely sorted.
- The comparison `nums[mid] > nums[right]` is crucial. If true, it means the pivot point (and thus the minimum element) is in the right half.
- Be careful with the termination condition of the binary search. We use `left < right` to ensure we find the exact pivot point.
- Edge cases to consider:
  - If the array is not rotated (i.e., already sorted), the minimum element is at index 0.
  - If the array has only one element, that element is the minimum.
- The problem guarantees that all elements are unique, which simplifies our approach.

## Topics
- Array
- Binary Search

## My Key Takeways

In your mind, imagine two line segments:

```
|
|    /
|   /  
|  /   
| /    
|        /
|       /
|_______________
```
or
```
|
|      /
|     /  
|    /   
|   /    
|  /
| /
|________________
```
