# Hints for Pacific Atlantic Water Flow

1. This problem asks you to find all cells from which water can flow to both the Pacific and Atlantic oceans.

2. Instead of trying to determine if water can flow from each cell to both oceans (which would be inefficient), try reversing the problem: start from the ocean and determine which cells can reach each ocean.

3. Water can flow from a cell to a neighboring cell if the neighboring cell's height is less than or equal to the current cell's height. When we reverse the problem, we're looking for cells with heights greater than or equal to the current cell.

4. Use Depth-First Search (DFS) or Breadth-First Search (BFS) to explore from the ocean boundaries inward:
   - For the Pacific Ocean, start from the top row and left column.
   - For the Atlantic Ocean, start from the bottom row and right column.

5. Keep track of cells that can be reached from each ocean using boolean matrices or sets.

6. After both traversals, find cells that are marked as reachable from both oceans.

7. Be careful with the grid boundaries and only explore valid cells.

8. Remember that water can flow in four directions: up, down, left, and right.
