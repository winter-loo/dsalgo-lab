# Hints for Word Search II

1. This problem combines the Word Search problem with the Trie data structure to efficiently find multiple words in a grid.

2. Start by building a Trie from the list of words you're looking for. This will allow you to efficiently check if a sequence of characters forms a prefix or a complete word.

3. For each cell in the board, perform a DFS to find all words that start with the character at that cell.

4. During the DFS:
   - Mark the current cell as visited to avoid using it again in the same path.
   - Check if the current path forms a prefix in the Trie. If not, backtrack.
   - If the current path forms a complete word in the Trie, add it to the result.
   - Recursively explore all four adjacent directions (up, down, left, right).
   - Unmark the current cell (backtrack) before returning.

5. To avoid duplicates, remove words from the Trie as they are found.

6. For efficiency, you can prune the Trie by removing nodes that no longer lead to any words.

7. Instead of using a separate visited array, you can temporarily modify the board itself to mark visited cells (e.g., by changing the character to a special character like '#').

8. Remember that the constraints mention the board can be up to 12x12, and there can be up to 3 * 10^4 words, so your solution needs to be efficient.
