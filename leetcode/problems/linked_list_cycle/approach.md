# Approach: Linked List Cycle

## Intuition
The key insight for this problem is to detect if there's a cycle in a linked list. If a cycle exists, we can use the fact that traversing the list will eventually lead to revisiting a node. There are several ways to detect this, with the most efficient being the two-pointer (fast and slow) approach.

## Multiple Approaches

### Approach 1: Floyd's Cycle-Finding Algorithm (Two Pointers)
1. Use two pointers, `slow` and `fast`, both initially pointing to the head of the list.
2. Move `slow` one step at a time and `fast` two steps at a time.
3. If there is a cycle, the two pointers will eventually meet at some point.
4. If there is no cycle, the `fast` pointer will reach the end of the list.

**Time Complexity**: O(n) - In the worst case, we might need to traverse the entire list once.
**Space Complexity**: O(1) - We only use two pointers regardless of the input size.

### Approach 2: Hash Set
1. Traverse the linked list and store each node in a hash set.
2. Before adding a node to the hash set, check if it's already present.
3. If it is, then there's a cycle.
4. If we reach the end of the list without finding a duplicate node, there's no cycle.

**Time Complexity**: O(n) - We might need to traverse the entire list once.
**Space Complexity**: O(n) - In the worst case, we might need to store all nodes in the hash set.

### Approach 3: Marking Visited Nodes
1. Traverse the linked list and mark each node as visited by modifying its value or structure.
2. If we encounter a node that's already marked as visited, there's a cycle.
3. If we reach the end of the list without finding a marked node, there's no cycle.

**Time Complexity**: O(n) - We might need to traverse the entire list once.
**Space Complexity**: O(1) - We modify the existing nodes without using extra space.

## Implementation Notes
- The two-pointer approach (Floyd's Cycle-Finding Algorithm) is the most efficient in terms of both time and space complexity.
- The hash set approach is straightforward but uses extra space.
- The marking approach modifies the original list, which might not be desirable in some cases.
- Edge cases to consider:
  - Empty list
  - List with only one node
  - List with no cycle
  - List with a cycle that starts at the head
  - List with a cycle that starts at a node other than the head
- In the two-pointer approach, if there's a cycle, the pointers will definitely meet. This is because once the fast pointer enters the cycle, it will eventually catch up to the slow pointer.

## Topics
- Linked List
- Two Pointers
- Hash Table
