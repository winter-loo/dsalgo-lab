// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut p1_cursor = &mut l1;
        let mut p2_cursor = l2;
        let mut carry = 0;

        loop {
            if p1_cursor.is_none() && p2_cursor.is_none() && carry == 0 {
                break;
            }

            let val1 = p1_cursor.as_ref().map_or(0, |node| node.val);
            let val2 = p2_cursor.as_ref().map_or(0, |node| node.val);

            let sum = val1 + val2 + carry;
            carry = sum / 10;
            let digit = sum % 10;

            let node1 = p1_cursor.get_or_insert_with(|| Box::new(ListNode::new(0)));

            node1.val = digit;

            p1_cursor = &mut node1.next;

            p2_cursor = p2_cursor.and_then(|n| n.next);
        }

        l1
    }

    pub fn add_two_numbers_new_box(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(0); // Dummy node to simplify list construction
        let mut current = &mut dummy_head; // Mutable reference to the tail of the new list
        let mut p1 = l1;
        let mut p2 = l2;
        let mut carry = 0;

        while p1.is_some() || p2.is_some() || carry != 0 {
            let val1 = p1.as_ref().map_or(0, |node| node.val);
            let val2 = p2.as_ref().map_or(0, |node| node.val);

            let sum = val1 + val2 + carry;
            carry = sum / 10;
            let digit = sum % 10;

            current.next = Some(Box::new(ListNode::new(digit)));
            current = current.next.as_mut().unwrap(); // Move current forward

            // Advance list pointers
            if let Some(node) = p1 {
                p1 = node.next;
            }
            if let Some(node) = p2 {
                p2 = node.next;
            }
        }

        dummy_head.next // Return the actual head of the new list
    }

    pub fn add_two_numbers_initial(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut sum = ListNode::new(0);
        let mut p = &mut sum;

        let mut p1 = l1.as_ref();
        let mut p2 = l2.as_ref();
        let mut carry = 0;

        while p1.is_some() && p2.is_some() {
            let mut val = p1.unwrap().val + p2.unwrap().val + carry;
            if val >= 10 {
                val -= 10;
                carry = 1;
            } else {
                carry = 0;
            }
            p.next = Some(Box::new(ListNode::new(val)));
            p = p.next.as_mut().unwrap();

            p1 = p1.unwrap().next.as_ref();
            p2 = p2.unwrap().next.as_ref();
        }

        p.next = p1.or(p2).cloned();

        if carry == 1 {
            while p.next.is_some() {
                let next = p.next.as_mut().unwrap();
                next.val += 1;
                if next.val < 10 {
                    carry = 0;
                    break;
                }
                next.val -= 10;
                carry = 1;
                p = p.next.as_mut().unwrap();
            }
            if carry == 1 {
                p.next = Some(Box::new(ListNode::new(1)));
            }
        }

        sum.next
    }
}
