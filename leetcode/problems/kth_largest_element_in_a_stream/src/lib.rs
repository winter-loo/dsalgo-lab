use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct KthLargest {
    k: i32,
    min_heap: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        // TODO: Implement constructor
        KthLargest {
            k,
            min_heap: BinaryHeap::new(),
        }
    }
    
    pub fn add(&mut self, val: i32) -> i32 {
        // TODO: Implement add method
        0
    }
}
