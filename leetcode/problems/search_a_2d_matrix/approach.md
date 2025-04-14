# Approach: Search a 2D Matrix

## Intuition
The key insight for this problem is to leverage the sorted nature of the matrix. Since each row is sorted and the first element of each row is greater than the last element of the previous row, we can treat the entire 2D matrix as a sorted 1D array and apply binary search.

## Multiple Approaches

### Approach 1: Treat as 1D Array
1. Consider the 2D matrix as a flattened 1D array of size m*n.
2. Perform binary search on this conceptual 1D array.
3. Map the middle index of the 1D array back to the corresponding row and column in the 2D matrix.
4. Compare the element at this position with the target and adjust the search space accordingly.

**Time Complexity**: O(log(m*n)) - Binary search on a conceptual array of size m*n.
**Space Complexity**: O(1) - Constant extra space.

### Approach 2: Row Binary Search + Column Binary Search
1. First, perform binary search on the first column to find the row where the target might be located.
2. Then, perform binary search on that specific row to find the target.

**Time Complexity**: O(log m + log n) - Binary search on rows followed by binary search on columns.
**Space Complexity**: O(1) - Constant extra space.

### Approach 3: Linear Search (Suboptimal)
1. Iterate through each row and check if the target is within the range of that row.
2. If it is, perform binary search on that row.

**Time Complexity**: O(m + log n) - Linear search on rows followed by binary search on a specific row.
**Space Complexity**: O(1) - Constant extra space.

## Implementation Notes
- For Approach 1, to map a 1D index `idx` to 2D coordinates in an m×n matrix:
  - Row = idx / n
  - Column = idx % n
- Be careful with edge cases:
  - Empty matrix
  - Matrix with only one row or column
  - Target smaller than the smallest element or larger than the largest element
- The problem guarantees that the matrix has at least one row and one column, so we don't need to check for an empty matrix.
- Since the matrix has sorted rows and the first element of each row is greater than the last element of the previous row, we can also use a more efficient approach by first binary searching for the row and then for the column.

## Topics
- Array
- Binary Search
- Matrix
