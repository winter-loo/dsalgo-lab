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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut help = Box::new(ListNode { val: 0, next: head });
        let mut fast = &(*help) as *const ListNode;
        for _ in 0..n {
            unsafe {
                if (*fast).next.is_none() {
                    return None;
                }
                fast = (*fast).next.as_deref().unwrap();
            }
        }
        let mut slow = &mut (*help) as *mut ListNode;
        unsafe {
            while (*fast).next.is_some() {
                fast = (*fast).next.as_deref().unwrap();
                slow = (*slow).next.as_deref_mut().unwrap();
            }
            // now fast is the last node
            let to_delete = (*slow).next.take();
            (*slow).next = to_delete.unwrap().next;
        }
        help.next
    }

    pub fn remove_nth_from_end_rec(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        fn recursive(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
            match head {
                None => (None, 1),
                Some(mut node) => {
                    let (prev, i) = recursive(node.next, n);
                    if i == n {
                        return (prev, i + 1);
                    } else {
                        node.next = prev;
                        return (Some(node), i + 1);
                    }
                }
            }
        }
        recursive(head, n).0
    }
}
