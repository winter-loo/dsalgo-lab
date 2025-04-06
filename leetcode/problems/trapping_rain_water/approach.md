# Approach: Trapping Rain Water

## Intuition
The key insight for this problem is understanding that the amount of water that can be trapped at any position depends on the minimum of the maximum heights to its left and right, minus the height at that position (if the result is positive).

## Multiple Approaches

### Approach 1: Brute Force
1. For each element in the array, find the maximum height to its left and right.
2. The water trapped at that element is min(maxLeft, maxRight) - height\[i\], if this value is positive.
3. Sum up the water trapped at each position.

**Time Complexity**: O(n²) - For each element, we traverse the array to find the maximum heights.
**Space Complexity**: O(1) - Constant extra space.

### Approach 2: Precomputation with Arrays
1. Precompute the maximum height to the left and right for each position.
2. Use these precomputed values to calculate the water trapped at each position.

**Time Complexity**: O(n) - We process each element three times (once for left max, once for right max, once for calculation).
**Space Complexity**: O(n) - We need two arrays of size n to store the left and right maximum heights.

### Approach 3: Two Pointers
1. Use two pointers, one from the left and one from the right.
2. Keep track of the maximum height from both sides.
3. If the left max is smaller, move the left pointer and calculate water for that position.
4. If the right max is smaller, move the right pointer and calculate water for that position.

**Time Complexity**: O(n) - We process each element exactly once.
**Space Complexity**: O(1) - Constant extra space.

### Approach 4: Stack-based
1. Use a stack to keep track of bars that can trap water.
2. When we find a bar that is higher than the top of the stack, we know we can trap water.
3. Calculate the water trapped and add it to the result.

**Time Complexity**: O(n) - Each bar is pushed and popped at most once.
**Space Complexity**: O(n) - In the worst case, the stack can contain all bars.

## Implementation Notes
- Be careful with edge cases like an empty array or an array with only one element.
- The two pointers approach is generally the most efficient in terms of both time and space complexity.
- The stack-based approach can be more intuitive for some people, especially those familiar with stack operations.
- Remember that water can only be trapped between two bars, so there must be a higher bar on both sides.
