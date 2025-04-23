# Approach

This problem asks us to find the minimum number of minutes until all fresh oranges become rotten, or determine if it's impossible.

## Strategy
We can use Breadth-First Search (BFS) to simulate the rotting process layer by layer, where each layer represents one minute of time.

## Steps for BFS Approach
1. Initialize a queue to store the coordinates of rotten oranges.
2. Count the total number of fresh oranges initially.
3. Add all initially rotten oranges to the queue.
4. Perform BFS:
   - For each minute, process all rotten oranges in the current queue.
   - For each rotten orange, check its four adjacent cells (up, down, left, right).
   - If an adjacent cell contains a fresh orange, make it rotten, decrease the fresh orange count, and add it to the queue.
   - After processing all oranges at the current minute, increment the minute count.
5. Continue until the queue is empty (no more oranges can rot).
6. If there are still fresh oranges left, return -1 (impossible to rot all oranges).
7. Otherwise, return the total minutes elapsed.

## Implementation Details
- We need to be careful with the grid boundaries and only explore valid cells.
- We should keep track of the number of fresh oranges to quickly determine if all oranges can be rotten.
- We can use a queue to perform BFS, where each element in the queue is a tuple (row, col).
- To track the minutes, we can either:
  - Use a separate variable and process all oranges at the current minute together.
  - Include the minute in the queue elements as (row, col, minute).

## Time and Space Complexity
- **Time Complexity**: O(m * n) where m is the number of rows and n is the number of columns in the grid. We visit each cell at most once.
- **Space Complexity**: O(m * n) for the queue in the worst case if all cells contain rotten oranges.

## Pseudocode for BFS Approach
```
function orangesRotting(grid):
    if grid is empty:
        return 0
    
    m = grid.length
    n = grid[0].length
    queue = new Queue()
    fresh_count = 0
    
    // Count fresh oranges and add rotten oranges to the queue
    for i from 0 to m-1:
        for j from 0 to n-1:
            if grid[i][j] == 1:
                fresh_count += 1
            else if grid[i][j] == 2:
                queue.add((i, j))
    
    // If there are no fresh oranges, return 0
    if fresh_count == 0:
        return 0
    
    // Directions: up, right, down, left
    directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]
    
    // BFS
    minutes = 0
    while queue is not empty and fresh_count > 0:
        size = queue.size()
        
        // Process all oranges at the current minute
        for i from 0 to size-1:
            row, col = queue.poll()
            
            for dir in directions:
                new_row = row + dir[0]
                new_col = col + dir[1]
                
                // Check if the new cell is valid and contains a fresh orange
                if new_row >= 0 and new_row < m and new_col >= 0 and new_col < n and grid[new_row][new_col] == 1:
                    // Make the orange rotten
                    grid[new_row][new_col] = 2
                    fresh_count -= 1
                    
                    // Add the new rotten orange to the queue
                    queue.add((new_row, new_col))
        
        // Increment the minute count
        minutes += 1
    
    // If there are still fresh oranges left, return -1
    if fresh_count > 0:
        return -1
    
    return minutes
```
