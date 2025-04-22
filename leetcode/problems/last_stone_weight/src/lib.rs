pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut max_heap = BinaryHeap::from(stones);
        while max_heap.len() > 1 {
            let s1 = max_heap.pop().unwrap();
            let s2 = max_heap.pop().unwrap();
            let d = (s1 - s2).abs();
            if d != 0 {
                max_heap.push(d);
            }
        }
        max_heap.peek().map_or(0, |n| *n)
    }
}
