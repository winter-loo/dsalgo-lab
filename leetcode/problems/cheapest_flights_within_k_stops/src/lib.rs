pub struct Solution;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        use std::collections::VecDeque;
        const COST_INF: i32 = (1e4 as i32) + 1;

        let (n, src, dst, k) = (n as usize, src as usize, dst as usize, k as usize);
        let cost_inf = COST_INF * (k as i32 + 1);

        let mut graph = vec![vec![]; n];
        for f in flights {
            let (src, dst, cost) = (f[0] as usize, f[1] as usize, f[2]);
            graph[src].push((dst, cost));
        }
        let graph = graph;

        let mut costs = vec![cost_inf; n];

        let mut q = VecDeque::new();
        q.push_back((src, 0, 0));

        let mut ans = cost_inf;

        while let Some((node, stops, cost)) = q.pop_front() {
            println!("node={node}, stops={stops}, cost={cost}");

            if node == dst {
                ans = ans.min(cost);
                continue;
            }

            if stops == k + 1 {
                continue;
            }

            for &(d, c) in &graph[node] {
                if cost + c < costs[d] {
                    costs[d] = cost + c;
                    q.push_back((d, stops + 1, cost + c));
                }
            }
        }

        return if ans == cost_inf { -1 } else { ans };
    }

    pub fn find_cheapest_price_too_slow(
        _n: i32,
        flights: Vec<Vec<i32>>,
        src: i32,
        dst: i32,
        k: i32,
    ) -> i32 {
        // Build the adjacency list representation of the graph
        let mut graph: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        for flight in flights {
            let from = flight[0];
            let to = flight[1];
            let price = flight[2];
            graph.entry(from).or_default().push((to, price));
        }

        // Initialize a min-heap for Dijkstra's algorithm
        // Each element is (cost, node, stops) where stops is the number of stops made so far
        let mut min_heap = BinaryHeap::new();
        min_heap.push(Reverse((0, src, -1))); // -1 because we're counting edges, not nodes

        // Keep track of the minimum cost to reach each node with a specific number of stops
        let mut best_costs = HashMap::new();

        while let Some(Reverse((cost, node, stops))) = min_heap.pop() {
            // If we've reached the destination, return the cost
            if node == dst {
                return cost;
            }

            // If we've used too many stops, skip this path
            if stops >= k {
                continue;
            }

            // Check if we have a better path to this node with the same or fewer stops
            let key = (node, stops);
            if let Some(&best_cost) = best_costs.get(&key) {
                if best_cost <= cost {
                    continue;
                }
            }

            // Update the best cost to reach this node with this many stops
            best_costs.insert(key, cost);

            // Explore neighbors
            if let Some(neighbors) = graph.get(&node) {
                for &(next_node, price) in neighbors {
                    let next_cost = cost + price;
                    min_heap.push(Reverse((next_cost, next_node, stops + 1)));
                }
            }
        }

        // If we can't reach the destination within k stops, return -1
        -1
    }
}
