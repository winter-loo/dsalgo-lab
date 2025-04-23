# Approach

This problem asks us to fill each empty room with the distance to its nearest gate. If it is impossible to reach a gate, it should remain filled with INF.

## Strategy
We can use Breadth-First Search (BFS) starting from all gates simultaneously. This allows us to find the shortest distance from each empty room to its nearest gate.

## Steps for BFS Approach
1. Initialize a queue to store the coordinates of cells to visit.
2. Add all gates (cells with value 0) to the queue as starting points.
3. Perform BFS:
   - For each cell in the queue, explore its four adjacent cells (up, down, left, right).
   - If an adjacent cell is an empty room (INF), update its value to the distance from the current cell plus 1, and add it to the queue.
   - Continue until the queue is empty.

## Implementation Details
- We need to be careful with the grid boundaries and only explore valid cells.
- We should not revisit cells that have already been visited (cells that are no longer INF).
- We can use a queue to perform BFS, where each element in the queue is a tuple (row, col, distance).
- Alternatively, we can use a queue of (row, col) and update the distance directly in the grid.

## Time and Space Complexity
- **Time Complexity**: O(m * n) where m is the number of rows and n is the number of columns in the grid. We visit each cell at most once.
- **Space Complexity**: O(m * n) for the queue in the worst case if all cells are gates or empty rooms.

## Alternative Approach: DFS
We could also use Depth-First Search (DFS) starting from each gate, but BFS is more efficient for finding the shortest distance.

## Pseudocode for BFS Approach
```
function wallsAndGates(rooms):
    if rooms is empty:
        return
    
    m = rooms.length
    n = rooms[0].length
    queue = new Queue()
    
    // Add all gates to the queue
    for i from 0 to m-1:
        for j from 0 to n-1:
            if rooms[i][j] == 0:
                queue.add((i, j))
    
    // Directions: up, right, down, left
    directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]
    
    // BFS
    while queue is not empty:
        row, col = queue.poll()
        
        for dir in directions:
            new_row = row + dir[0]
            new_col = col + dir[1]
            
            // Check if the new cell is valid and is an empty room
            if new_row < 0 or new_row >= m or new_col < 0 or new_col >= n or rooms[new_row][new_col] != INF:
                continue
            
            // Update the distance
            rooms[new_row][new_col] = rooms[row][col] + 1
            
            // Add the new cell to the queue
            queue.add((new_row, new_col))
```
