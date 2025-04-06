# Approach: Two Sum II - Input Array Is Sorted

## Intuition
The key insight for this problem is to leverage the fact that the input array is already sorted in non-decreasing order. This allows us to use more efficient approaches than the hash map approach used in the original Two Sum problem.

## Multiple Approaches

### Approach 1: Two Pointers
1. Initialize two pointers, one at the beginning of the array (left) and one at the end (right).
2. Calculate the sum of the elements at the left and right pointers.
3. If the sum equals the target, return the indices (added by one as per the problem requirement).
4. If the sum is less than the target, move the left pointer to the right (to increase the sum).
5. If the sum is greater than the target, move the right pointer to the left (to decrease the sum).
6. Continue until the two pointers meet or the target is found.

**Time Complexity**: O(n) - In the worst case, we might need to scan the entire array once.
**Space Complexity**: O(1) - We only use two pointers, regardless of the input size.

### Approach 2: Binary Search
1. For each element in the array, use binary search to find if there exists another element that, when added to the current element, equals the target.
2. Return the indices (added by one) if such a pair is found.

**Time Complexity**: O(n log n) - For each of the n elements, we perform a binary search which takes O(log n) time.
**Space Complexity**: O(1) - We only use a constant amount of extra space.

## Implementation Notes
- Remember that the array is 1-indexed in the problem, but 0-indexed in most programming languages. Adjust indices accordingly.
- The two pointers approach is generally more efficient than the binary search approach for this problem.
- Be careful with edge cases like an array with only two elements.
- Since the problem guarantees exactly one solution, we don't need to handle cases where no solution exists.
