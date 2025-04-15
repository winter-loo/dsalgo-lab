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
//```go
// type ListNode struct {
//     Val int
//     Next *ListNode
// }
//
// func hasCycle(head *ListNode) bool {
//     fast, slow := head, head
//     for {
//         if fast == nil || fast.Next == nil {
//             return false
//         }
//         fast = fast.Next.Next
//         slow = slow.Next
//         if fast == slow {
//             return true;
//         }
//     }
//     return false
// }
//```
impl Solution {
    pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
        let mut slow = &head;
        let mut fast = &head;

        loop {
            if fast.is_none() || fast.as_ref().unwrap().next.is_none() {
                return false;
            }
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
            slow = &slow.as_ref().unwrap().next;
            if fast == slow {
                return true;
            }
        }
        unreachable!()
    }
}
