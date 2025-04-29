# Approach

This problem asks us to reconstruct an itinerary from a list of airline tickets, starting from "JFK" and using all tickets exactly once. If multiple valid itineraries exist, we should return the one with the smallest lexical order.

## Strategy
This problem can be modeled as finding an Eulerian path in a directed graph, where:
- Each airport is a node.
- Each ticket is a directed edge from the departure airport to the arrival airport.
- We need to find a path that uses each edge exactly once, starting from "JFK".

We can use a depth-first search (DFS) with backtracking to find the Eulerian path. To ensure we get the itinerary with the smallest lexical order, we'll sort the destinations from each airport.

## Hierholzer's Algorithm
Hierholzer's algorithm is an efficient way to find an Eulerian path:
1. Build an adjacency list representation of the graph, where for each airport, we maintain a list of destinations sorted in lexical order.
2. Perform a DFS starting from "JFK".
3. When we reach an airport with no outgoing edges (or all outgoing edges have been used), add it to the result.
4. Reverse the result to get the correct order of the itinerary.

## Time and Space Complexity
- **Time Complexity**: O(E log E) where E is the number of edges (tickets). The sorting of destinations for each airport takes O(E log E) time, and the DFS traversal takes O(E) time.
- **Space Complexity**: O(E) for storing the adjacency list and the result.

## Pseudocode for Hierholzer's Algorithm
```
function findItinerary(tickets):
    // Build adjacency list
    graph = empty map from string to priority queue of strings
    for each [from, to] in tickets:
        add to to graph[from]
    
    // Sort destinations for each airport
    for each airport in graph:
        sort graph[airport] in lexical order
    
    // DFS
    result = empty list
    dfs("JFK", graph, result)
    
    // Reverse the result to get the correct order
    return result.reverse()

function dfs(airport, graph, result):
    while graph[airport] is not empty:
        next = graph[airport].poll()
        dfs(next, graph, result)
    
    result.add(airport)
```

## Alternative Approach: Backtracking
We can also use a backtracking approach:
1. Build an adjacency list representation of the graph.
2. Sort the destinations for each airport in lexical order.
3. Use backtracking to try all possible paths starting from "JFK".
4. Return the first valid path found (which will be the one with the smallest lexical order due to the sorting).

However, this approach is less efficient than Hierholzer's algorithm, especially for graphs with many valid Eulerian paths.

## Edge Cases and Considerations
- The problem guarantees that at least one valid itinerary exists, so we don't need to handle the case where no valid itinerary can be formed.
- We need to use all tickets exactly once, which means we need to find an Eulerian path, not just any path.
- The starting airport is always "JFK".
- If there are multiple valid itineraries, we need to return the one with the smallest lexical order.
