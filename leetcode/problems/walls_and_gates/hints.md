# Hints for Walls and Gates

1. This problem asks you to fill each empty room with the distance to its nearest gate. If it is impossible to reach a gate, it should remain filled with INF.

2. Consider using Breadth-First Search (BFS) starting from all gates simultaneously. This allows you to find the shortest distance from each empty room to its nearest gate.

3. BFS is particularly useful for finding the shortest path in an unweighted graph, which is what we have here (each step has a distance of 1).

4. Start by adding all gates to a queue as starting points for the BFS.

5. For each gate, explore its four adjacent cells (up, down, left, right). If an adjacent cell is an empty room (INF), update its value to the distance from the current cell plus 1, and add it to the queue.

6. Continue until the queue is empty, which means all reachable empty rooms have been assigned their shortest distance to a gate.

7. Be careful with the grid boundaries and only explore valid cells.

8. You don't need to revisit cells that have already been visited (cells that are no longer INF).

9. Remember that the value 2^31 - 1 (2147483647) represents INF in this problem.
