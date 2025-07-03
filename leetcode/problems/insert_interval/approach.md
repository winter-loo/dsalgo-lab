# Approach

The "Insert Interval" problem asks us to insert a new interval into a sorted
list of non-overlapping intervals, merging overlapping intervals if necessary.

## Understanding the Problem

We have a sorted list of non-overlapping intervals, and we need to insert a new
interval. The key challenge is handling the potential overlaps between the new
interval and existing intervals.

## One-Pass Approach

We can solve this problem with a single pass through the intervals:

1. Add all intervals that come before the new interval (end < newStart).
2. Merge the new interval with any overlapping intervals (start <= newEnd && end >= newStart).
3. Add all intervals that come after the merged interval (start > newEnd).

## Time and Space Complexity

- **Time Complexity**: O(n), where n is the number of intervals. We process each interval once.
- **Space Complexity**: O(n) for the result array. The actual space used depends on how many intervals are in the final result.

## Pseudocode

```
function insert(intervals, newInterval):
    result = empty list
    i = 0
    n = length of intervals
    
    // Add all intervals that come before newInterval
    while i < n and intervals[i][1] < newInterval[0]:
        add intervals[i] to result
        i++
    
    // Merge overlapping intervals
    while i < n and intervals[i][0] <= newInterval[1]:
        newInterval[0] = min(newInterval[0], intervals[i][0])
        newInterval[1] = max(newInterval[1], intervals[i][1])
        i++
    
    // Add the merged interval
    add newInterval to result
    
    // Add all intervals that come after newInterval
    while i < n:
        add intervals[i] to result
        i++
    
    return result
```

## Step-by-Step Example

Let's trace through the algorithm with the example: intervals = [[1,3],[6,9]], newInterval = [2,5]

1. Initialize result = []
2. Process intervals that come before newInterval:
   - [1,3]: 3 > 2, so it overlaps with newInterval. We don't add it yet.
3. Merge overlapping intervals:
   - [1,3]: 1 <= 5 && 3 >= 2, so it overlaps.
   - Update newInterval to [min(2,1), max(5,3)] = [1,5]
4. Add the merged interval:
   - result = [[1,5]]
5. Process intervals that come after newInterval:
   - [6,9]: 6 > 5, so add it to result.
   - result = [[1,5], [6,9]]
6. Return result = [[1,5], [6,9]]

## Alternative Approach: Binary Search

Since the intervals are sorted, we could use binary search to find the position
where the new interval should be inserted. However, we would still need to
handle merging, so the one-pass approach is generally simpler and equally
efficient.
