// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // Helper to check if there are at least k nodes
        fn has_k(mut node: &Option<Box<ListNode>>, k: i32) -> bool {
            let mut count = 0;
            while let Some(n) = node {
                count += 1;
                if count == k {
                    return true;
                }
                node = &n.next;
            }
            false
        }

        // Helper to reverse k nodes, returns (new_head, next_group_head)
        fn reverse_k(
            head: Option<Box<ListNode>>,
            k: i32,
        ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
            let mut prev = None;
            let mut curr = head;
            let mut count = 0;
            while let Some(mut node) = curr {
                if count == k {
                    return (prev, Some(node));
                }
                curr = node.next.take();
                node.next = prev;
                prev = Some(node);
                count += 1;
            }
            (prev, None)
        }

        if k <= 1 || head.is_none() {
            return head;
        }
        if !has_k(&head, k) {
            return head;
        }
        // Reverse first k nodes
        let mut tail = head;
        let curr = tail.take();
        let (mut new_head, next_group_head) = reverse_k(curr, k);
        // Recursively process the rest
        if let Some(ref mut tail_node) = new_head {
            let mut tail_ptr = tail_node.as_mut();
            // Move to the end of reversed part
            while let Some(ref mut next) = tail_ptr.next {
                tail_ptr = next.as_mut();
            }
            tail_ptr.next = Solution::reverse_k_group(next_group_head, k);
        }
        new_head
    }
}
