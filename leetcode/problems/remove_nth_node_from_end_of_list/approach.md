# Approach: Remove Nth Node From End of List

## Intuition
The key insight for this problem is to find the nth node from the end of the list efficiently. The challenge is that we don't know the length of the list in advance, and we want to do this in a single pass if possible.

## Multiple Approaches

### Approach 1: Two-Pass Solution
1. First pass: Count the total number of nodes in the list.
2. Second pass: Traverse to the (length - n)th node from the beginning and remove the next node.

**Time Complexity**: O(n) - We traverse the list twice.
**Space Complexity**: O(1) - We only use a constant amount of extra space.

### Approach 2: One-Pass Solution with Two Pointers
1. Use two pointers, `fast` and `slow`, both initially pointing to a dummy node placed before the head.
2. Move `fast` n nodes ahead.
3. Move both `fast` and `slow` one node at a time until `fast` reaches the end of the list.
4. At this point, `slow` is pointing to the node just before the one we want to remove.
5. Remove the next node by updating the next pointer of `slow`.

**Time Complexity**: O(n) - We traverse the list once.
**Space Complexity**: O(1) - We only use a constant amount of extra space.

### Approach 3: Using a Stack
1. Push all nodes onto a stack while traversing the list.
2. Pop n nodes from the stack.
3. The next node to pop is the one we want to remove.
4. Update the next pointer of this node to skip the node to be removed.

**Time Complexity**: O(n) - We traverse the list once.
**Space Complexity**: O(n) - We use a stack to store all nodes.

## Implementation Notes
- The one-pass solution with two pointers is the most efficient in terms of both time and space complexity.
- Using a dummy head node simplifies edge cases, especially when removing the first node.
- Be careful with the indexing when removing nodes. The slow pointer should point to the node just before the one to be removed.
- Edge cases to consider:
  - Removing the first node (head)
  - Removing the last node
  - List with only one node
  - n equals the length of the list
- The problem guarantees that n is valid (1 <= n <= sz), so we don't need to handle cases where n is out of bounds.

## Topics
- Linked List
- Two Pointers
