# Approach

This problem is an extension of the basic Trie (Prefix Tree) implementation, with the added complexity of supporting wildcard searches using the dot character ('.').

## Strategy
We'll use a Trie data structure similar to the basic implementation, but with a modified search algorithm to handle the wildcard character.

## Steps for Implementation

### TrieNode Structure
- Create a `TrieNode` struct with:
  - A boolean field `is_end_of_word` to mark the end of a word
  - A collection of children (typically a HashMap) mapping characters to child nodes

### WordDictionary Structure
- Create a `WordDictionary` struct with a root `TrieNode`

### addWord Operation
1. Start at the root node
2. For each character in the word:
   - If the character doesn't exist in the current node's children, create a new node for it
   - Move to the child node corresponding to the character
3. Mark the final node as the end of a word

### search Operation
The search operation is more complex due to the wildcard character:

1. Implement a recursive search function that takes:
   - The current node
   - The remaining part of the word to search
   
2. Base case: If there are no more characters to search:
   - Return whether the current node is marked as the end of a word
   
3. Get the current character:
   - If it's a regular character (not '.'):
     - If the character exists in the current node's children, recursively search with the child node and the rest of the word
     - Otherwise, return false
   - If it's a wildcard character ('.'):
     - Try all possible children of the current node and recursively search with each child and the rest of the word
     - If any recursive call returns true, return true
     - Otherwise, return false

## Implementation Details
- In Rust, we can use a `HashMap<char, TrieNode>` to represent the children of each node
- The recursive search function will need to handle the wildcard character by exploring all possible paths
- We need to be careful with ownership and borrowing in Rust, especially in the recursive search function

## Time and Space Complexity
- **Time Complexity**:
  - addWord: O(m) where m is the length of the word
  - search: 
    - Best case (no wildcards): O(m) where m is the length of the word
    - Worst case (all wildcards): O(n * 26^m) where n is the number of words in the dictionary and m is the length of the search word
- **Space Complexity**: O(n * m) where n is the number of words and m is the average length of the words
