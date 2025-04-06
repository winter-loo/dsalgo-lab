# Approach: Stack with Minimum Tracking

## Intuition
The key challenge in this problem is maintaining constant time access to the minimum element in the stack. A regular stack can handle push, pop, and top operations in O(1) time, but finding the minimum would typically require O(n) time to scan all elements.

## Algorithm
There are several approaches to solve this problem:

### Approach 1: Pair Stack
1. Store pairs of values in the stack: (value, current_minimum)
2. When pushing a new element:
   - Compare it with the current minimum (or use the new value as minimum if the stack is empty)
   - Push the pair (value, min(value, current_minimum)) onto the stack
3. When popping, the minimum is automatically updated since each element knows the minimum at its level
4. The top and getMin operations simply return the appropriate part of the pair at the top of the stack

### Approach 2: Two Stacks
1. Maintain two separate stacks:
   - A regular stack for the values
   - A "minimum stack" that keeps track of the minimum values
2. When pushing a new element:
   - Always push it to the regular stack
   - Push it to the minimum stack only if it's less than or equal to the current minimum (or if the minimum stack is empty)
3. When popping:
   - Always pop from the regular stack
   - If the popped value equals the top of the minimum stack, also pop from the minimum stack
4. The top operation returns the top of the regular stack
5. The getMin operation returns the top of the minimum stack

## Complexity Analysis
- Time Complexity: O(1) for all operations
  - All operations (push, pop, top, getMin) are performed in constant time
  
- Space Complexity: O(n) where n is the number of elements in the stack
  - In the worst case, we need to store all elements in the stack

## Implementation Notes
- Be careful with the edge cases, especially when the stack is empty
- For the two-stack approach, ensure that the minimum stack is properly maintained during pop operations
- Consider using Option<i32> in Rust to handle potential empty stack situations
