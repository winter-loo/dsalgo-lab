# Approach for Merge k Sorted Lists

There are several ways to approach merging `k` sorted linked lists. The goal is to do this efficiently.

## 1. Brute Force Comparison

*   **Idea:** In each step, iterate through the current head nodes of all `k` lists to find the node with the minimum value. Append this minimum node to the result list and advance the pointer of the list from which the minimum node was taken.
*   **Process:**
    1.  Create a dummy head for the result list.
    2.  While at least one list still has nodes:
        *   Initialize `min_val` to infinity and `min_idx` to -1.
        *   Iterate from `i = 0` to `k-1`:
            *   If `lists[i]` is not null and `lists[i].val < min_val`:
                *   Update `min_val = lists[i].val` and `min_idx = i`.
        *   If `min_idx` is not -1 (meaning we found a minimum node):
            *   Append `lists[min_idx]` to the result list.
            *   Advance `lists[min_idx] = lists[min_idx].next`.
        *   Else (all lists are exhausted), break the loop.
    3.  Return the `next` of the dummy head.
*   **Complexity:**
    *   Time: O(N * k), where `N` is the total number of nodes across all lists. In each step to find the next node for the result list (N steps total), we compare up to `k` nodes.
    *   Space: O(1) additional space (or O(N) if creating a new list instead of rearranging pointers).
*   **Drawback:** This approach is inefficient, especially when `k` is large.

## 2. Using a Min-Heap (Priority Queue)

*   **Idea:** Use a min-heap to efficiently find the smallest node among the current heads of all lists.
*   **Process:**
    1.  Create a min-heap. The heap will store references/pointers to `ListNode`s. The comparison logic in the heap should be based on the `val` of the nodes.
    2.  Initialize the heap by adding the head node of each non-empty list from the input `lists` array.
    3.  Create a dummy head for the result list and a `current` pointer initialized to the dummy head.
    4.  While the min-heap is not empty:
        *   Extract the node with the smallest value (`min_node`) from the heap (O(log k)).
        *   Append `min_node` to the result list (`current.next = min_node`) and advance `current` (`current = current.next`).
        *   If `min_node` has a `next` node in its original list, add `min_node.next` to the heap (O(log k)).
    5.  Return the `next` of the dummy head.
*   **Complexity:**
    *   Time: O(N log k). There are `N` nodes in total. Each heap operation (insertion and extraction) takes O(log k) time. We perform roughly `N` extractions and `N` insertions.
    *   Space: O(k). The heap stores at most `k` nodes (one from each list).
*   **Advantage:** Significantly faster than the brute-force approach when `k` is large.

## 3. Divide and Conquer (Pairwise Merging)

*   **Idea:** Repeatedly merge pairs of lists until only one list remains. This mimics the process of merge sort.
*   **Process:**
    1.  Handle the base case: If `lists` is empty or has only one list, return it.
    2.  While the number of lists is greater than 1:
        *   Create a new list `merged_lists`.
        *   Iterate through the current `lists` array in steps of 2:
            *   Take two adjacent lists, `l1 = lists[i]` and `l2 = lists[i+1]` (handle the case where `i+1` is out of bounds, taking `l2` as null/empty).
            *   Merge `l1` and `l2` using the standard two-list merge algorithm (from Problem 21. Merge Two Sorted Lists).
            *   Add the result of the merge to `merged_lists`.
        *   Replace the old `lists` array with `merged_lists`.
    3.  Return the single remaining list in `lists[0]`.
*   **Complexity:**
    *   Time: O(N log k). Merging two lists takes time proportional to the sum of their lengths. Consider the total number of nodes `N`. In each level of merging, we process all `N` nodes. The number of merging levels is O(log k).
    *   Space: O(1) if merging in place (modifying the input array and list pointers), or O(log k) recursive call stack space if implemented recursively. Could be O(N) if new lists are created at each merge step without reusing nodes.
*   **Advantage:** Often efficient and can sometimes be implemented with lower constant factors than the heap approach, depending on the merge implementation.

## Conclusion

Both the Min-Heap and Divide and Conquer approaches offer optimal O(N log k) time complexity. The choice between them might depend on implementation details or specific language features (e.g., built-in heap support). The brute-force O(N * k) approach should generally be avoided for larger `k`.
