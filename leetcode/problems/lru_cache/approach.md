# Approach for LRU Cache

## Combining Hash Map and Doubly Linked List

*   **Challenge:** The core difficulty lies in achieving O(1) time complexity for both `get` and `put` operations, which involve lookup, insertion, deletion, and maintaining usage order (evicting the least recently used).
*   **Hash Map:** Provides O(1) average time for key lookups (`get`, checking existence in `put`), insertion, and deletion by key. However, it doesn't inherently maintain insertion or access order.
*   **Doubly Linked List (DLL):** Allows O(1) insertion and deletion of nodes *if you have a pointer to the node*. We can use the order in the DLL to represent the usage order (e.g., most recently used at the head, least recently used at the tail).

*   **Combined Structure:**
    1.  **Hash Map:** `map: HashMap<Key, Pointer/Reference to DLL Node>`
        *   Stores the mapping from the cache key to the actual node in the doubly linked list containing that key-value pair.
    2.  **Doubly Linked List:** `list: DoublyLinkedList<Node(key, value)>`
        *   Stores the `(key, value)` pairs.
        *   The order reflects usage: the head (or front) represents the most recently used (MRU) item, and the tail (or back) represents the least recently used (LRU) item.

*   **Helper Operations on DLL (all O(1)):**
    *   `move_to_front(node)`: Moves an existing node to the front of the list (marks it as MRU).
    *   `add_to_front(node)`: Adds a new node to the front of the list.
    *   `remove_node(node)`: Removes a specific node from the list.
    *   `remove_last()`: Removes the node at the tail of the list (LRU) and returns it (or its key).

*   **`LRUCache::new(capacity)`:**
    1.  Initialize the hash map.
    2.  Initialize the doubly linked list (potentially with dummy head/tail nodes to simplify edge cases).
    3.  Store the `capacity`.

*   **`LRUCache::get(key)`:**
    1.  Check if `key` exists in the `map`. If not, return -1.
    2.  If it exists, retrieve the `node` pointer from the `map`.
    3.  Move this `node` to the front of the `list` (`move_to_front(node)`). This marks it as the most recently used.
    4.  Return the `value` stored in the `node`.

*   **`LRUCache::put(key, value)`:**
    1.  Check if `key` exists in the `map`.
        *   **If yes (Update):**
            *   Retrieve the `node` pointer from the `map`.
            *   Update the `value` within the `node`.
            *   Move this `node` to the front of the `list` (`move_to_front(node)`).
        *   **If no (Insert):**
            *   Check if the cache is at full `capacity` (`map.len() == capacity`).
                *   If full, remove the LRU item:
                    *   Get the `lru_node` by removing it from the tail of the `list` (`remove_last()`).
                    *   Remove the `lru_node`'s key from the `map`.
            *   Create a `new_node` containing the `(key, value)` pair.
            *   Add the `new_node` to the front of the `list` (`add_to_front(new_node)`).
            *   Add the mapping `{key -> pointer_to_new_node}` to the `map`.

*   **Complexity:**
    *   Time: O(1) average for both `get` and `put`. Hash map operations are O(1) average, and DLL operations (add/remove front/back, move node given pointer) are O(1).
    *   Space: O(capacity), as we store up to `capacity` key-value pairs in the hash map and the doubly linked list.
