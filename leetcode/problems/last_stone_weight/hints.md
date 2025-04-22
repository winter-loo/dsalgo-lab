# Hints for Last Stone Weight

1. Think about what data structure would allow you to efficiently find and remove the two heaviest stones in each step.

2. A max-heap (priority queue) is perfect for this problem as it allows O(log n) extraction of the maximum element.

3. The algorithm is straightforward:
   - Create a max-heap from the input array.
   - Repeatedly extract the two heaviest stones, smash them, and insert the result back if necessary.
   - Continue until there is at most one stone left.

4. In Rust, you can use the `BinaryHeap` collection which is a max-heap by default.

5. Be careful with the edge cases:
   - If there's only one stone initially, return its weight.
   - If all stones are destroyed, return 0.
