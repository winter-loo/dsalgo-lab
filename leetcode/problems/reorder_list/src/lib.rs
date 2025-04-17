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
        Self::reorder_list_deque(head)
    }

    // benchmarking shows that this is the best method
    pub fn reorder_list_deque(head: &mut Option<Box<ListNode>>) {
        use std::collections::VecDeque;
        if head.is_none() {
            return;
        }

        let mut tail = &mut head.as_mut().unwrap().next;
        let mut head = tail.take();
        let mut deque = VecDeque::new();
        while head.is_some() {
            let next = head.as_mut().unwrap().next.take();
            deque.push_back(head);
            head = next;
        }
        let mut flag = false;
        while !deque.is_empty() {
            *tail = if flag {
                deque.pop_front().unwrap()
            } else {
                deque.pop_back().unwrap()
            };
            tail = &mut tail.as_mut().unwrap().next;
            flag = !flag;
        }
    }

    // benchmarking shows this is the slowest method
    pub fn reorder_list_two_pointer(head: &mut Option<Box<ListNode>>) {
        #[inline(always)]
        fn get_list_middle(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let (mut fast, mut slow) = (&head.clone(), head);
            while fast.is_some() {
                fast = &(fast.as_ref().unwrap().next);
                if fast.is_some() {
                    fast = &fast.as_ref().unwrap().next;   
                    slow = &mut(slow.as_mut().unwrap().next); 
                }
            }
            slow.as_mut().unwrap().next.take()
        }

        #[inline(always)]
        fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut prev = None;
            while let Some(mut curr) = head {
                head = curr.next;
                curr.next = prev;
                prev = Some(curr);
            }
            prev
        }

        #[inline(always)]
        fn merge_lists(head1: &mut Option<Box<ListNode>>, head2: Option<Box<ListNode>>) {
            let mut h1 = head1;
            let mut h2 = head2;
            while h1.is_some() && h2.is_some() {
                let h1next = h1.as_mut().unwrap().next.take();
                let h2next = h2.as_mut().unwrap().next.take();
                h1.as_mut().unwrap().next = h2;
                h1.as_mut().unwrap().next.as_mut().unwrap().next = h1next;
                h1 = &mut(h1.as_mut().unwrap().next.as_mut().unwrap().next);
                h2 = h2next;
            }
        }

        if head.is_none() {
            return;
        }

        let mut head2 = get_list_middle(head);
        head2 = reverse_list(head2);
        merge_lists(head, head2);
    }


    // benchmarking shows this is an averange method
    pub fn reorder_list_couting(head: &mut Option<Box<ListNode>>) {
        
        let mut href = head.as_ref();
        let mut n = 0;

        while let Some(node) = href.take() {
            href = node.next.as_ref();
            n += 1;
        }

        let mut hmut = head.as_mut();
        let mut cnt = 1;

        while cnt < (n + 1) / 2 {
            if let Some(node) = hmut.take() {
                hmut = node.next.as_mut();
                cnt += 1;
            } else {
                break;
            }
        }

        match hmut.take() {
            None => (),
            Some(node) => {
                let tail = Self::reverse(node.next.take());
                if let Some(node) = head {
                    node.next = Self::merge_two_lists(tail, node.next.take());
                }
            }
        }
    }

    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1.as_mut(), l2.as_mut()) {
            (None, None) => None,
            (Some(_), None) => l1,
            (None, Some(_)) => l2,
            (Some(node1), Some(_)) => {
                node1.next = Self::merge_two_lists(l2, node1.next.take());
                l1
            }
        }
    }

    pub fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;

        while let Some(mut curr_inner) = curr.take() {
            curr = curr_inner.next.take();
            curr_inner.next = prev.take();
            prev = Some(curr_inner);
        }

        prev
    }
}
