# Approach

To find the kth largest element in an array, we need to efficiently identify the element that would be at the kth position if the array were sorted in descending order.

## Strategy
There are several approaches to solve this problem:
- Sort the array and return the kth element (simple but not optimal).
- Use a min-heap of size k to keep track of the k largest elements.
- Use the QuickSelect algorithm, which is an optimized approach with average O(n) time complexity.

## Steps
### Sorting Approach
1. Sort the array in descending order.
2. Return the element at index k-1.

### Min-Heap Approach
1. Create a min-heap of size k.
2. Iterate through the array:
   - If the heap size is less than k, add the current element.
   - If the current element is larger than the smallest element in the heap, remove the smallest element and add the current element.
3. The root of the heap will be the kth largest element.

### QuickSelect Approach
1. Choose a pivot element from the array.
2. Partition the array around the pivot (elements greater than pivot on one side, smaller on the other).
3. If the pivot is at the kth position, return it.
4. If the pivot is at a position greater than k, recursively search in the left subarray.
5. If the pivot is at a position less than k, recursively search in the right subarray.

## Implementation Notes
- The QuickSelect algorithm is similar to QuickSort but only needs to recurse into one side of the partition.
- The average time complexity of QuickSelect is O(n), making it more efficient than sorting (O(n log n)) for this specific problem.

// TODO: Add time and space complexity analysis.
