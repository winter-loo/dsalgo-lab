# Approach: Two Pointers

## Intuition
The key insight for this problem is that the amount of water a container can hold is determined by the shorter of the two lines and the distance between them. We want to maximize the area, which is calculated as: min(height[left], height[right]) * (right - left).

## Algorithm
1. Initialize two pointers, one at the beginning of the array (left) and one at the end (right).
2. Calculate the area between the two pointers and keep track of the maximum area seen so far.
3. Move the pointer that points to the shorter line inward (since moving the pointer with the taller line would only decrease the area or keep it the same).
4. Continue this process until the pointers meet.

## Complexity Analysis
- Time Complexity: O(n) where n is the length of the height array
  - We process each element at most once
  
- Space Complexity: O(1)
  - We only use a constant amount of extra space regardless of the input size

## Implementation Notes
- The area is calculated as min(height[left], height[right]) * (right - left)
- Always move the pointer pointing to the shorter line inward
- If both lines have the same height, you can move either pointer (or both, but one at a time is simpler)
- Be careful with edge cases like an array with only two elements
