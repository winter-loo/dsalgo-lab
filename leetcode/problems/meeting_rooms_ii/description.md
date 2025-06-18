# Meeting Rooms II

[LeetCode Link](https://leetcode.com/problems/meeting-rooms-ii/)

Given an array of meeting time intervals `intervals` where `intervals[i] = [starti, endi]`, return the minimum number of conference rooms required.

**Example 1:**
```
Input: intervals = [[0,30],[5,10],[15,20]]
Output: 2
Explanation: We need two conference rooms.
Room 1: [0,30]
Room 2: [5,10], [15,20]
```

**Example 2:**
```
Input: intervals = [[7,10],[2,4]]
Output: 1
Explanation: We only need one conference room because no two meetings overlap.
```

**Constraints:**
- `1 <= intervals.length <= 10^4`
- `0 <= starti < endi <= 10^6`
