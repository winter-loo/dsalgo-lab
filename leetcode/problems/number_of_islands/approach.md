# Approach

To count the number of islands in a grid, we need to identify connected components of land cells (cells with value '1').

## Strategy
We can use either Depth-First Search (DFS) or Breadth-First Search (BFS) to explore each island. The key idea is to:
1. Iterate through each cell in the grid.
2. When we find a land cell ('1'), we explore the entire island connected to it and mark all cells of that island as visited.
3. Increment the island count for each new island we discover.

## Steps for DFS Approach
1. Initialize an island counter to 0.
2. Iterate through each cell in the grid:
   - If the cell is water ('0') or has been visited, skip it.
   - If the cell is land ('1') and hasn't been visited:
     - Increment the island counter.
     - Use DFS to explore the entire island connected to this cell.
     - Mark all cells of the island as visited (by changing '1' to '0' or using a separate visited matrix).
3. Return the island counter.

## Steps for BFS Approach
1. Initialize an island counter to 0.
2. Iterate through each cell in the grid:
   - If the cell is water ('0') or has been visited, skip it.
   - If the cell is land ('1') and hasn't been visited:
     - Increment the island counter.
     - Use BFS to explore the entire island connected to this cell.
     - Mark all cells of the island as visited.
3. Return the island counter.

## Implementation Details
- For DFS, we can use a recursive function that explores all four adjacent cells (up, down, left, right).
- For BFS, we can use a queue to keep track of cells to visit.
- To mark cells as visited, we can either:
  - Modify the original grid by changing '1' to '0' (if allowed).
  - Use a separate visited matrix.
- We need to be careful with the grid boundaries and only explore valid cells.

## Time and Space Complexity
- **Time Complexity**: O(M * N) where M is the number of rows and N is the number of columns in the grid. We visit each cell at most once.
- **Space Complexity**: 
  - O(M * N) in the worst case for the recursion stack (DFS) or queue (BFS) if the entire grid is filled with land.
  - O(1) if we modify the original grid to mark visited cells.
  - O(M * N) if we use a separate visited matrix.
