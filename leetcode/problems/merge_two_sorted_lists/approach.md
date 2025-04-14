# Approach: Merge Two Sorted Lists

## Intuition
The key insight for this problem is to leverage the fact that both input lists are already sorted. We can merge them by comparing the current nodes of both lists and selecting the smaller one to add to our result list, then moving to the next node in that list.

## Multiple Approaches

### Approach 1: Iterative Approach
1. Create a dummy head node to simplify edge cases.
2. Maintain a pointer `current` to build the merged list.
3. While both lists have nodes:
   - Compare the values of the current nodes in both lists.
   - Append the smaller node to the merged list.
   - Move to the next node in the list from which we took the node.
4. After one list is exhausted, append the remaining nodes of the other list to the merged list.
5. Return the next of the dummy head (the actual head of the merged list).

**Time Complexity**: O(n + m) - Where n and m are the lengths of the two lists. We process each node exactly once.
**Space Complexity**: O(1) - We only use a constant amount of extra space regardless of input size.

### Approach 2: Recursive Approach
1. Base cases:
   - If list1 is null, return list2.
   - If list2 is null, return list1.
2. Compare the values of the current nodes of both lists:
   - If list1.val <= list2.val, set list1.next to the result of recursively merging list1.next and list2, and return list1.
   - Otherwise, set list2.next to the result of recursively merging list1 and list2.next, and return list2.

**Time Complexity**: O(n + m) - Where n and m are the lengths of the two lists. We process each node exactly once.
**Space Complexity**: O(n + m) - Due to the recursive call stack.

### Approach 3: In-place Merge with Pointer Manipulation
1. Determine which list has the smaller head value; this will be our result list.
2. Maintain two pointers: `prev` (points to the last node in the merged list) and `current` (points to the current node being considered for merging).
3. While both lists have nodes:
   - If list1.val <= list2.val, update `prev` to list1 and move list1 to list1.next.
   - Otherwise, insert list2 between `prev` and list1, update `prev` to list2, and move list2 to list2.next.
4. After one list is exhausted, append the remaining nodes of the other list to the merged list.
5. Return the head of the result list.

**Time Complexity**: O(n + m) - Where n and m are the lengths of the two lists. We process each node exactly once.
**Space Complexity**: O(1) - We only use a constant amount of extra space regardless of input size.

## Implementation Notes
- The iterative approach is generally more space-efficient than the recursive approach, as it doesn't use the call stack.
- Be careful with edge cases, such as when one or both lists are empty.
- When implementing the in-place merge, be careful with pointer manipulation to avoid creating cycles or losing nodes.
- The dummy head node in the iterative approach simplifies handling edge cases, especially when the merged list might start with a node from either list1 or list2.

## Topics
- Linked List
- Recursion
