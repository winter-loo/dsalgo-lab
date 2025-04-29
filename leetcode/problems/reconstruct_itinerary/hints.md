# Hints for Reconstruct Itinerary

1. This problem asks you to find a valid itinerary that uses all tickets exactly once, starting from "JFK". If multiple valid itineraries exist, you should return the one with the smallest lexical order.

2. Think of this problem as finding an Eulerian path in a directed graph, where:
   - Each airport is a node.
   - Each ticket is a directed edge from the departure airport to the arrival airport.

3. Hierholzer's algorithm is an efficient way to find an Eulerian path:
   - Build an adjacency list representation of the graph.
   - For each airport, maintain a list of destinations sorted in lexical order.
   - Perform a DFS starting from "JFK".
   - When you reach an airport with no outgoing edges (or all outgoing edges have been used), add it to the result.
   - Reverse the result to get the correct order of the itinerary.

4. To ensure you get the itinerary with the smallest lexical order, sort the destinations from each airport in lexical order before starting the DFS.

5. In many programming languages, you can use a priority queue or a sorted list to maintain the destinations in lexical order.

6. The problem guarantees that at least one valid itinerary exists, so you don't need to handle the case where no valid itinerary can be formed.

7. Remember that you need to use all tickets exactly once, which means you need to find an Eulerian path, not just any path.

8. If you're using a recursive DFS approach, be careful with the order in which you add airports to the result. You should add an airport to the result only after you've explored all its outgoing edges.

9. The time complexity of this approach is O(E log E) where E is the number of edges (tickets), due to the sorting of destinations for each airport.
