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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return;
        }
        let mut slow = &head;
        let mut fast = &head.as_ref().unwrap().next;
        while fast.is_some() && fast.as_ref.unwrap().next.is_some() {
            slow = &slow.next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
    }
}
