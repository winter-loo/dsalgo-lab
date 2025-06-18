# Approach

The "Meeting Rooms" problem asks us to determine if a person can attend all meetings, which means no two meetings can overlap.

## Understanding the Problem

Two meetings overlap if one meeting starts before another meeting ends. We need to check if any two meetings in the given list overlap.

## Sorting-Based Approach

1. Sort the intervals based on their start times.
2. Iterate through the sorted intervals and check if any adjacent intervals overlap.
3. If any overlap is found, return false; otherwise, return true.

## Time and Space Complexity

- **Time Complexity**: O(n log n), where n is the number of intervals. Sorting takes O(n log n) time, and then we do a linear scan through the intervals.
- **Space Complexity**: O(1) if we don't count the space for sorting, as we only need a few variables to keep track of the previous interval.

## Pseudocode

```
function canAttendMeetings(intervals):
    if intervals is empty:
        return true
    
    // Sort intervals by start time
    sort intervals by intervals[i][0]
    
    for i from 1 to length(intervals) - 1:
        if intervals[i][0] < intervals[i-1][1]:
            // Current meeting starts before the previous meeting ends
            return false
    
    return true
```

## Step-by-Step Example

Let's trace through the algorithm with the example: intervals = [[0,30],[5,10],[15,20]]

1. Sort the intervals by start time: [[0,30],[5,10],[15,20]] (already sorted).
2. Check if intervals[1] overlaps with intervals[0]:
   - intervals[1][0] = 5 < intervals[0][1] = 30, so they overlap.
   - Return false.

Let's trace through another example: intervals = [[7,10],[2,4]]

1. Sort the intervals by start time: [[2,4],[7,10]].
2. Check if intervals[1] overlaps with intervals[0]:
   - intervals[1][0] = 7 >= intervals[0][1] = 4, so they don't overlap.
3. No more intervals to check, return true.

## Alternative Approach: Checking All Pairs

Another approach is to check all pairs of intervals for overlap:
1. For each pair of intervals, check if they overlap.
2. If any pair overlaps, return false; otherwise, return true.

However, this approach has a time complexity of O(n²), which is less efficient than the sorting-based approach.
