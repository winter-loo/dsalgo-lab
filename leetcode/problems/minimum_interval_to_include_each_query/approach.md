# Approach

The "Minimum Interval to Include Each Query" problem asks us to find the size
of the smallest interval that contains each query point.

## Understanding the Problem

For each query point, we need to find the smallest interval (in terms of size)
that contains the query point. If no interval contains the query point, the
answer is -1.

## Sorting and Priority Queue Approach

1. Sort the queries while keeping track of their original indices (since we
   need to return the answers in the original order).
2. Sort the intervals by their start times.
3. Use a min-heap (priority queue) to keep track of intervals that contain the
   current query point, ordered by their size.
4. For each query point:
   - Add all intervals that start at or before the query point to the heap.
   - Remove all intervals that end before the query point from the heap.
   - If the heap is not empty, the top of the heap is the smallest interval containing the query point.
   - If the heap is empty, no interval contains the query point, so the answer is -1.

## Time and Space Complexity

- **Time Complexity**: O((n + q) log (n + q)), where n is the number of intervals and q is the number of queries. Sorting takes O(n log n) and O(q log q) time, and each heap operation takes O(log n) time.
- **Space Complexity**: O(n + q) for storing the sorted queries and the heap.

## Pseudocode

```
function minInterval(intervals, queries):
    // Sort intervals by start time
    sort intervals by intervals[i][0]
    
    // Sort queries and keep track of original indices
    sortedQueries = []
    for i from 0 to length(queries) - 1:
        sortedQueries.append((queries[i], i))
    sort sortedQueries by sortedQueries[i][0]
    
    // Initialize result array
    result = [-1] * length(queries)
    
    // Initialize min-heap for intervals
    heap = empty min-heap  // (size, end)
    
    i = 0  // pointer for intervals
    
    // Process each query
    for (query, originalIndex) in sortedQueries:
        // Add all intervals that start at or before the query point
        while i < length(intervals) and intervals[i][0] <= query:
            start = intervals[i][0]
            end = intervals[i][1]
            size = end - start + 1
            if end >= query:  // Only add intervals that might contain the query
                heap.push((size, end))
            i++
        
        // Remove all intervals that end before the query point
        while heap is not empty and heap.top()[1] < query:
            heap.pop()
        
        // If the heap is not empty, the top of the heap is the smallest interval containing the query
        if heap is not empty:
            result[originalIndex] = heap.top()[0]
    
    return result
```

## Step-by-Step Example

Let's trace through the algorithm with the example:
`intervals = [[1,4],[2,4],[3,6],[4,4]]`, `queries = [2,3,4,5]`

1. Sort intervals by start time: `[[1,4],[2,4],[3,6],[4,4]]` (already sorted).
2. Sort queries and keep track of original indices: `[(2,0), (3,1), (4,2), (5,3)]`.
3. Initialize result = `[-1, -1, -1, -1]`.
4. Process query = 2, originalIndex = 0:
   - Add intervals that start at or before 2: `[1,4]` (size 4), `[2,4]` (size 3).
   - Heap = `[(3, 4), (4, 4)]`.
   - `result[0] = 3`.
5. Process query = 3, originalIndex = 1:
   - Add intervals that start at or before 3: `[3,6]` (size 4).
   - Heap = `[(3, 4), (4, 4), (4, 6)]`.
   - `result[1] = 3`.
6. Process query = 4, originalIndex = 2:
   - Add intervals that start at or before 4: `[4,4]` (size 1).
   - Heap = `[(1, 4), (3, 4), (4, 4), (4, 6)]`.
   - `result[2] = 1`.
7. Process query = 5, originalIndex = 3:
   - No more intervals to add.
   - Remove intervals that end before 5: `[1,4], [2,4], [4,4]`.
   - Heap = `[(4, 6)]`.
   - `result[3] = 4`.
8. Return `result = [3, 3, 1, 4]`.

## Alternative Approach: Binary Search

Another approach is to use binary search:
1. For each query, binary search for all intervals that contain the query point.
2. Find the smallest such interval.

However, this approach would have a time complexity of O(q * log n * n) in the
worst case, which is less efficient than the priority queue approach.
