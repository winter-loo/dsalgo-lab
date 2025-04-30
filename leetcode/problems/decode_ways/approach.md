# Approach

The "Decode Ways" problem asks us to count the number of ways to decode a string of digits into letters according to a specific mapping.

## Strategy

This problem can be solved using:
1. **Dynamic Programming**: Build a solution from the bottom up
2. **Recursion with Memoization**: Top-down approach with caching

## Dynamic Programming Approach
1. Create an array `dp` where `dp[i]` represents the number of ways to decode the substring `s[0...i-1]`
2. Initialize base cases:
   - `dp[0] = 1` (empty string has one way to decode - as nothing)
   - `dp[1] = 1` if `s[0] != '0'`, otherwise `dp[1] = 0` (a single digit can be decoded if it's not zero)
3. For each position `i` from 2 to n:
   - If the current digit `s[i-1]` is not '0', then `dp[i] += dp[i-1]` (we can decode it as a single digit)
   - If the last two digits `s[i-2:i]` form a valid letter (10-26), then `dp[i] += dp[i-2]` (we can decode it as a two-digit number)
4. Return `dp[n]`

## Key Considerations
1. Handle leading zeros carefully - a digit starting with 0 cannot be decoded
2. Only digits between 1-26 can be mapped to letters
3. The problem asks for the number of ways to decode, not the actual decodings

## Time and Space Complexity
- **Time Complexity**: O(n) where n is the length of the string
- **Space Complexity**: O(n) for the standard implementation, but can be optimized to O(1) by only keeping track of the last two values

## Pseudocode for Dynamic Programming Approach
```
function numDecodings(s):
    n = length(s)
    if n == 0 or s[0] == '0':
        return 0
    
    dp = array of size n+1, initialized to 0
    dp[0] = 1
    dp[1] = 1
    
    for i from 2 to n:
        // Check if current digit can be decoded (not '0')
        if s[i-1] != '0':
            dp[i] += dp[i-1]
        
        // Check if last two digits form a valid letter (10-26)
        twoDigit = int(s[i-2:i])
        if twoDigit >= 10 and twoDigit <= 26:
            dp[i] += dp[i-2]
    
    return dp[n]
```

## Pseudocode for Optimized Space Approach
```
function numDecodings(s):
    n = length(s)
    if n == 0 or s[0] == '0':
        return 0
    
    prev2 = 1  // dp[0]
    prev1 = 1  // dp[1]
    
    for i from 2 to n:
        current = 0
        
        // Check if current digit can be decoded (not '0')
        if s[i-1] != '0':
            current += prev1
        
        // Check if last two digits form a valid letter (10-26)
        twoDigit = int(s[i-2:i])
        if twoDigit >= 10 and twoDigit <= 26:
            current += prev2
        
        prev2 = prev1
        prev1 = current
    
    return prev1
```
