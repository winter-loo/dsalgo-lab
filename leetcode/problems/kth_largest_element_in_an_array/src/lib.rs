pub struct Solution;

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        assert!((k as usize) <= nums.len());

        let mut heap = BinaryHeap::new();
        for n in nums {
            if heap.len() < k as usize {
                heap.push(Reverse(n));
            } else {
                if let Some(Reverse(top)) = heap.peek() {
                    if n > *top {
                        heap.pop();
                        heap.push(Reverse(n));
                    }
                }
            }
        }
        return heap.peek().unwrap().0;
    }
}
