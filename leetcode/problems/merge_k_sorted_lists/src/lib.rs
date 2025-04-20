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

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut p = &mut dummy;
        let mut heap = BinaryHeap::new();
        let mut lists = lists;
        for node in lists.iter_mut() {
            if node.is_some() {
                heap.push(Reverse(node.take()));
            }
        }
        while let Some(Reverse(mut node)) = heap.pop() {
            let next = node.as_mut().unwrap().next.take();
            p.next = node;
            p = p.next.as_mut().unwrap();
            if next.is_some() {
                heap.push(Reverse(next));
            }
        }
        dummy.next
    }

    pub fn merge_k_lists_one_by_one(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        let mut lists = lists;
        let mut first = lists[0].take();
        for other in &mut lists[1..] {
            first = merge_two_sorted_lists(first, other.take());
        }
        first
    }
}

fn merge_two_sorted_lists(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    let mut p = &mut dummy;
    while l1.is_some() && l2.is_some() {
        let n1 = l1.as_ref().unwrap();
        let n2 = l2.as_ref().unwrap();
        p.next = Some(Box::new(ListNode::new(n1.val.min(n2.val))));
        p = p.next.as_mut().unwrap();
        if n1.val < n2.val {
            l1 = l1.unwrap().next;
        } else {
            l2 = l2.unwrap().next;
        }
    }
    p.next = l1.or(l2);
    dummy.next
}
