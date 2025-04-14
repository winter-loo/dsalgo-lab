# Approach: Largest Rectangle in Histogram

## Intuition
The key insight for this problem is to find, for each bar, the maximum rectangle that can be formed with that bar as the shortest bar.
To do this efficiently, we need to determine how far left and right we can extend from each bar while maintaining a height at least as tall as the current bar.

## Multiple Approaches

### Approach 1: Brute Force
1. For each bar, consider it as the minimum height bar of a rectangle.
2. Extend to the left and right as far as possible while maintaining a height at least as tall as the current bar.
3. Calculate the area of this rectangle and keep track of the maximum area found.

**Time Complexity**: O(n²) - For each bar, we potentially scan the entire array.
**Space Complexity**: O(1) - Constant extra space.

### Approach 2: Using Stack (Optimal)
1. Use a monotonic stack to keep track of indices of bars in increasing order of height.
2. When we encounter a bar shorter than the top of the stack, we pop the stack and calculate the area of the rectangle with the popped bar as the shortest bar.
3. The width of this rectangle is determined by the current position and the next element in the stack.
4. Continue this process for all bars in the histogram.

**Time Complexity**: O(n) - Each bar is pushed and popped at most once.
**Space Complexity**: O(n) - In the worst case, the stack can contain all bars.

### Approach 3: Using Left and Right Boundaries
1. For each bar, pre-compute the first smaller bar to its left and right.
2. Use these boundaries to calculate the maximum rectangle with the current bar as the shortest.
3. Take the maximum of all such rectangles.

**Time Complexity**: O(n) - We process each element a constant number of times.
**Space Complexity**: O(n) - We use arrays to store the left and right boundaries.

## Implementation Notes
- The stack-based approach is generally the most efficient in terms of both time and space complexity.
- Be careful with edge cases like empty arrays or arrays with a single element.
- When using the stack approach, remember to handle the case when the stack becomes empty.
- Consider adding sentinel values (like -1 for left boundary and n for right boundary) to simplify the implementation.

## Topics
- Array
- Stack
- Monotonic Stack
