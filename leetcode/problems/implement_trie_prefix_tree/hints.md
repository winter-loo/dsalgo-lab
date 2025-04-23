# Hints for Implement Trie (Prefix Tree)

1. A Trie is a tree-like data structure where each node represents a character in a word. The root node is typically empty.

2. Each node in the Trie should contain:
   - A flag to indicate if the node represents the end of a word
   - A collection of child nodes, each corresponding to a possible next character

3. In Rust, you can represent the children of a node using a `HashMap<char, TrieNode>` or a fixed-size array if you know the character set is limited (e.g., lowercase English letters).

4. For the `insert` operation:
   - Start at the root node
   - For each character in the word, create a new node if it doesn't exist
   - Mark the final node as the end of a word

5. For the `search` operation:
   - Start at the root node
   - For each character in the word, check if it exists in the current node's children
   - Return true only if the final node is marked as the end of a word

6. For the `startsWith` operation:
   - Similar to search, but you don't need to check if the final node is the end of a word
   - Return true if you can traverse all characters in the prefix

7. Remember that in Rust, you'll need to handle ownership and borrowing carefully, especially when traversing the Trie.
