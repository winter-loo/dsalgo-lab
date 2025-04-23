# Approach

This problem asks us to find all cells from which water can flow to both the Pacific and Atlantic oceans.

## Strategy
Instead of trying to determine if water can flow from each cell to both oceans (which would be inefficient), we can reverse the problem: start from the ocean and determine which cells can reach each ocean.

We can use Depth-First Search (DFS) or Breadth-First Search (BFS) to explore from the ocean boundaries inward. A cell can be reached from the ocean if water can flow from that cell to the ocean, which happens when the neighboring cell's height is less than or equal to the current cell's height.

## Steps
1. Create two boolean matrices to mark cells that can reach the Pacific and Atlantic oceans.
2. Perform DFS/BFS starting from the cells adjacent to each ocean:
   - For the Pacific Ocean, start from the top row and left column.
   - For the Atlantic Ocean, start from the bottom row and right column.
3. During the DFS/BFS, mark cells that can be reached from each ocean.
4. After both traversals, find cells that are marked as reachable from both oceans.

## Implementation Details
- We can use a boolean matrix or a set to keep track of cells that can reach each ocean.
- For DFS, we recursively explore neighboring cells with heights greater than or equal to the current cell.
- For BFS, we use a queue to keep track of cells to visit.
- We need to be careful with the grid boundaries and only explore valid cells.

## Time and Space Complexity
- **Time Complexity**: O(m * n) where m is the number of rows and n is the number of columns in the grid. We visit each cell at most twice (once for each ocean).
- **Space Complexity**: O(m * n) for the boolean matrices and the recursion stack (DFS) or queue (BFS).

## Pseudocode for DFS Approach
```
function pacificAtlantic(heights):
    if heights is empty:
        return []
    
    m = heights.length
    n = heights[0].length
    
    // Create boolean matrices to mark cells that can reach each ocean
    pacific = new boolean[m][n]
    atlantic = new boolean[m][n]
    
    // DFS from the Pacific Ocean boundaries
    for i from 0 to m-1:
        dfs(heights, pacific, i, 0, heights[i][0])  // Left column
    for j from 0 to n-1:
        dfs(heights, pacific, 0, j, heights[0][j])  // Top row
    
    // DFS from the Atlantic Ocean boundaries
    for i from 0 to m-1:
        dfs(heights, atlantic, i, n-1, heights[i][n-1])  // Right column
    for j from 0 to n-1:
        dfs(heights, atlantic, m-1, j, heights[m-1][j])  // Bottom row
    
    // Find cells that can reach both oceans
    result = []
    for i from 0 to m-1:
        for j from 0 to n-1:
            if pacific[i][j] and atlantic[i][j]:
                result.add([i, j])
    
    return result

function dfs(heights, visited, i, j, prevHeight):
    // Check if the cell is valid and not visited
    if i < 0 or i >= heights.length or j < 0 or j >= heights[0].length or visited[i][j] or heights[i][j] < prevHeight:
        return
    
    // Mark the cell as visited
    visited[i][j] = true
    
    // Explore the four adjacent cells
    dfs(heights, visited, i-1, j, heights[i][j])  // Up
    dfs(heights, visited, i+1, j, heights[i][j])  // Down
    dfs(heights, visited, i, j-1, heights[i][j])  // Left
    dfs(heights, visited, i, j+1, heights[i][j])  // Right
```
