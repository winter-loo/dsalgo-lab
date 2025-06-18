# Hints for Meeting Rooms II

1. Think about what determines the number of conference rooms needed - it's the maximum number of meetings that overlap at any point in time.

2. Consider separating the start times and end times of all meetings and sorting them separately.

3. As you iterate through the sorted times, a start time means you need a new room, and an end time means you can free up a room.

4. Keep track of the number of rooms in use at each point in time. The maximum value is your answer.

5. Alternatively, you can use a min-heap (priority queue) to keep track of the end times of meetings.

6. Sort the intervals by start time, and for each interval, check if any previous meeting has ended.

7. If a previous meeting has ended (its end time is less than or equal to the current start time), you can reuse that room.

8. Otherwise, you need a new room for the current meeting.

9. Be careful with edge cases, such as when all meetings overlap or when there are no meetings.

10. The time complexity of both approaches (chronological ordering and priority queue) is O(n log n), where n is the number of intervals.
