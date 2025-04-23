pub struct Solution;

use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        if tasks.is_empty() {
            return 0;
        }
        let mut mymap = HashMap::new();
        let (mut max_freq, mut most_char) = (0, tasks[0]);
        for t in tasks {
            let count = mymap.entry(t).or_insert(0);
            *count += 1;
            if *count > max_freq {
                (max_freq, most_char) = (*count, t);
            }
        }
        let mut heap = BinaryHeap::new();
        for (task, count) in mymap.iter() {
            heap.push((*count, *task));
        }
        println!("max_freq: {max_freq}, most_char: {most_char}");
        println!("mymap: {mymap:?}");
        println!("heap: {heap:?}");
        mymap.remove(&most_char);
        heap.pop();
        println!("mymap: {mymap:?}");
        println!("heap: {heap:?}");
        let mut answer = max_freq;
        let placeholders = (max_freq - 1) * n;
        0
    }
}
