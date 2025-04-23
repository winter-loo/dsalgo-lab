# Approach

To efficiently find the median from a data stream, we need a data structure that allows for:
1. Fast insertion of new elements
2. Quick access to the middle element(s)

## Strategy
The key insight is to maintain two heaps:
- A max-heap for the smaller half of the numbers
- A min-heap for the larger half of the numbers

By balancing these heaps properly, we can efficiently find the median at any time.

## Steps
1. **Initialization**: Create an empty max-heap and an empty min-heap.

2. **Adding a Number**:
   - If the max-heap is empty or the number is less than the top of the max-heap, add it to the max-heap.
   - Otherwise, add it to the min-heap.
   - After insertion, balance the heaps if necessary to ensure that:
     - The size difference between the heaps is at most 1
     - The max-heap contains the smaller half of the numbers
     - The min-heap contains the larger half of the numbers

3. **Finding the Median**:
   - If the heaps have the same size, the median is the average of the tops of both heaps.
   - If the max-heap has one more element than the min-heap, the median is the top of the max-heap.
   - If the min-heap has one more element than the max-heap, the median is the top of the min-heap.

## Implementation Notes
- In Rust, we can use `BinaryHeap` for both heaps:
  - For the max-heap, we can use `BinaryHeap` directly as it is a max-heap by default.
  - For the min-heap, we can use `BinaryHeap<Reverse<i32>>` to create a min-heap.
- We need to carefully handle the balancing of heaps after each insertion.

## Time and Space Complexity
- **Time Complexity**:
  - `addNum`: O(log n) for heap operations
  - `findMedian`: O(1) to access the tops of the heaps
- **Space Complexity**: O(n) to store all elements in the heaps
