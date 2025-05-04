use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        // Build the graph
        let mut graph: HashMap<char, HashSet<char>> = HashMap::new();
        
        // Add all characters to the graph
        for word in &words {
            for c in word.chars() {
                graph.entry(c).or_insert_with(HashSet::new);
            }
        }
        
        // Add edges to the graph by comparing adjacent words
        for i in 0..words.len() - 1 {
            let word1 = &words[i];
            let word2 = &words[i + 1];
            
            // Check for invalid ordering (shorter word should come before longer word if it's a prefix)
            let min_len = word1.len().min(word2.len());
            let mut found_diff = false;
            
            for j in 0..min_len {
                let c1 = word1.chars().nth(j).unwrap();
                let c2 = word2.chars().nth(j).unwrap();
                
                if c1 != c2 {
                    graph.get_mut(&c1).unwrap().insert(c2);
                    found_diff = true;
                    break;
                }
            }
            
            // If word1 is longer than word2 and word1 starts with word2, it's invalid
            if !found_diff && word1.len() > word2.len() {
                return String::new();
            }
        }
        
        // Topological sort using DFS
        let mut result = String::new();
        let mut visited: HashMap<char, i32> = HashMap::new(); // 0: unvisited, 1: visiting, 2: visited
        
        // Initialize all nodes as unvisited
        for &c in graph.keys() {
            visited.insert(c, 0);
        }
        
        // DFS function
        fn dfs(
            node: char, 
            graph: &HashMap<char, HashSet<char>>, 
            visited: &mut HashMap<char, i32>, 
            result: &mut String
        ) -> bool {
            match visited.get(&node).unwrap() {
                1 => return false, // Cycle detected
                2 => return true,  // Already processed
                _ => {}
            }
            
            visited.insert(node, 1); // Mark as visiting
            
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    if !dfs(neighbor, graph, visited, result) {
                        return false;
                    }
                }
            }
            
            visited.insert(node, 2); // Mark as visited
            result.push(node);       // Add to result
            
            true
        }
        
        // Perform DFS for each unvisited node
        for &node in graph.keys() {
            if *visited.get(&node).unwrap() == 0 {
                if !dfs(node, &graph, &mut visited, &mut result) {
                    return String::new(); // Cycle detected
                }
            }
        }
        
        // Reverse the result to get the correct order
        result.chars().rev().collect()
    }
}
