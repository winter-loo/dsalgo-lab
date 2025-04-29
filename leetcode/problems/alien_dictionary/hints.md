# Hints for Alien Dictionary

1. This problem is asking you to determine the order of letters in an alien language, given a list of words sorted lexicographically according to that language.

2. Think of this problem as finding a topological sort of a directed graph, where:
   - Each node represents a letter in the alien language.
   - An edge from letter 'a' to letter 'b' means 'a' comes before 'b' in the alien language.

3. To build the graph, compare adjacent words in the sorted list. For each pair of adjacent words, find the first position where the letters differ. This gives you an ordering constraint: the letter in the first word comes before the letter in the second word.

4. Be careful with the case where a shorter word is a prefix of a longer word. In a valid lexicographical ordering, the shorter word should come before the longer word. If this is not the case in the input, the ordering is invalid.

5. Once you have the graph, you can use either Depth-First Search (DFS) or Breadth-First Search (BFS) to perform a topological sort.

6. For DFS:
   - Use three states for each node: unvisited, visiting, and visited.
   - If you encounter a node that is in the "visiting" state during DFS, you have detected a cycle, and the ordering is invalid.
   - After visiting all neighbors of a node, mark it as "visited" and add it to the result.
   - Reverse the result to get the correct order.

7. For BFS (Kahn's Algorithm):
   - Calculate the in-degree of each node (the number of edges pointing to it).
   - Start with nodes that have an in-degree of 0 (no dependencies).
   - Remove these nodes and their outgoing edges from the graph, potentially creating new nodes with an in-degree of 0.
   - Repeat until all nodes are processed or no nodes with an in-degree of 0 remain (indicating a cycle).

8. If there is a cycle in the graph, no valid ordering exists, and you should return an empty string.

9. If there are no ordering constraints between certain letters, multiple valid orderings may exist. The problem allows returning any valid ordering.

10. Remember that the graph only includes letters that appear in the words. If a letter doesn't appear in any word, it's not part of the alien language's alphabet.
