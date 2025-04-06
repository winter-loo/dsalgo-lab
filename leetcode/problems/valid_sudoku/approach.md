# Approach: Valid Sudoku

## Intuition
To determine if a Sudoku board is valid, we need to check three conditions: each row, each column, and each 3x3 sub-box contains no duplicate digits from 1-9. We can use hash sets to track seen digits in each of these contexts.

## Approach
1. Iterate through each cell in the 9x9 board.
2. For each cell that contains a digit (not '.'):
   - Check if the digit has already appeared in the current row.
   - Check if the digit has already appeared in the current column.
   - Check if the digit has already appeared in the current 3x3 sub-box.
3. If any duplicate is found, return false; otherwise, return true after processing all cells.

There are several ways to implement this validation:

### Hash Sets Approach
1. Create three sets of hash sets: one for rows, one for columns, and one for sub-boxes.
2. For each digit in the board, add it to the corresponding row, column, and sub-box sets.
3. If adding a digit to any set fails (because it's already present), return false.

### Bit Manipulation Approach (Optimized)
1. Use bit manipulation to track seen digits in each row, column, and sub-box.
2. Each digit (1-9) can be represented by a bit position in an integer.
3. When a digit is encountered, check if its bit is already set in the corresponding row, column, or sub-box integers.
4. If the bit is already set, return false; otherwise, set the bit and continue.

### Array-based Approach
1. Use boolean arrays to track seen digits in each row, column, and sub-box.
2. For each digit, mark its presence in the corresponding arrays.
3. If a digit is already marked as present, return false.

## Complexity Analysis
- **Time Complexity**: O(1) since the board size is fixed at 9x9. We're essentially doing a constant number of operations.
- **Space Complexity**: O(1) for the same reason. We use a fixed amount of extra space regardless of input size.

## Edge Cases
- Empty cells (marked with '.') should be ignored.
- A valid Sudoku may not be solvable, but we only need to check if it follows the rules so far.
- The board is always 9x9 according to the constraints. 