# Approach: Stack-based Evaluation

## Intuition
Reverse Polish Notation (RPN), also known as postfix notation, is a mathematical notation where every operator follows all of its operands. This format is particularly well-suited for stack-based evaluation.

The key insight is that when we encounter an operator, we need to apply it to the two most recent operands.

## Algorithm
1. Initialize an empty stack to store operands.
2. Iterate through each token in the input array:
   - If the token is a number, push it onto the stack.
   - If the token is an operator (+, -, *, /):
     - Pop the top two values from the stack (the second value popped is the first operand).
     - Perform the operation on these two values.
     - Push the result back onto the stack.
3. After processing all tokens, the stack should contain exactly one value, which is the final result.

## Complexity Analysis
- Time Complexity: O(n) where n is the number of tokens
  - We process each token exactly once
  
- Space Complexity: O(n) in the worst case
  - In the worst case, the stack could grow to the size of the input (e.g., if all tokens are numbers)

## Implementation Notes
- Use a Vec as a stack in Rust
- Be careful with the order of operands for subtraction and division
- Remember that integer division should truncate toward zero
- Handle parsing of string tokens to integers
- Check for edge cases like empty input or invalid expressions
