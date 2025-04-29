# Approach

This problem asks us to determine the order of letters in an alien language, given a list of words sorted lexicographically according to that language. This is essentially a problem of finding a topological sort of a directed graph.

## Strategy
1. Build a directed graph where:
   - Each node represents a letter in the alien language.
   - An edge from letter 'a' to letter 'b' means 'a' comes before 'b' in the alien language.
2. Perform a topological sort on the graph to determine the order of letters.
3. Check for cycles in the graph, which would indicate an invalid ordering.

## Building the Graph
To build the graph, we compare adjacent words in the sorted list. For each pair of adjacent words, we find the first position where the letters differ. This gives us an ordering constraint: the letter in the first word comes before the letter in the second word in the alien language.

For example, if we have words "wrt" and "wrf", the first position where they differ is at index 2, where 't' in "wrt" and 'f' in "wrf" differ. This gives us the constraint: 't' comes before 'f' in the alien language.

## Topological Sort
Once we have the graph, we can use either Depth-First Search (DFS) or Breadth-First Search (BFS) to perform a topological sort:

### DFS Approach (with cycle detection)
1. Perform a DFS traversal of the graph.
2. Use three states for each node: unvisited, visiting, and visited.
3. If we encounter a node that is in the "visiting" state during DFS, we have detected a cycle, and the ordering is invalid.
4. After visiting all neighbors of a node, mark it as "visited" and add it to the result.
5. Reverse the result to get the correct order.

### BFS Approach (Kahn's Algorithm)
1. Calculate the in-degree of each node (the number of edges pointing to it).
2. Start with nodes that have an in-degree of 0 (no dependencies).
3. Remove these nodes and their outgoing edges from the graph, potentially creating new nodes with an in-degree of 0.
4. Repeat until all nodes are processed or no nodes with an in-degree of 0 remain (indicating a cycle).

## Time and Space Complexity
- **Time Complexity**: O(C) where C is the total length of all words in the input. We need to iterate through all characters in all words to build the graph.
- **Space Complexity**: O(1) or O(U) where U is the number of unique letters in the alien language. Since we're dealing with lowercase English letters, this is bounded by 26, so it's effectively O(1).

## Pseudocode for DFS Approach
```
function alienOrder(words):
    // Build the graph
    graph = empty map from char to set of chars
    for each word in words:
        for each char in word:
            if char not in graph:
                graph[char] = empty set
    
    for i from 0 to words.length - 2:
        word1 = words[i]
        word2 = words[i+1]
        
        // Check for invalid ordering
        if word1.length > word2.length and word1.startsWith(word2):
            return ""
        
        for j from 0 to min(word1.length, word2.length) - 1:
            if word1[j] != word2[j]:
                graph[word1[j]].add(word2[j])
                break
    
    // Topological sort using DFS
    visited = map from char to {0: unvisited, 1: visiting, 2: visited}
    result = empty string
    
    function dfs(node):
        if visited[node] == 1:
            return false  // Cycle detected
        if visited[node] == 2:
            return true  // Already processed
        
        visited[node] = 1  // Mark as visiting
        
        for each neighbor in graph[node]:
            if not dfs(neighbor):
                return false
        
        visited[node] = 2  // Mark as visited
        result = node + result  // Add to result
        
        return true
    
    for each node in graph:
        if visited[node] == 0:
            if not dfs(node):
                return ""  // Cycle detected
    
    return result
```

## Pseudocode for BFS Approach (Kahn's Algorithm)
```
function alienOrder(words):
    // Build the graph
    graph = empty map from char to set of chars
    in_degree = empty map from char to int
    
    for each word in words:
        for each char in word:
            if char not in graph:
                graph[char] = empty set
                in_degree[char] = 0
    
    for i from 0 to words.length - 2:
        word1 = words[i]
        word2 = words[i+1]
        
        // Check for invalid ordering
        if word1.length > word2.length and word1.startsWith(word2):
            return ""
        
        for j from 0 to min(word1.length, word2.length) - 1:
            if word1[j] != word2[j]:
                if word2[j] not in graph[word1[j]]:
                    graph[word1[j]].add(word2[j])
                    in_degree[word2[j]] += 1
                break
    
    // Topological sort using BFS
    queue = empty queue
    for each node in graph:
        if in_degree[node] == 0:
            queue.enqueue(node)
    
    result = empty string
    while queue is not empty:
        node = queue.dequeue()
        result += node
        
        for each neighbor in graph[node]:
            in_degree[neighbor] -= 1
            if in_degree[neighbor] == 0:
                queue.enqueue(neighbor)
    
    // Check if all nodes are included in the result
    if result.length != graph.size:
        return ""  // Cycle detected
    
    return result
```

## Edge Cases and Considerations
- If there are no ordering constraints between certain letters, multiple valid orderings may exist. The problem allows returning any valid ordering.
- If there is a cycle in the graph, no valid ordering exists, and we should return an empty string.
- We need to handle the case where a shorter word is a prefix of a longer word. In a valid lexicographical ordering, the shorter word should come before the longer word. If this is not the case in the input, the ordering is invalid.
