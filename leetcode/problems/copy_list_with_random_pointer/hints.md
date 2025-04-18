# Hints for Copy List with Random Pointer

## Key Question

*   How can you keep track of the mapping between original nodes and their corresponding newly created copies, especially when setting the `random` pointers?

## Additional Hints

*   Consider using a hash map to store the mapping between original nodes and copied nodes.
*   Think about different ways to iterate through the list. Can you do it in one pass or multiple passes?
*   Is there a way to modify the original list structure temporarily to help with the copying process, without using extra space proportional to `n` (like a hash map)?
