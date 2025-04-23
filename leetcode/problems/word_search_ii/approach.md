# Approach

This problem combines the Word Search problem with the Trie data structure to efficiently find multiple words in a grid.

## Strategy
The key insight is to use a Trie to store all the words we're looking for, and then perform a DFS on the board to find these words.

## Steps
1. Build a Trie from the list of words.
2. For each cell in the board, perform a DFS to find all words that start with the character at that cell.
3. During the DFS:
   - Mark the current cell as visited to avoid using it again in the same path.
   - Check if the current path forms a prefix in the Trie.
   - If it's not a prefix, backtrack.
   - If it's a complete word in the Trie, add it to the result.
   - Recursively explore all four adjacent directions.
   - Unmark the current cell (backtrack) before returning.
4. Return the list of found words.

## Implementation Details
### Trie Structure
- Create a `TrieNode` struct with:
  - A field to store a complete word (if the node represents the end of a word)
  - A collection of children (typically a HashMap) mapping characters to child nodes

### Building the Trie
- Insert each word from the input list into the Trie.
- For each word, mark the final node with the complete word.

### DFS Search
- For each cell in the board, start a DFS if the character matches a child of the Trie root.
- During the DFS, keep track of the current Trie node and the current path.
- If the current node has a word stored, add it to the result and remove it from the Trie to avoid duplicates.
- Explore all four adjacent directions if they are valid and match a child of the current Trie node.

## Optimizations
- Remove words from the Trie as they are found to avoid duplicates and reduce the search space.
- Prune the Trie by removing nodes that no longer lead to any words.
- Use a character in the board itself to mark visited cells instead of a separate visited array.

## Time and Space Complexity
- **Time Complexity**: O(M * N * 4^L) where M and N are the dimensions of the board, and L is the maximum length of a word. In the worst case, we might need to explore all four directions for each character in the word.
- **Space Complexity**: O(K) where K is the total number of characters in all words, for storing the Trie.
