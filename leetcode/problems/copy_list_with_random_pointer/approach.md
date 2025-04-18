# Approaches for Copy List with Random Pointer

## 1. Hash Map Approach (Two Passes)

*   **Intuition:** Use a hash map to store the mapping between each original node and its newly created copy. This allows easy access to the copied node corresponding to any original node when setting the `random` pointers.
*   **Pass 1:** Iterate through the original list. For each node, create a deep copy (only copying the `val`) and store the mapping `original_node -> copied_node` in the hash map.
*   **Pass 2:** Iterate through the original list again. For each `original_node`, find its `copied_node` using the hash map. Set `copied_node.next` to `map[original_node.next]` and `copied_node.random` to `map[original_node.random]`. Handle `null` pointers appropriately.
*   **Complexity:**
    *   Time: O(N), as we iterate through the list twice.
    *   Space: O(N), for the hash map.

## 2. Interweaving Nodes Approach (O(1) Space)

*   **Intuition:** Avoid using extra space by modifying the original list structure temporarily. Weave the copied nodes into the original list.
*   **Pass 1:** Iterate through the original list. For each `node`, create its copy `copy`. Insert the `copy` immediately after the `node`. So, the list becomes `node -> copy -> node.next -> copy.next -> ...`.
*   **Pass 2:** Iterate through the modified list. For each `node`, its `copy` is `node.next`. Set `copy.random` based on `node.random`. Specifically, `copy.random = node.random.next` (if `node.random` is not null). This works because `node.random.next` is the copy of `node.random`.
*   **Pass 3:** Iterate through the modified list again and decouple the original and copied lists. Restore the original list's `next` pointers and set the copied list's `next` pointers correctly.
*   **Complexity:**
    *   Time: O(N), as we iterate through the list three times.
    *   Space: O(1), as we only use a few pointers and modify the list in place.

## 3. Hash Map Approach (Single Pass - Recursive)

*   **Intuition:** Use recursion combined with a hash map to handle the creation and linking of nodes in a single pass.
*   **Function:** Define a recursive function `copyRandomListHelper(node, map)`.
*   **Base Case:** If `node` is null, return null.
*   **Memoization:** If `node` is already in the `map`, return `map[node]`.
*   **Recursive Step:**
    1. Create a `new_node` with `node.val`.
    2. Add the mapping `node -> new_node` to the `map`.
    3. Recursively call `copyRandomListHelper` for `node.next` and `node.random`.
    4. Set `new_node.next` and `new_node.random` to the results of the recursive calls.
    5. Return `new_node`.
*   **Complexity:**
    *   Time: O(N), each node is visited once.
    *   Space: O(N), for the hash map and recursion stack (in the worst case, for a skewed list).
