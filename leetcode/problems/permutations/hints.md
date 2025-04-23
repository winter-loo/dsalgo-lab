# Hints for Permutations

1. Think about how to systematically generate all permutations. For each position in the permutation, you need to try all available numbers.

2. Backtracking is a natural approach for this problem. Start with an empty permutation and recursively add one element at a time.

3. You'll need to keep track of which elements have already been used in the current permutation. Consider using a boolean array or a set to mark used elements.

4. Another approach is to swap elements in the original array. For each position, try swapping the current element with each element that comes after it, then recursively generate permutations for the rest of the array.

5. Remember that the input array contains distinct integers, so you don't need to worry about duplicate permutations.

6. The total number of permutations for an array of length n is n! (n factorial). Make sure your solution can handle this efficiently.
