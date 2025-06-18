# Approach

The "Meeting Rooms II" problem asks us to find the minimum number of conference rooms required to host all meetings.

## Understanding the Problem

We need to determine how many conference rooms are needed such that no two meetings overlap in the same room. This is equivalent to finding the maximum number of meetings that overlap at any point in time.

## Chronological Ordering Approach

1. Separate the start times and end times of all meetings.
2. Sort both arrays.
3. Use two pointers to iterate through the sorted arrays.
4. When we encounter a start time, we need a new room.
5. When we encounter an end time, we can free up a room.
6. The maximum number of rooms needed at any point is our answer.

## Time and Space Complexity

- **Time Complexity**: O(n log n), where n is the number of intervals. Sorting takes O(n log n) time.
- **Space Complexity**: O(n) for storing the separate start and end time arrays.

## Pseudocode

```
function minMeetingRooms(intervals):
    if intervals is empty:
        return 0
    
    // Extract start and end times
    start_times = []
    end_times = []
    for interval in intervals:
        start_times.append(interval[0])
        end_times.append(interval[1])
    
    // Sort start and end times
    sort(start_times)
    sort(end_times)
    
    // Count rooms
    rooms = 0
    end_ptr = 0
    
    for start_ptr from 0 to length(start_times) - 1:
        // If the earliest meeting has ended before the current meeting starts
        if start_times[start_ptr] >= end_times[end_ptr]:
            // Free up a room
            rooms--
            end_ptr++
        
        // We need a new room for the current meeting
        rooms++
    
    return rooms
```

## Alternative Approach: Priority Queue

Another approach is to use a min-heap (priority queue) to keep track of the end times of meetings:

1. Sort the intervals by start time.
2. Iterate through the sorted intervals.
3. For each interval, check if any previous meeting has ended (by comparing the current start time with the earliest end time in the heap).
4. If a previous meeting has ended, remove it from the heap (free up a room).
5. Add the current meeting's end time to the heap (allocate a room).
6. The size of the heap at the end is the minimum number of rooms required.

This approach also has a time complexity of O(n log n) but may be more intuitive for some.

## Step-by-Step Example

Let's trace through the chronological ordering approach with the example: intervals = [[0,30],[5,10],[15,20]]

1. Extract start and end times:
   - start_times = [0, 5, 15]
   - end_times = [30, 10, 20]
2. Sort start and end times:
   - start_times = [0, 5, 15]
   - end_times = [10, 20, 30]
3. Count rooms:
   - Initialize rooms = 0, end_ptr = 0
   - Process start_time = 0:
     - 0 < 10, so we need a new room.
     - rooms = 1, end_ptr = 0
   - Process start_time = 5:
     - 5 < 10, so we need a new room.
     - rooms = 2, end_ptr = 0
   - Process start_time = 15:
     - 15 >= 10, so we can free up a room.
     - rooms = 1, end_ptr = 1
     - 15 < 20, so we need a new room.
     - rooms = 2, end_ptr = 1
4. Return rooms = 2.
