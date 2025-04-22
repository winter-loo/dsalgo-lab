# Approach

To find the k closest points to the origin, we need to calculate the distance of each point from the origin and select the k points with the smallest distances.

## Strategy
- Calculate the Euclidean distance of each point from the origin (0, 0).
- Sort the points based on their distances or use a max-heap of size k to efficiently find the k closest points.

## Steps
1. For each point [x, y], calculate its distance from the origin: d = x² + y².
   - Note: We can use the squared distance instead of the actual Euclidean distance (√(x² + y²)) for comparison, as it preserves the ordering and avoids the square root calculation.
2. Use one of the following approaches:
   - Sort all points by their distances and return the first k points.
   - Use a max-heap of size k to keep track of the k closest points seen so far.
   - Use the QuickSelect algorithm to find the k closest points in O(n) average time.

## Implementation Options
- **Sorting Approach**: Simple to implement but has O(n log n) time complexity.
- **Max-Heap Approach**: Maintains a heap of size k, with O(n log k) time complexity.
- **QuickSelect Approach**: More complex to implement but has O(n) average time complexity.

// TODO: Add time and space complexity analysis.
