# Hints for Surrounded Regions

1. This problem asks you to capture all regions of 'O's that are surrounded by 'X's, by flipping them to 'X's. A region is surrounded if it is not connected to any 'O' on the border of the board.

2. Instead of trying to identify surrounded regions directly, use a reverse approach: identify regions that are not surrounded (i.e., connected to the border) and then flip all other 'O's to 'X's.

3. Start by identifying all 'O's on the border of the board. These 'O's cannot be captured.

4. Use Depth-First Search (DFS) or Breadth-First Search (BFS) to mark all 'O's that are connected to the border 'O's. These 'O's also cannot be captured.

5. After marking all 'O's that cannot be captured, flip all unmarked 'O's to 'X's.

6. Be careful with the implementation details:
   - You can use a temporary mark (e.g., 'M') to distinguish between 'O's that are connected to the border and those that are not.
   - Remember to restore the temporary marks back to 'O's after flipping the surrounded 'O's to 'X's.

7. The four directions to explore are up, down, left, and right (not diagonally).

8. Be careful with the grid boundaries and only explore valid cells.
