# Hints for Design Add and Search Words Data Structure

1. This problem is an extension of the basic Trie (Prefix Tree) implementation, with the added complexity of supporting wildcard searches using the dot character ('.').

2. For the `addWord` operation, you can use the same approach as in a standard Trie:
   - Start at the root node
   - For each character, create a new node if it doesn't exist
   - Mark the final node as the end of a word

3. The `search` operation is more complex due to the wildcard character:
   - For regular characters, follow the standard Trie search
   - For the wildcard character ('.'), you need to explore all possible paths

4. Consider using a recursive approach for the `search` operation:
   - If the current character is a regular character, check if it exists in the current node's children
   - If the current character is a wildcard, try all possible children
   - Base case: If you've processed all characters, check if the current node is marked as the end of a word

5. In Rust, be careful with ownership and borrowing in the recursive search function. You may need to use references to avoid ownership issues.

6. The constraints mention that there will be at most 3 dots in the search queries, which means the worst-case scenario (all wildcards) won't be too frequent.

7. Consider optimizing the search by pruning paths early if they can't lead to a match.
