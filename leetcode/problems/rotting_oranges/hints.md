# Hints for Rotting Oranges

1. This problem asks you to find the minimum number of minutes until all fresh oranges become rotten, or determine if it's impossible.

2. Consider using Breadth-First Search (BFS) to simulate the rotting process layer by layer, where each layer represents one minute of time.

3. Start by identifying all initially rotten oranges and count the total number of fresh oranges.

4. Use a queue to keep track of rotten oranges. Initially, add all rotten oranges to the queue.

5. For each minute, process all rotten oranges in the current queue:
   - For each rotten orange, check its four adjacent cells (up, down, left, right).
   - If an adjacent cell contains a fresh orange, make it rotten, decrease the fresh orange count, and add it to the queue.

6. After processing all oranges at the current minute, increment the minute count.

7. Continue until the queue is empty (no more oranges can rot).

8. If there are still fresh oranges left, return -1 (impossible to rot all oranges).

9. Otherwise, return the total minutes elapsed.

10. Be careful with edge cases:
    - If there are no fresh oranges initially, return 0.
    - If there are fresh oranges but no rotten oranges initially, return -1.
    - If there are fresh oranges that cannot be reached by any rotten orange, return -1.
