# Approach for Add Two Numbers

## Simulation with Carry

*   **Intuition:** The problem asks us to add two numbers represented by linked lists where digits are stored in reverse order. This setup perfectly matches how we perform addition by hand, starting from the least significant digit (the head of the list).

*   **Algorithm:**
    1.  **Initialization:**
        *   Create a `dummy_head` node for the result list. This simplifies adding the first digit without special checks.
        *   Initialize a `current` pointer to `dummy_head`.
        *   Initialize a `carry` variable to `0`.
        *   Initialize pointers `p1` and `p2` to the heads of the input lists `l1` and `l2`.
    2.  **Iteration:**
        *   Loop as long as `p1` is not null, `p2` is not null, or there is a `carry` left over (`carry > 0`).
        *   Get the value `v1` from the node pointed to by `p1` (or 0 if `p1` is null).
        *   Get the value `v2` from the node pointed to by `p2` (or 0 if `p2` is null).
        *   Calculate the `sum = v1 + v2 + carry`.
        *   Update the `carry` for the next iteration: `carry = sum / 10`.
        *   Determine the digit for the new node: `digit = sum % 10`.
        *   Create a `new_node` with the value `digit`.
        *   Append the `new_node` to the result list: `current.next = Some(new_node)`.
        *   Advance the `current` pointer: `current = current.next.as_mut().unwrap()` (or equivalent reference update).
        *   Advance `p1` and `p2` to their next nodes if they are not null.
    3.  **Result:**
        *   The sum list is linked starting from `dummy_head.next`. Return `dummy_head.next`.

*   **Complexity:**
    *   Time: O(max(N, M)), where N and M are the lengths of `l1` and `l2` respectively. We traverse each list at most once.
    *   Space: O(max(N, M)). The length of the new list is at most `max(N, M) + 1` (to accommodate a potential final carry).
