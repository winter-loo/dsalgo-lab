pub struct Solution;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        // Build adjacency list with a min-heap for each airport
        // Using Reverse with BinaryHeap to create a min-heap
        let mut graph: HashMap<String, BinaryHeap<Reverse<String>>> = HashMap::new();

        // Add all tickets to the graph
        for ticket in tickets {
            let from = ticket[0].clone();
            let to = ticket[1].clone();
            graph.entry(from).or_default().push(Reverse(to));
            // Ensure destination airports are in the graph even if they have no outgoing flights
            graph.entry(ticket[1].clone()).or_default();
        }

        // Use a VecDeque to build the result in reverse order
        let mut result: VecDeque<String> = VecDeque::new();

        // println!("graph={graph:?}");

        // Start DFS from JFK
        Self::dfs("JFK".to_string(), &mut graph, &mut result);

        // Convert the VecDeque to a Vec
        result.into_iter().collect()
    }

    // DFS function that implements Hierholzer's algorithm
    fn dfs(
        airport: String,
        graph: &mut HashMap<String, BinaryHeap<Reverse<String>>>,
        result: &mut VecDeque<String>,
    ) {
        // println!("airport={airport}, result={result:?}");
        // Process all outgoing edges
        while let Some(Reverse(next)) = graph.get_mut(&airport).and_then(|dests| dests.pop()) {
            Self::dfs(next, graph, result);
        }

        // Add the current airport to the result (in reverse order)
        result.push_front(airport);
        // println!("result={result:?}");
    }
}
