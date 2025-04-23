# Hints for Find Median from Data Stream

1. Think about using two heaps: a max-heap for the smaller half of the numbers and a min-heap for the larger half.

2. When adding a new number, decide which heap it should go into based on the current tops of the heaps.

3. After adding a number, you may need to rebalance the heaps to ensure their sizes differ by at most 1.

4. The median can be found by:
   - If both heaps have the same size, take the average of their tops.
   - If one heap has one more element, the median is the top of that heap.

5. In Rust, you can use `BinaryHeap` for the max-heap and `BinaryHeap<Reverse<i32>>` for the min-heap.

6. Be careful with integer overflow when calculating the average of two numbers.
