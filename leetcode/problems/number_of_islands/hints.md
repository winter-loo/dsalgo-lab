# Hints for Number of Islands

1. This problem is about finding connected components in a grid, where connections are defined by adjacent cells (horizontally or vertically) with value '1'.

2. Consider using either Depth-First Search (DFS) or Breadth-First Search (BFS) to explore each island.

3. For DFS approach:
   - Use a recursive function to explore all connected land cells.
   - Mark visited cells to avoid counting them again.
   - The four directions to explore are up, down, left, and right.

4. For BFS approach:
   - Use a queue to keep track of cells to visit.
   - Start BFS from each unvisited land cell and mark all connected land cells as visited.

5. To mark cells as visited, you can either:
   - Modify the original grid by changing '1' to '0' or another value.
   - Use a separate visited matrix.

6. Be careful with the grid boundaries and only explore valid cells.

7. The number of times you start a new DFS or BFS (from an unvisited land cell) is the number of islands.
