# Hints for Partition Labels

1. Think about what it means for a letter to appear in at most one part - all occurrences of that letter must be in the same partition.

2. For each letter, you need to know its first and last occurrence in the string. Particularly, the last occurrence is crucial.

3. Consider using a hash map or array to store the last occurrence index of each character.

4. As you iterate through the string, keep track of the furthest index you need to include in the current partition.

5. When you reach the furthest index for the current partition, you've completed a partition.

6. Remember that if you encounter a character that extends beyond your current partition boundary, you need to extend the boundary.

7. The greedy approach works here: always extend the current partition as needed, and only create a new partition when you've reached the end of the current one.

8. Be careful with the calculation of partition sizes - remember that indices are zero-based.

9. For the result, you need to return the sizes of the partitions, not their start or end indices.

10. Test your solution with simple cases first, like strings with all distinct characters or strings with many repeated characters.
