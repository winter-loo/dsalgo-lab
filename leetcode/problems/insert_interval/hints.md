# Hints for Insert Interval

1. Remember that the input intervals are already sorted by start time and are non-overlapping.

2. Consider breaking down the problem into three parts:
   - Intervals that come before the new interval (no overlap)
   - Intervals that overlap with the new interval
   - Intervals that come after the new interval (no overlap)

3. For intervals that come before the new interval, their end time will be less than the start time of the new interval.

4. For intervals that come after the new interval, their start time will be greater than the end time of the new interval.

5. For overlapping intervals, you'll need to merge them with the new interval.

6. Two intervals overlap if one's start time is less than or equal to the other's end time, and one's end time is greater than or equal to the other's start time.

7. When merging overlapping intervals, take the minimum of the start times and the maximum of the end times.

8. You can solve this in a single pass through the intervals array.

9. Be careful with edge cases, such as when the new interval should be inserted at the beginning or end of the array.

10. Consider what happens when the intervals array is empty.
