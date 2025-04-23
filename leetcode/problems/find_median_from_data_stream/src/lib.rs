use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        MedianFinder {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        match self.max_heap.len().cmp(&self.min_heap.len()) {
            Ordering::Greater => {
                let n = self.max_heap.peek().unwrap();
                if *n < num {
                    self.min_heap.push(Reverse(num));
                } else {
                    let bigger = self.max_heap.pop().unwrap();
                    self.min_heap.push(Reverse(bigger));
                    self.max_heap.push(num);
                }
            }
            Ordering::Less => {
                let Reverse(n) = self.min_heap.peek().unwrap();
                if *n > num {
                    self.max_heap.push(num);
                } else {
                    let Reverse(smaller) = self.min_heap.pop().unwrap();
                    self.min_heap.push(Reverse(num));
                    self.max_heap.push(smaller);
                }
            }
            Ordering::Equal => {
                if self.min_heap.peek().is_some_and(|Reverse(x)| *x < num) {
                    self.min_heap.push(Reverse(num));
                } else {
                    self.max_heap.push(num);
                }
            }
        }
    }

    pub fn find_median(&self) -> f64 {
        let count = self.max_heap.len() + self.min_heap.len();
        assert!(count > 0, "nums should not be empty");
        match self.max_heap.len().cmp(&self.min_heap.len()) {
            Ordering::Equal => {
                let n1 = self.max_heap.peek().unwrap();
                let Reverse(n2) = self.min_heap.peek().unwrap();
                (n1 + n2) as f64 / 2.0
            }
            Ordering::Greater => *self.max_heap.peek().unwrap() as f64,
            Ordering::Less => self.min_heap.peek().unwrap().0 as f64,
        }
    }
}
