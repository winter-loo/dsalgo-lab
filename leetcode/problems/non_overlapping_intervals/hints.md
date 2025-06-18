# Hints for Non-overlapping Intervals

1. Think about this problem in terms of finding the maximum number of non-overlapping intervals that you can keep.

2. Consider sorting the intervals to make it easier to identify overlaps.

3. There are multiple ways to sort the intervals: by start time, by end time, or by interval length. Which one would be most helpful?

4. Sorting by end time is particularly useful for this problem. Why? Because intervals with earlier end times leave more room for subsequent intervals.

5. After sorting by end time, you can use a greedy approach to select intervals.

6. Keep track of the end time of the last selected interval. If the start time of the current interval is greater than or equal to this end time, you can select the current interval.

7. The minimum number of intervals to remove is the total number of intervals minus the maximum number of non-overlapping intervals you can keep.

8. Be careful with edge cases, such as when all intervals overlap or when there are duplicate intervals.

9. Consider what happens when intervals are exactly adjacent (e.g., [1,2] and [2,3]). According to the problem, these are not considered overlapping.

10. The time complexity of your solution will be dominated by the sorting step, which is O(n log n).
