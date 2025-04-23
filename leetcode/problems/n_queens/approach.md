# Approach

The N-Queens problem is a classic backtracking problem where we need to place N queens on an N×N chessboard such that no two queens attack each other.

## Strategy
This problem can be solved using backtracking, where we try to place queens row by row, ensuring that each placement is valid.

## Steps
1. Start with an empty board.
2. Try to place a queen in the first row.
3. After placing a queen, move to the next row and try to place another queen.
4. If at any point we can't place a queen in a row without it being attacked, backtrack and try a different position for the previous queen.
5. If we successfully place all N queens, add the current board configuration to the result.

## Implementation Details
- We need to check if a position is valid for placing a queen:
  - No queen in the same column
  - No queen in the same diagonal
  - No queen in the same anti-diagonal
- We can use arrays or sets to keep track of occupied columns and diagonals.
- For diagonals, we can use the formula `row - col` to identify a diagonal.
- For anti-diagonals, we can use the formula `row + col` to identify an anti-diagonal.

## Pseudocode
```
function solveNQueens(n):
    result = []
    board = create an n×n board filled with '.'
    backtrack(0, board, result, n)
    return result

function backtrack(row, board, result, n):
    if row == n:
        add the current board configuration to result
        return
    
    for col from 0 to n-1:
        if isValid(board, row, col, n):
            place a queen at board[row][col]
            backtrack(row + 1, board, result, n)
            remove the queen from board[row][col] (backtrack)

function isValid(board, row, col, n):
    // Check if there's a queen in the same column
    for i from 0 to row-1:
        if board[i][col] == 'Q':
            return false
    
    // Check if there's a queen in the upper-left diagonal
    for i, j = row-1, col-1; i >= 0 and j >= 0; i--, j--:
        if board[i][j] == 'Q':
            return false
    
    // Check if there's a queen in the upper-right diagonal
    for i, j = row-1, col+1; i >= 0 and j < n; i--, j++:
        if board[i][j] == 'Q':
            return false
    
    return true
```

## Optimization
- Instead of checking for validity each time, we can use three sets to keep track of occupied columns, diagonals, and anti-diagonals.
- This reduces the time complexity of checking validity from O(n) to O(1).

## Time and Space Complexity
- **Time Complexity**: O(N!) where N is the input size. This is because in the worst case, we explore all possible placements of queens.
- **Space Complexity**: O(N) for the recursion stack and the sets used to track occupied columns and diagonals.
