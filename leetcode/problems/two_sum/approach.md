# Approach: Two Sum

## Intuition
The key insight for this problem is to efficiently find pairs of numbers in the array that sum to the target value. While a brute force approach would check every possible pair, we can optimize this using a hash map to achieve O(n) time complexity.

## Multiple Approaches

### Approach 1: Brute Force
1. For each element in the array, check every other element to see if they sum to the target.
2. Return the indices of the two elements if found.

**Time Complexity**: O(n²) - For each element, we check all other elements.
**Space Complexity**: O(1) - Constant extra space.

### Approach 2: Hash Map (One-pass)
1. Use a hash map to store elements we've seen so far, with their indices.
2. For each element, check if the complement (target - current element) exists in the hash map.
3. If it does, we've found our pair. Return the current index and the index stored in the hash map.
4. Otherwise, add the current element and its index to the hash map and continue.

**Time Complexity**: O(n) - We process each element exactly once.
**Space Complexity**: O(n) - In the worst case, the hash map stores n elements.

### Approach 3: Hash Map (Two-pass)
1. First pass: Add all elements and their indices to a hash map.
2. Second pass: For each element, check if the complement exists in the hash map.
3. Make sure not to use the same element twice (check if the indices are different).

**Time Complexity**: O(n) - We process each element twice.
**Space Complexity**: O(n) - The hash map stores n elements.

## Implementation Notes
- The one-pass hash map approach is generally the most efficient in terms of both time and space complexity.
- Be careful with edge cases like duplicate elements (e.g., [3,3] with target 6).
- Remember that the problem guarantees exactly one valid solution.
- When checking for the complement in the hash map, ensure you're not using the same element twice.
