# Approach

This problem extends the Course Schedule problem by asking us to return a valid ordering of courses (a topological sort) rather than just determining if such an ordering exists.

## Strategy
We can use either Depth-First Search (DFS) or Breadth-First Search (BFS) to solve this problem:

1. **DFS Approach**: We perform a DFS traversal and build the topological sort in reverse order.
2. **BFS Approach (Kahn's Algorithm)**: We use Kahn's algorithm to build the topological sort directly.

## DFS Approach
1. Build an adjacency list representation of the graph from the prerequisites.
2. Use three states for each node to detect cycles:
   - Unvisited (0)
   - Visiting (1) - Node is in the current DFS path
   - Visited (2) - Node and all its descendants have been processed
3. Perform DFS on each unvisited node.
4. Add nodes to the result list in post-order (after all descendants are processed).
5. Reverse the result list to get the correct order.
6. If a cycle is detected, return an empty array.

## BFS Approach (Kahn's Algorithm)
1. Build an adjacency list and calculate the in-degree (number of prerequisites) for each course.
2. Enqueue all courses with in-degree 0 (no prerequisites).
3. Process the queue:
   - Dequeue a course and add it to the result list.
   - Reduce the in-degree of all courses that depend on it.
   - Enqueue any new courses that now have in-degree 0.
4. If the result list contains all courses, return it; otherwise, return an empty array.

## Time and Space Complexity
- **Time Complexity**: O(V + E) where V is the number of courses and E is the number of prerequisites. We need to visit each course and each prerequisite relationship once.
- **Space Complexity**: O(V + E) for storing the adjacency list, the visited array, and the result list.

## Pseudocode for DFS Approach
```
function findOrder(numCourses, prerequisites):
    // Build adjacency list
    graph = empty adjacency list of size numCourses
    for each [course, prerequisite] in prerequisites:
        add course to graph[prerequisite]
    
    // Array to track visited status
    // 0: unvisited, 1: visiting, 2: visited
    visited = array of size numCourses, all 0
    
    // Result list
    result = empty list
    
    // Check for cycle and build topological sort
    for course from 0 to numCourses-1:
        if visited[course] == 0:
            if hasCycle(course, graph, visited, result):
                return empty array
    
    // Reverse result to get correct order
    return result.reverse()

function hasCycle(course, graph, visited, result):
    visited[course] = 1  // Mark as visiting
    
    for each neighbor in graph[course]:
        if visited[neighbor] == 1:
            // Cycle detected
            return true
        if visited[neighbor] == 0:
            if hasCycle(neighbor, graph, visited, result):
                return true
    
    visited[course] = 2  // Mark as visited
    result.push(course)  // Add to result in post-order
    return false
```

## Pseudocode for BFS Approach (Kahn's Algorithm)
```
function findOrder(numCourses, prerequisites):
    // Build adjacency list and calculate in-degrees
    graph = empty adjacency list of size numCourses
    inDegree = array of size numCourses, all 0
    
    for each [course, prerequisite] in prerequisites:
        add course to graph[prerequisite]
        inDegree[course]++
    
    // Enqueue all courses with no prerequisites
    queue = empty queue
    for course from 0 to numCourses-1:
        if inDegree[course] == 0:
            queue.enqueue(course)
    
    // Process the queue (topological sort)
    result = empty list
    while queue is not empty:
        current = queue.dequeue()
        result.push(current)
        
        for each neighbor in graph[current]:
            inDegree[neighbor]--
            if inDegree[neighbor] == 0:
                queue.enqueue(neighbor)
    
    // If we can take all courses, return the order
    if result.length == numCourses:
        return result
    else:
        return empty array
```
