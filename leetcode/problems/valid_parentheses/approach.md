# Approach: Stack-based Validation

## Intuition
This problem is a classic example of using a stack data structure. The key insight is that parentheses must be balanced and properly nested, which naturally fits a Last-In-First-Out (LIFO) structure like a stack.

## Algorithm
1. Initialize an empty stack to keep track of opening brackets.
2. Iterate through each character in the string:
   - If the character is an opening bracket (`(`, `{`, or `[`), push it onto the stack.
   - If the character is a closing bracket (`)`, `}`, or `]`), check if the stack is empty:
     - If the stack is empty, return false (no matching opening bracket).
     - If the stack is not empty, pop the top element and check if it matches the current closing bracket:
       - If it doesn't match, return false.
3. After processing all characters, check if the stack is empty:
   - If the stack is empty, return true (all brackets were properly closed).
   - If the stack is not empty, return false (some opening brackets were not closed).

## Complexity Analysis
- Time Complexity: O(n) where n is the length of the input string
  - We process each character exactly once
  
- Space Complexity: O(n) in the worst case
  - In the worst case (all opening brackets), we would push all characters onto the stack

## Implementation Notes
- Use a Vec as a stack in Rust
- Create a mapping between closing brackets and their corresponding opening brackets
- Handle edge cases like an empty string or a string with odd length
