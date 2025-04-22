use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct KthLargest {
    k: usize,
    min_heap: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut this = KthLargest {
            k: k as usize,
            min_heap: BinaryHeap::new(),
        };

        for n in nums {
            this.add(n);
        }
        this
    }
    
    pub fn add(&mut self, val: i32) -> i32 {
        if self.min_heap.len() < self.k {
            self.min_heap.push(Reverse(val));
            if let Some(Reverse(top)) = self.min_heap.peek() {
                return *top;
            }
        } else {
            if let Some(Reverse(top)) = self.min_heap.peek() {
                if val > *top {
                    self.min_heap.pop();
                    self.min_heap.push(Reverse(val));
                    return self.min_heap.peek().unwrap().0;
                } else {
                    return *top;
                }
            }
        }
        unreachable!()
    }
}
