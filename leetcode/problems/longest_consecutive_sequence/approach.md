# Approach: Longest Consecutive Sequence

## Intuition
The key insight for solving this problem in O(n) time is to recognize that we can use a hash set for O(1) lookups to check if a number is part of a sequence. Instead of sorting the array (which would be O(n log n)), we can identify the start of each sequence and count its length.

## Approach
1. First, insert all numbers into a hash set for O(1) lookup.
2. For each number in the array, check if it's the start of a sequence by determining if the number - 1 is not in the set.
3. If the number is the start of a sequence, count how many consecutive numbers follow it by incrementally checking if the next number exists in the set.
4. Keep track of the longest sequence found.

This approach ensures that we only do one pass through the array (O(n)) and each number is part of at most one sequence count (also O(n) in total).

## Complexity Analysis
- **Time Complexity**: O(n), where n is the number of elements in the array. Although there are nested loops, the inner loop will execute at most n times across all iterations of the outer loop.
- **Space Complexity**: O(n) for storing the numbers in the hash set.

## Edge Cases
- Empty array: Return 0 as there is no sequence.
- Duplicate numbers: These don't impact the length of consecutive sequences, so they can be handled naturally by the hash set.
- Negative numbers: The algorithm works the same for negative numbers as for positive ones.

## Optimizations
- We can skip numbers that we already know are part of a sequence (i.e., they are not the start of a new sequence).
- If the array size is small, it might be more efficient to use a sorting approach for simplicity and to avoid the overhead of a hash set. 