pub struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut adjlist = vec![vec![]; (n + 1) as usize];
        for time in times {
            let (source, target, weight) = (time[0] as usize, time[1] as usize, time[2] as usize);
            adjlist[source].push((target, weight));
        }
        // println!("adjlist={adjlist:?}");

        let mut distance = vec![usize::MAX; (n + 1) as usize];
        distance[k as usize] = 0;

        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut min_heap = BinaryHeap::new();
        min_heap.push(Reverse((0, k as usize)));

        while let Some(Reverse((dist, node))) = min_heap.pop() {
            for (neighbor, weight) in adjlist[node].iter() {
                if distance[*neighbor] > dist + *weight {
                    distance[*neighbor] = dist + *weight;
                    min_heap.push(Reverse((distance[*neighbor], *neighbor)));
                }
            }
        }
        if distance[1..].iter().any(|v| *v == usize::MAX) {
            -1
        } else {
            *distance[1..].iter().max().unwrap() as i32
        }
    }
}
