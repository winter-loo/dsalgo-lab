pub struct Solution;

impl Solution {
    // union-find algorithm
    // https://www.geeksforgeeks.org/introduction-to-disjoint-set-data-structure-or-union-find-algorithm/
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        fn find_root(roots: &mut [i32], n: i32) -> i32 {
            if roots[n as usize] != n {
                roots[n as usize] = find_root(roots, roots[n as usize]);
            }
            roots[n as usize]
        }

        let mut roots: Vec<_> = (0..edges.len() as i32 + 1).collect();
        for edge in edges {
            let u = find_root(&mut roots, edge[0]);
            let v = find_root(&mut roots, edge[1]);

            if u == v {
                return edge;
            }
            roots[u as usize] = v;
        }
        unreachable!()
    }

    pub fn find_redundant_connection_by_dfs(edges: Vec<Vec<i32>>) -> Vec<i32> {
        // Find the number of nodes in the graph
        let n = edges.len();

        // Build an adjacency list representation of the graph
        let graph: Vec<Vec<usize>> = vec![vec![]; n + 1];

        // Process edges in reverse order to find the last edge that creates a cycle
        for i in (0..n).rev() {
            let u = edges[i][0] as usize;
            let v = edges[i][1] as usize;

            // Temporarily remove this edge from the graph
            let mut temp_graph = graph.clone();

            // Add all edges except the current one
            for j in 0..n {
                if j != i {
                    let edge_u = edges[j][0] as usize;
                    let edge_v = edges[j][1] as usize;
                    temp_graph[edge_u].push(edge_v);
                    temp_graph[edge_v].push(edge_u); // Undirected graph
                }
            }

            // Check if u and v are still connected without this edge
            if !Self::is_connected(&temp_graph, u, v, n) {
                // If not connected, this edge is a bridge and not part of the cycle
                continue;
            } else {
                // If still connected, this edge is part of the cycle and can be removed
                return edges[i].clone();
            }
        }

        // This should not happen if the input is valid
        vec![]
    }

    // DFS to check if two nodes are connected
    fn is_connected(graph: &Vec<Vec<usize>>, start: usize, end: usize, n: usize) -> bool {
        let mut visited = vec![false; n + 1];
        Self::dfs(graph, start, end, &mut visited)
    }

    fn dfs(
        graph: &Vec<Vec<usize>>,
        current: usize,
        target: usize,
        visited: &mut Vec<bool>,
    ) -> bool {
        // If we reached the target, return true
        if current == target {
            return true;
        }

        // Mark current node as visited
        visited[current] = true;

        // Check all neighbors
        for &neighbor in &graph[current] {
            if !visited[neighbor] {
                if Self::dfs(graph, neighbor, target, visited) {
                    return true;
                }
            }
        }

        false
    }
}
