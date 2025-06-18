# Hints for Merge Intervals

1. Think about how to determine if two intervals overlap. Two intervals `[a, b]` and `[c, d]` overlap if `b >= c` and `a <= d`.

2. Consider sorting the intervals by their start times. This makes it easier to identify overlapping intervals.

3. After sorting, you can merge intervals in a single pass by comparing each interval with the last interval in your result.

4. If the current interval overlaps with the last interval in your result, merge them by updating the end time of the last interval.

5. If the current interval doesn't overlap with the last interval in your result, add it to your result.

6. Be careful with the merging condition. When merging intervals, the end time of the merged interval should be the maximum of the end times of the overlapping intervals.

7. Remember to handle edge cases, such as when the input array is empty or contains only one interval.

8. Consider what happens when intervals are exactly adjacent (e.g., [1,3] and [3,5]). According to the problem, these should be merged into [1,5].

9. The time complexity of your solution will be dominated by the sorting step, which is O(n log n).

10. You can optimize the space complexity by sorting the input array in-place, but this will modify the input.
