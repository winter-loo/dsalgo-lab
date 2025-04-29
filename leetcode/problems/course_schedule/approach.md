# Approach

This problem is essentially asking us to detect if there is a cycle in a directed graph. If a cycle exists, it means there's a circular dependency between courses, making it impossible to complete all courses.

## Strategy
We can use either Depth-First Search (DFS) or Breadth-First Search (BFS) to solve this problem:

1. **DFS Approach**: We perform a DFS traversal and check for back edges, which indicate cycles in the graph.
2. **BFS Approach (Topological Sort)**: We use Kahn's algorithm for topological sorting to check if all courses can be taken.

## DFS Approach
1. Build an adjacency list representation of the graph from the prerequisites.
2. For each course, perform a DFS to check if there's a cycle.
3. Use a visited array to keep track of nodes visited in the current DFS traversal.
4. Use a recursion stack to detect back edges (cycles).

## BFS Approach (Topological Sort)
1. Build an adjacency list and calculate the in-degree (number of prerequisites) for each course.
2. Enqueue all courses with in-degree 0 (no prerequisites).
3. Process the queue:
   - Dequeue a course and "take" it (reduce the in-degree of all courses that depend on it).
   - Enqueue any new courses that now have in-degree 0.
4. If we can take all courses (queue processes all nodes), return true; otherwise, return false.

## Time and Space Complexity
- **Time Complexity**: O(V + E) where V is the number of courses and E is the number of prerequisites. We need to visit each course and each prerequisite relationship once.
- **Space Complexity**: O(V + E) for storing the adjacency list and the visited/in-degree arrays.

## Pseudocode for DFS Approach
```
function canFinish(numCourses, prerequisites):
    // Build adjacency list
    graph = empty adjacency list of size numCourses
    for each [course, prerequisite] in prerequisites:
        add prerequisite to graph[course]
    
    // Array to track visited nodes in current traversal
    visited = array of size numCourses, all false
    
    // Array to track nodes in current recursion stack
    recursionStack = array of size numCourses, all false
    
    // Check for cycle starting from each unvisited node
    for course from 0 to numCourses-1:
        if not visited[course] and hasCycle(course, graph, visited, recursionStack):
            return false
    
    return true

function hasCycle(course, graph, visited, recursionStack):
    visited[course] = true
    recursionStack[course] = true
    
    for each neighbor in graph[course]:
        if not visited[neighbor]:
            if hasCycle(neighbor, graph, visited, recursionStack):
                return true
        else if recursionStack[neighbor]:
            // Back edge found, cycle exists
            return true
    
    recursionStack[course] = false
    return false
```

## Pseudocode for BFS Approach (Topological Sort)
```
function canFinish(numCourses, prerequisites):
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
    coursesTaken = 0
    while queue is not empty:
        current = queue.dequeue()
        coursesTaken++
        
        for each neighbor in graph[current]:
            inDegree[neighbor]--
            if inDegree[neighbor] == 0:
                queue.enqueue(neighbor)
    
    // If we can take all courses, return true
    return coursesTaken == numCourses
```
