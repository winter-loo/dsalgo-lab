# Hints for Kth Largest Element in a Stream

1. Think about what data structure would be most efficient for finding the kth largest element as new elements are added.

2. A min-heap (priority queue) of size k can efficiently maintain the k largest elements seen so far.

3. The smallest element in the min-heap will be the kth largest element in the stream.

4. When adding a new element to the stream:
   - If the heap size is less than k, simply add the element.
   - If the heap size is already k, compare the new element with the smallest element in the heap.
   - If the new element is larger, remove the smallest element and add the new one.

5. In Rust, you can use `BinaryHeap` with the `Reverse` wrapper to create a min-heap.
