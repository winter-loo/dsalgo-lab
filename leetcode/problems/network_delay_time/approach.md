# Approach

This problem asks us to find the minimum time it takes for a signal to reach all nodes in a network, starting from a given node. This is essentially a shortest path problem in a weighted directed graph.

## Strategy
We can use several algorithms to solve this problem:

1. **Dijkstra's Algorithm**: Find the shortest path from the source node to all other nodes.
2. **Bellman-Ford Algorithm**: Find the shortest path from the source node to all other nodes, even with negative edge weights (though not needed for this problem).
3. **Floyd-Warshall Algorithm**: Find the shortest paths between all pairs of nodes.

Since we only need to find the shortest paths from a single source node, Dijkstra's algorithm is the most efficient choice.

## Dijkstra's Algorithm Approach
1. Build an adjacency list representation of the graph from the given edges.
2. Initialize a distance array with infinity for all nodes except the source node, which is set to 0.
3. Use a priority queue to process nodes in order of their current distance from the source.
4. For each node, update the distances of its neighbors if a shorter path is found.
5. After processing all nodes, find the maximum distance among all nodes. If any node has a distance of infinity, return -1 (impossible to reach all nodes). Otherwise, return the maximum distance.

## Time and Space Complexity
- **Time Complexity**: O(E log V) where E is the number of edges and V is the number of nodes. This is the time complexity of Dijkstra's algorithm using a priority queue.
- **Space Complexity**: O(V + E) for storing the adjacency list and the distance array.

## Pseudocode for Dijkstra's Algorithm
```
function networkDelayTime(times, n, k):
    // Build adjacency list
    graph = empty adjacency list of size n+1
    for each [u, v, w] in times:
        add (v, w) to graph[u]
    
    // Initialize distance array
    dist = array of size n+1, all infinity
    dist[k] = 0
    
    // Priority queue for Dijkstra's algorithm
    pq = empty priority queue
    pq.enqueue((0, k))  // (distance, node)
    
    // Dijkstra's algorithm
    while pq is not empty:
        (d, node) = pq.dequeue()
        
        if d > dist[node]:
            continue
        
        for each (neighbor, weight) in graph[node]:
            if dist[node] + weight < dist[neighbor]:
                dist[neighbor] = dist[node] + weight
                pq.enqueue((dist[neighbor], neighbor))
    
    // Find maximum distance
    maxDist = 0
    for i from 1 to n:
        if dist[i] == infinity:
            return -1
        maxDist = max(maxDist, dist[i])
    
    return maxDist
```

## Bellman-Ford Algorithm Approach
1. Initialize a distance array with infinity for all nodes except the source node, which is set to 0.
2. Relax all edges V-1 times, where V is the number of nodes.
3. For each edge (u, v, w), if dist[u] + w < dist[v], update dist[v] to dist[u] + w.
4. After relaxing all edges, find the maximum distance among all nodes. If any node has a distance of infinity, return -1. Otherwise, return the maximum distance.

## Floyd-Warshall Algorithm Approach
1. Initialize a distance matrix with infinity for all pairs of nodes except (i, i), which is set to 0.
2. For each edge (u, v, w), set dist[u][v] to w.
3. For each intermediate node k, for each pair of nodes (i, j), update dist[i][j] to min(dist[i][j], dist[i][k] + dist[k][j]).
4. After processing all nodes, find the maximum distance from the source node to any other node. If any node has a distance of infinity, return -1. Otherwise, return the maximum distance.
