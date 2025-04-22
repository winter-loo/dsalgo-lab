// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
pub struct Codec {}

/**
 * Codec is a structure that implements methods for serializing and deserializing a binary tree.
 */
impl Codec {
    pub fn new() -> Self {
        Codec {}
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut values = vec![];
        traverse(root, &mut values); // Assuming traverse populates values: Vec<Option<i32>>

        // Find the index of the last non-None element.
        // rposition() efficiently searches from the end.
        if let Some(last_some_index) = values.iter().rposition(|x| x.is_some()) {
            // We only need to process elements up to the last Some.
            // Create a slice containing elements from the beginning up to last_some_index.
            let relevant_slice = &values[0..=last_some_index];

            // Iterate over the relevant slice, map to strings, collect *once*, and join.
            relevant_slice
                .iter()
                .map(|x| x.map_or_else(|| "null".to_string(), |n| n.to_string()))
                .collect::<Vec<String>>() // Collect the mapped strings into a Vec
                .join(",") // Join the Vec<String>
        } else {
            // If values was empty or contained only None, return an empty string.
            String::new()
        }
    }

    // i, 2i + 1, 2(i + 1)
    //
    // [1,
    //  2,          3,
    //  null,       null,       4,       null,
    //  null, null, null, null, null, 5, null, null]
    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
        let mut nodes: Vec<Rc<RefCell<TreeNode>>> = vec![];
        for (i, ele) in data.split(',').enumerate() {
            if ele != "null" {
                let node = TreeNode::new(ele.parse::<i32>().unwrap());
                let node = Rc::new(RefCell::new(node));
                if !nodes.is_empty() {
                    let pidx = i / 2;
                    if i % 2 == 1 {
                        nodes[pidx].borrow_mut().left = Some(Rc::clone(&node));
                    } else {
                        nodes[pidx - 1].borrow_mut().right = Some(Rc::clone(&node));
                    }
                }
                nodes.push(Rc::clone(&node));
            }
        }
        if nodes.is_empty() {
            None
        } else {
            Some(Rc::clone(&nodes[0]))
        }
    }
}

use std::collections::VecDeque;

// this is level-order traversal but preorder traversal is the best choice
fn traverse(root: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<Option<i32>>) {
    if root.is_none() {
        return;
    }
    let mut myq = VecDeque::new();
    myq.push_back(root);

    while !myq.is_empty() {
        let level_size = myq.len();

        for _ in 0..level_size {
            if let Some(node) = myq.pop_front() {
                match &node {
                    None => {
                        values.push(None);
                    },
                    Some(x) => {
                        values.push(Some(x.borrow().val));
                        myq.push_back(x.borrow().left.clone());
                        myq.push_back(x.borrow().right.clone());
                    }
                }
            }
        }
    }
}
