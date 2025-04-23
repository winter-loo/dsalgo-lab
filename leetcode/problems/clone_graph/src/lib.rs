use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

// Definition for a Node.
#[derive(Debug, Clone)]
pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            neighbors: vec![],
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        // TODO: Implement the function
        None
    }
}
