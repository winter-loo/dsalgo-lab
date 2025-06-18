# Approach

The "Non-overlapping Intervals" problem asks us to find the minimum number of intervals to remove to make the rest non-overlapping.

## Understanding the Problem

We need to remove the minimum number of intervals such that no two intervals in the remaining set overlap. This is equivalent to finding the maximum number of non-overlapping intervals we can keep.

## Greedy Approach

1. Sort the intervals based on their end times.
2. Greedily select intervals that don't overlap with the previously selected interval.
3. The number of intervals to remove is the total number of intervals minus the number of intervals we can keep.

## Why Sort by End Time?

Sorting by end time is crucial for the greedy approach. By selecting intervals with earlier end times, we leave more room for subsequent intervals, maximizing the number of intervals we can keep.

## Time and Space Complexity

- **Time Complexity**: O(n log n), where n is the number of intervals. Sorting takes O(n log n) time, and then we do a linear scan through the intervals.
- **Space Complexity**: O(1) if we don't count the space for the output, as we only need a few variables to keep track of the count and the end time of the last selected interval.

## Pseudocode

```
function eraseOverlapIntervals(intervals):
    if intervals is empty:
        return 0
    
    // Sort intervals by end time
    sort intervals by intervals[i][1]
    
    count = 1  // Count of intervals we can keep
    end = intervals[0][1]  // End time of the last selected interval
    
    for i from 1 to length(intervals) - 1:
        if intervals[i][0] >= end:
            // Current interval doesn't overlap with the last selected interval
            count++
            end = intervals[i][1]
    
    return length(intervals) - count
```

## Step-by-Step Example

Let's trace through the algorithm with the example: intervals = [[1,2],[2,3],[3,4],[1,3]]

1. Sort the intervals by end time: [[1,2],[2,3],[1,3],[3,4]]
2. Initialize count = 1, end = 2 (from the first interval [1,2]).
3. Process [2,3]:
   - 2 >= 2, so it doesn't overlap with the last selected interval.
   - Update count = 2, end = 3.
4. Process [1,3]:
   - 1 < 3, so it overlaps with the last selected interval.
   - Skip this interval.
5. Process [3,4]:
   - 3 >= 3, so it doesn't overlap with the last selected interval.
   - Update count = 3, end = 4.
6. Return length(intervals) - count = 4 - 3 = 1.

## Alternative Approach: Dynamic Programming

We can also solve this problem using dynamic programming:
1. Sort the intervals by start time.
2. For each interval, consider two options: include it or exclude it.
3. If we include it, we can only include intervals that don't overlap with it.
4. If we exclude it, we can consider the next interval.
5. Choose the option that maximizes the number of intervals we can keep.

However, the greedy approach is more efficient and simpler for this problem.
