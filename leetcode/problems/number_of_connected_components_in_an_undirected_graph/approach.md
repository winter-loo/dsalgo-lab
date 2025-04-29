# Approach

This problem asks us to find the number of connected components in an undirected graph. A connected component is a subgraph where any two vertices are connected by a path, and which is not connected to any additional vertices.

## Strategy
We can use either Depth-First Search (DFS), Breadth-First Search (BFS), or Union-Find (Disjoint Set) to solve this problem:

1. **DFS/BFS Approach**: Traverse the graph and count the number of times we need to start a new traversal.
2. **Union-Find Approach**: Use a disjoint set data structure to group connected nodes and count the number of distinct groups.

## DFS Approach
1. Build an adjacency list representation of the graph from the edges.
2. Use a visited array to keep track of nodes visited during the DFS traversal.
3. For each unvisited node, perform a DFS to visit all nodes in its connected component and increment a counter.
4. Return the counter as the number of connected components.

## BFS Approach
1. Build an adjacency list representation of the graph from the edges.
2. Use a visited array to keep track of nodes visited during the BFS traversal.
3. For each unvisited node, perform a BFS to visit all nodes in its connected component and increment a counter.
4. Return the counter as the number of connected components.

## Union-Find Approach
1. Initialize a Union-Find data structure with n nodes, where each node is in its own set.
2. For each edge (u, v), union the sets containing u and v.
3. After processing all edges, count the number of distinct sets (roots) as the number of connected components.

## Time and Space Complexity
- **Time Complexity**: 
  - DFS/BFS: O(V + E) where V is the number of nodes and E is the number of edges.
  - Union-Find: O(E * α(n)) where α is the inverse Ackermann function, which grows very slowly and is effectively constant for practical purposes.
- **Space Complexity**: O(V + E) for storing the adjacency list and the visited array or the disjoint set data structure.

## Pseudocode for DFS Approach
```
function countComponents(n, edges):
    // Build adjacency list
    graph = empty adjacency list of size n
    for each [u, v] in edges:
        add v to graph[u]
        add u to graph[v]
    
    // Array to track visited nodes
    visited = array of size n, all false
    
    // Count connected components
    count = 0
    for i from 0 to n-1:
        if not visited[i]:
            dfs(i, graph, visited)
            count++
    
    return count

function dfs(node, graph, visited):
    visited[node] = true
    
    for each neighbor in graph[node]:
        if not visited[neighbor]:
            dfs(neighbor, graph, visited)
```

## Pseudocode for Union-Find Approach
```
function countComponents(n, edges):
    // Initialize Union-Find data structure
    parent = array of size n, where parent[i] = i
    rank = array of size n, all 0
    
    // Process each edge
    for each [u, v] in edges:
        union(u, v, parent, rank)
    
    // Count distinct sets
    count = 0
    for i from 0 to n-1:
        if parent[i] == i:
            count++
    
    return count

function find(x, parent):
    if parent[x] != x:
        parent[x] = find(parent[x], parent)
    return parent[x]

function union(x, y, parent, rank):
    rootX = find(x, parent)
    rootY = find(y, parent)
    
    if rootX == rootY:
        return
    
    if rank[rootX] < rank[rootY]:
        parent[rootX] = rootY
    else if rank[rootX] > rank[rootY]:
        parent[rootY] = rootX
    else:
        parent[rootY] = rootX
        rank[rootX]++
```
