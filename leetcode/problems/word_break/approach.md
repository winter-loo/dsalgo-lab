# Approach

The "Word Break" problem asks us to determine if a string can be segmented into a sequence of dictionary words.

## Strategy

This problem can be solved using:
1. **Dynamic Programming**: Build a solution from the bottom up
2. **Recursion with Memoization**: Top-down approach with caching
3. **Breadth-First Search (BFS)**: Treat it as a graph traversal problem

## Dynamic Programming Approach
1. Create a boolean array `dp` where `dp[i]` represents whether the substring `s[0...i-1]` can be segmented into dictionary words
2. Initialize `dp[0] = true` (empty string can always be segmented)
3. For each position `i` from 1 to n:
   - For each position `j` from 0 to i-1:
     - If `dp[j]` is true and the substring `s[j...i-1]` is in the dictionary, then `dp[i] = true`
4. Return `dp[n]`

## BFS Approach
1. Start a BFS from position 0
2. For each position, try all dictionary words and see if they match the substring starting at that position
3. If a match is found, add the ending position to the queue
4. If we can reach position n, return true

## Time and Space Complexity
- **Time Complexity**: O(n²) where n is the length of the string
  - For each position, we might need to check all previous positions
- **Space Complexity**: O(n) for the DP array or the visited set in BFS

## Pseudocode for Dynamic Programming Approach
```
function wordBreak(s, wordDict):
    n = length(s)
    dp = boolean array of size n+1, initialized to false
    dp[0] = true
    
    for i from 1 to n:
        for j from 0 to i-1:
            if dp[j] and s[j...i-1] is in wordDict:
                dp[i] = true
                break
    
    return dp[n]
```

## Pseudocode for BFS Approach
```
function wordBreak(s, wordDict):
    n = length(s)
    queue = new Queue()
    queue.push(0)
    visited = set()
    
    while queue is not empty:
        start = queue.pop()
        
        if start == n:
            return true
        
        if start in visited:
            continue
        
        visited.add(start)
        
        for word in wordDict:
            if s[start...start+length(word)-1] == word:
                queue.push(start + length(word))
    
    return false
```
