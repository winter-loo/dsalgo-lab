# Approach

This problem asks us to capture all regions of 'O's that are surrounded by 'X's, by flipping them to 'X's. A region is surrounded if it is not connected to any 'O' on the border of the board.

## Strategy
Instead of trying to identify surrounded regions directly, we can use a reverse approach: identify regions that are not surrounded (i.e., connected to the border) and then flip all other 'O's to 'X's.

## Steps
1. Identify all 'O's on the border of the board.
2. Use Depth-First Search (DFS) or Breadth-First Search (BFS) to mark all 'O's that are connected to the border 'O's.
3. Flip all unmarked 'O's to 'X's.

## Implementation Details
- We can use a boolean matrix or modify the board directly to mark 'O's that are connected to the border.
- For DFS, we recursively explore neighboring cells that are 'O's.
- For BFS, we use a queue to keep track of cells to visit.
- We need to be careful with the grid boundaries and only explore valid cells.

## Time and Space Complexity
- **Time Complexity**: O(m * n) where m is the number of rows and n is the number of columns in the board. We visit each cell at most once.
- **Space Complexity**: O(m * n) for the recursion stack (DFS) or queue (BFS) in the worst case.

## Pseudocode for DFS Approach
```
function solve(board):
    if board is empty:
        return
    
    m = board.length
    n = board[0].length
    
    // Mark 'O's on the border and their connected 'O's
    // Top and bottom rows
    for j from 0 to n-1:
        dfs(board, 0, j)       // Top row
        dfs(board, m-1, j)     // Bottom row
    
    // Left and right columns
    for i from 0 to m-1:
        dfs(board, i, 0)       // Left column
        dfs(board, i, n-1)     // Right column
    
    // Flip unmarked 'O's to 'X's and restore marked 'O's
    for i from 0 to m-1:
        for j from 0 to n-1:
            if board[i][j] == 'O':
                board[i][j] = 'X'
            else if board[i][j] == 'M':  // 'M' is a temporary mark
                board[i][j] = 'O'

function dfs(board, i, j):
    // Check if the cell is valid and is an 'O'
    if i < 0 or i >= board.length or j < 0 or j >= board[0].length or board[i][j] != 'O':
        return
    
    // Mark the cell as visited
    board[i][j] = 'M'  // 'M' is a temporary mark
    
    // Explore the four adjacent cells
    dfs(board, i-1, j)  // Up
    dfs(board, i+1, j)  // Down
    dfs(board, i, j-1)  // Left
    dfs(board, i, j+1)  // Right
```

## Alternative Approach: Union-Find
We can also use the Union-Find data structure to solve this problem:
1. Create a virtual node that represents the border.
2. Union all 'O's on the border with this virtual node.
3. For each 'O' in the board, union it with its adjacent 'O's.
4. After building the Union-Find structure, flip all 'O's that are not connected to the virtual border node.
