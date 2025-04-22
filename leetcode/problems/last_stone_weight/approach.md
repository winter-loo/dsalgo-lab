# Approach

To solve the Last Stone Weight problem, we need to repeatedly find the two heaviest stones, smash them together, and potentially add a new stone back to the collection.

## Strategy
- Use a max-heap (priority queue) to efficiently find the two heaviest stones in each step.
- Repeatedly extract the two heaviest stones, smash them, and insert the result back if necessary.
- Continue until there is at most one stone left.

## Steps
1. Create a max-heap from the input array of stone weights.
2. While the heap has at least two stones:
   - Extract the heaviest stone (y).
   - Extract the second heaviest stone (x).
   - If y > x, push y - x back into the heap.
3. If the heap is empty, return 0. Otherwise, return the weight of the remaining stone.

## Time and Space Complexity
- Time Complexity: O(n log n), where n is the number of stones.
  - Building the heap takes O(n) time.
  - Each extraction and insertion operation takes O(log n) time.
  - We perform at most n-1 smashing operations.
- Space Complexity: O(n) for the heap.

## Implementation Notes
- In Rust, we can use `BinaryHeap` which is a max-heap by default.
- We can directly use the heap operations to implement the stone smashing game.
