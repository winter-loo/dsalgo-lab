# Hints for LRU Cache

## Key Questions

*   How can you achieve O(1) time complexity for checking if a key exists in the cache?
*   How can you efficiently identify and remove the *least* recently used item in O(1) time when the cache is full?
*   How can you efficiently move an existing item to the *most* recently used position in O(1) time after it's accessed (`get` or `put`)?

## Additional Hints

*   Think about combining two data structures. One structure is needed for fast key lookups, and another is needed to maintain the order of usage.
*   A standard hash map provides O(1) average time complexity for lookups, insertions, and deletions by key.
*   A standard singly linked list allows O(1) insertion/deletion at the head/tail, but removing an arbitrary node or moving a node requires traversing the list (not O(1)).
*   Consider using a **doubly linked list**. Why might this be advantageous for removing *any* node in O(1) time, provided you have a direct reference to it?
*   How can the hash map and the doubly linked list work together? What should the hash map store as its value? (Hint: Not just the data value, but something related to the list).
*   When an item is accessed (`get`) or updated/inserted (`put`), it needs to become the most recently used. In the context of a doubly linked list, where should the most recently used item reside? How do you move a node there in O(1)?
*   Remember to handle the capacity constraint: when adding a new item to a full cache, the least recently used item (which end of the doubly linked list?) must be removed first. This removal needs to be reflected in both the list and the hash map.
