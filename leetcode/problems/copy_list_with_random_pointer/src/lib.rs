#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub val: i32,
    pub next: Option<Box<Node>>,
    pub random: Option<Box<Node>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            val,
            next: None,
            random: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn copy_random_list(head: Option<Box<Node>>) -> Option<Box<Node>> {
        None
    }
}
