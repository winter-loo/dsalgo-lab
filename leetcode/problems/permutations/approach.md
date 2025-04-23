# Approach

To generate all possible permutations of a given array of distinct integers, we need to explore all ways to arrange the elements.

## Strategy
There are several approaches to solve this problem:

1. **Backtracking**: Recursively build permutations by selecting one element at a time.
2. **Iterative**: Build permutations incrementally by inserting each new element at all possible positions.
3. **Heap's Algorithm**: An efficient algorithm for generating all permutations.

## Steps for Backtracking Approach
1. Start with an empty permutation.
2. For each recursive call:
   - If the current permutation has the same length as the input array, add it to the result.
   - Otherwise, try adding each unused element from the input array to the current permutation.
   - Recursively generate permutations with the updated current permutation.
   - After the recursive call, remove the last element (backtrack) to try other elements.

## Steps for Iterative Approach
1. Start with a single permutation containing just the first element.
2. For each remaining element:
   - Create a new set of permutations by inserting the current element at all possible positions in each existing permutation.
   - Replace the existing permutations with these new permutations.

## Implementation Notes
- The backtracking approach is intuitive and commonly used.
- We need to keep track of which elements have been used in the current permutation.
- Since all integers in the input are unique, we don't need to worry about duplicate permutations.

## Time and Space Complexity
- **Time Complexity**: O(n!) where n is the length of the input array. This is because there are n! possible permutations.
- **Space Complexity**: O(n) for the recursion stack in the backtracking approach, plus O(n!) to store all permutations.
