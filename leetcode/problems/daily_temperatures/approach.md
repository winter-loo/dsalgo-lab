# Approach: Monotonic Stack

## Intuition
For each day, we need to find the next day with a higher temperature. This is a perfect use case for a monotonic stack, which helps us find the "next greater element" efficiently.

## Algorithm
1. Initialize an array `answer` of the same length as `temperatures`, filled with zeros.
2. Maintain a monotonic decreasing stack of indices (not temperatures).
3. Iterate through the temperatures:
   - While the stack is not empty and the current temperature is higher than the temperature at the index at the top of the stack:
     - Pop the top index from the stack.
     - Calculate the number of days between the current index and the popped index.
     - Update the answer for the popped index with this number of days.
   - Push the current index onto the stack.
4. Return the answer array.

## Complexity Analysis
- Time Complexity: O(n) where n is the length of the temperatures array
  - Each index is pushed and popped at most once from the stack
  
- Space Complexity: O(n) in the worst case
  - The stack can contain up to n elements in the worst case (e.g., for a strictly decreasing temperature array)

## Implementation Notes
- Use a Vec as a stack in Rust
- Remember that we're storing indices in the stack, not the actual temperatures
- Initialize the answer array with zeros
- Be careful with the edge cases, especially when the stack is empty
