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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = list1;
        let mut l2 = list2;

        let mut dummy = ListNode::new(0);
        let mut p = &mut dummy.next;

        while l1.is_some() && l2.is_some() {
            let node1 = l1.as_ref().unwrap();
            let node2 = l2.as_ref().unwrap();

            if node1.val <= node2.val { // Use <= to maintain stability if needed
                // Move node from l1 to *p
                *p = l1.take(); 
                // Update l1 to point to the rest of the original list1
                l1 = p.as_mut().unwrap().next.take();
            } else {
                // Move node from l2 to *p
                *p = l2.take();
                // Update l2 to point to the rest of the original list2
                l2 = p.as_mut().unwrap().next.take();
            }
            // Advance p to point to the *next* field of the node just added
            p = &mut p.as_mut().unwrap().next;
        }

        // After the loop, at least one list is empty.
        // Append the remaining nodes from the non-empty list.
        // `or` takes the first Some, or None if both are None.
        *p = l1.or(l2);

        dummy.next
    }
}
