# Approach

To find the kth largest element in a stream, we need an efficient data structure that can maintain the k largest elements as new values are added.

## Strategy
- Use a min-heap (priority queue) to store only the k largest elements.
- The root of the min-heap will always be the kth largest element.
- When a new element is added to the stream, we need to decide whether it should be included in our k largest elements.

## Steps
1. Initialize a min-heap with a capacity of k.
2. Add all elements from the initial array to the heap.
3. If the heap size exceeds k, remove the smallest element (the root of the min-heap).
4. For each new element added to the stream:
   - Add it to the heap.
   - If the heap size exceeds k, remove the smallest element.
   - The root of the heap is the kth largest element.

## Time and Space Complexity
- Time Complexity:
  - Constructor: O(n log k) where n is the size of the initial array.
  - Add method: O(log k) for heap operations.
- Space Complexity: O(k) to store the k largest elements.

## Implementation Notes
- In Rust, we can use `BinaryHeap` with `Reverse` wrapper to create a min-heap.
- We only need to keep track of the k largest elements, which makes this approach efficient for large streams.
