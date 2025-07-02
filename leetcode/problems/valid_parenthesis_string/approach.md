# Approach

The "Valid Parenthesis String" problem asks us to determine if a string
containing `(`, `)`, and `*` is valid, where `*` can be treated as `(`,
`)`, or an empty string.

## Understanding the Problem

A string is valid if all opening parentheses have matching closing
parentheses in the correct order. The asterisk `*` adds complexity because
it can be treated as either an opening parenthesis, a closing parenthesis,
or nothing.

## Greedy Approach with Min-Max Balance

We can solve this problem by keeping track of the minimum and maximum
possible number of open parentheses as we scan through the string:

1. The minimum count represents the scenario where all `*` are treated as `)` or empty strings.
2. The maximum count represents the scenario where all `*` are treated as `(` or empty strings.

If at any point the maximum count becomes negative, it means we have more
closing parentheses than opening ones, making the string invalid. If
the minimum count goes below zero, we reset it to zero (treating some
`*` as `(` to balance).

At the end, the string is valid if the minimum count is zero (all opening
parentheses have matching closing ones).

## Time and Space Complexity

- **Time Complexity**: O(n), where n is the length of the string. We scan through the string once.
- **Space Complexity**: O(1), as we only need a few variables to keep track of the counts.

## Pseudocode

```
function checkValidString(s):
    minOpen = 0  // Minimum possible open parentheses
    maxOpen = 0  // Maximum possible open parentheses
    
    for each character c in s:
        if c == `(`:
            minOpen++
            maxOpen++
        else if c == `)`:
            minOpen--
            maxOpen--
        else:  // c == `*`
            minOpen--  // Treat `*` as `)`
            maxOpen++  // Treat `*` as `(`
        
        // If we have more closing than opening parentheses
        if maxOpen < 0:
            return false
        
        // Reset minOpen if it goes negative
        minOpen = max(minOpen, 0)
    
    // Valid if we can have zero open parentheses at the end
    return minOpen == 0
```

## Alternative Approach: Dynamic Programming

Another approach is to use dynamic programming to keep track of all
possible valid states at each position. However, the greedy approach is
more efficient and easier to understand.

## Alternative Approach: Two-Pass Scan

We can also solve this by scanning the string twice:
1. Left to right: Count `(` as +1, `)` as -1, `*` as 0. If the count ever goes negative, try using `*` as `(`.
2. Right to left: Count `)` as +1, `(` as -1, `*` as 0. If the count ever goes negative, try using `*` as `)`.

If both passes succeed, the string is valid.
